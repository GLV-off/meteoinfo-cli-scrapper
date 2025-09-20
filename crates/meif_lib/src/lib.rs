use log::{info, error};
use scraper::{Html, Selector, html::Select};

pub mod consts;

pub fn setup_logging() {
    const LOG_CFG: &str = "log.yaml";
    log4rs::init_file(LOG_CFG, Default::default()).unwrap();
}

pub fn get_host() -> String {
    match get_meteoinfo_host() {
        Ok(host) => host,
        Err(error) => {
            info!("Failed to read from environment host: {}", error);
            info!("Using default host: {}", consts::GLV_METEOINFO_HOST_DEFAULT);
            consts::GLV_METEOINFO_HOST_DEFAULT.to_string()
        }
    }
}

pub fn get_images_url(host: &String) -> String {
    format!("{}/satellite-images", host)
}

pub fn get_meteoinfo_host() -> Result<String, std::env::VarError> {
    std::env::var(consts::GLV_METEOINFO_HOST)
}

pub fn get_content(url: String, filename: &str) -> Result<String, ureq::Error> {
    Ok(if std::fs::exists(filename)? {
        info!("parsing filename");
        get_content_file(filename, |e| error!("Error reading file: {}", e))
    } else {
        info!("parsing web content");
        get_content_web(url)?
    })
}

pub fn get_content_file<C: FnOnce(std::io::Error)>(
    path: &str,
    on_error: C,
) -> String {
    match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => {
            on_error(error);
            "error happened".to_string()
        }
    }
}

pub fn get_content_web(url: String) -> Result<String, ureq::Error> {
    let mut resp = ureq::get(url).call()?;

    let cont = resp
        .body_mut()
        .read_to_string()?;

    Ok(cont)
}

pub fn parse_images(content: String) -> Vec<String> {
    let document = Html::parse_document(&content);
    let sel = Selector::parse("a[href^='/images/media/satel/res']")
        .expect("Selector parsing failed!");

    let images_paths = get_images_paths(document.select(&sel));
    images_paths
}

pub fn process_images_gif(host: &str, images: Vec<String>) -> Result<(), ureq::Error> {
    for (index, image_path) in images
        .iter()
        .enumerate()
    {
        info!("{}: {}", index, image_path);

        let image_content = get_content_bytes(String::from(host) + image_path)?;

        std::fs::write(format!("img_{}.gif", index), image_content)?;
    }
    Ok(())
}

pub fn get_synaptic_url(host: &String) -> String {
    format!("{}/mapsynop", host)
}

pub fn parse_synaptic_from_content(content: String) -> Vec<String> {
    let document = Html::parse_document(&content);
    let sel = Selector::parse("meta[property='og:image']")
        .expect("Selector parsing failed!");       
    let ss = document.select(&sel);
    let local: Vec<String> = ss.map(|x| {
            x.value()
                .attr("content")
                .unwrap_or("")
                .to_string()
        })
        .collect();
    local
}

pub fn process_images_png(images: Vec<String>) -> Result<(), ureq::Error> {
    for (index, image_path) in images
        .iter()
        .enumerate()
    {
        info!("{}: {}", index, image_path);

        let image_content = get_content_bytes(image_path.to_string())?;

        std::fs::write(format!("syn_img_{}.png", index), image_content)?;
    }
    Ok(())    
}

pub fn get_content_bytes(url: String) -> Result<Vec<u8>, ureq::Error> {
    let mut resp = ureq::get(url).call()?;

    let cont = resp
        .body_mut()
        .with_config()
        .read_to_vec()?;

    Ok(cont)
}

pub fn get_images_paths(sel: Select) -> Vec<String> {
    sel.map(|x| {
        x.value()
            .attr("href")
            .unwrap_or("")
            .to_string()
    })
    .collect()
}

