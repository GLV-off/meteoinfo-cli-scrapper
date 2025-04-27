use scraper::{Html, Selector, html::Select};

use simplelog::{
    LevelFilter,
    CombinedLogger,
    TermLogger,
    WriteLogger,
    Config,
    TerminalMode,
    ColorChoice
};

use log::info;

fn get_content_file<C>(path: &str, on_error: C) -> String
where
    C: FnOnce(std::io::Error),
{
    match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => {
            on_error(error);
            "error happened".to_string()
        }
    }
}

fn get_content_bytes(url: String) -> Result<Vec<u8>, ureq::Error> {
    let mut resp = ureq::get(url).call()?;

    let cont = resp
        .body_mut()
        .with_config()
        .read_to_vec()?;

    Ok(cont)
}

fn get_content(url: String) -> Result<String, ureq::Error> {
    let mut resp = ureq::get(url).call()?;

    let cont = resp
        .body_mut()
        .read_to_string()?;

    Ok(cont)
}

fn get_images_paths(sel: Select) -> Vec<String> {
    sel.map(|x| {
        x.value()
            .attr("href")
            .unwrap_or("")
            .to_string()
    })
    .collect()
}

#[tokio::main]
async fn main() -> Result<(), ureq::Error> {
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, 
                Config::default(), 
                TerminalMode::Mixed, 
                ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, 
                Config::default(), 
                std::fs::File::create("my_rust_binary.log").unwrap()),
        ]
    ).unwrap();

    let host = "https://meteoinfo.ru";
    let url_images = String::from(host) + "/satellite-images";
    const FILENAME: &str = "content.html";

    let content = if std::fs::exists(FILENAME)? {
        info!("parsing filename");
        get_content_file(FILENAME, |e| println!("Error reading file: {}", e))
    } else {
        info!("parsing web content");
        get_content(url_images)?
    };

    let document = Html::parse_document(&content);
    let sel = Selector::parse("a[href^='/images/media/satel/res']")
        .expect("Selector parsing failed!");
    let images_paths = get_images_paths(document.select(&sel));

    for (index, image_path) in images_paths
        .iter()
        .enumerate()
    {
        info!("{}: {}", index, image_path);       

        let image_content = get_content_bytes(String::from(host) + image_path)?;

        std::fs::write(format!("img_{}.gif", index), image_content)?;
    }

    Ok(())
}
