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
use log::{info, error};

const GLV_METEOINFO_HOST: &str = "";
const FILENAME: &str = "content.html";
const DEFAULT_LOG_FILENAME: &str = "meteo_log.log";

#[tokio::main]
async fn main() -> Result<(), ureq::Error> {
    setup_logging();

    let host = get_host();
    let url_images = get_images_url(&host);
    let content = get_content(url_images)?;
    let images_paths = parse_images_from_content(content);
    process_images(host, images_paths)?;

    Ok(())
}

fn setup_logging() {
        // Настройка логирования по всему продукту
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Info, 
                Config::default(), 
                TerminalMode::Mixed, 
                ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Info, 
                Config::default(), 
                std::fs::File::create(DEFAULT_LOG_FILENAME).unwrap()),
        ]
    ).unwrap();
}

fn get_host() -> String {
    let host = match get_meteoinfo_host() {
        Ok(host) => host,
        Err(error) => { 
            let def_host = "https://meteoinfo.ru";
            info!("Failed to read from environment host: {}", error);
            info!("Using default host: {}", def_host);
            def_host.to_string() 
        }
    };
    host
}

fn get_images_url(host:& String) -> String {
    format!("{}/satellite-images", host)
}

fn get_meteoinfo_host() -> Result<String, std::env::VarError> {
    std::env::var(GLV_METEOINFO_HOST)
}

fn get_content(url: String) -> Result<String, ureq::Error> {
    Ok(if std::fs::exists(FILENAME)? {
        info!("parsing filename");
        get_content_file(FILENAME, |e| error!("Error reading file: {}", e))
    } else {
        info!("parsing web content");
        get_content_web(url)?
    })
}

fn get_content_file<C: FnOnce(std::io::Error)>(path: &str, on_error: C) -> String {
    match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => {
            on_error(error);
            "error happened".to_string()
        }
    }
}

fn get_content_web(url: String) -> Result<String, ureq::Error> {
    let mut resp = ureq::get(url).call()?;

    let cont = resp
        .body_mut()
        .read_to_string()?;

    Ok(cont)
}

fn parse_images_from_content(content: String) -> Vec<String> {
    let document = Html::parse_document(&content);
    let sel = Selector::parse("a[href^='/images/media/satel/res']")
        .expect("Selector parsing failed!");

    let images_paths = get_images_paths(document.select(&sel));
    images_paths
}

fn process_images(host: String, images: Vec<String>) -> Result<(), ureq::Error> {
    for (index, image_path) in images
        .iter()
        .enumerate()
    {
        info!("{}: {}", index, image_path);       

        let image_content = get_content_bytes(host.clone() + image_path)?;

        std::fs::write(format!("img_{}.gif", index), image_content)?;
    }
    Ok(())
}

fn get_content_bytes(url: String) -> Result<Vec<u8>, ureq::Error> {
    let mut resp = ureq::get(url).call()?;

    let cont = resp
        .body_mut()
        .with_config()
        .read_to_vec()?;

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

