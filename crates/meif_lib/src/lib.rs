use log::{info, error};
use scraper::{Html, Selector, html::Select};
use chrono::{DateTime, Utc};

pub mod consts;

pub fn setup_logging() {
    const LOG_CFG: &str = "log.yaml";
    log4rs::init_file(LOG_CFG, Default::default()).unwrap();
}

/**
 * Получение хоста через переменную окружения с обработкой
*/
pub fn get_host() -> String {
    get_meteoinfo_host()
        .unwrap_or_else(|error| {
            info!("Failed to read from environment host: {}", error);
            info!("Using default host: {}", consts::GLV_METEOINFO_HOST_DEFAULT);
            consts::GLV_METEOINFO_HOST_DEFAULT.to_string()
        })
}

/**
 * На основании хоста, сконструировать относительный путь до 
 * изображений.
 */ 
pub fn get_images_url(host: &String) -> String {
    format!("{}/satellite-images", host)
}

/**
 * Получение хоста через переменную окружения без обработки.
 * То через какую переменную мы работаем - выясняется через
 * константу GLV_METEOINFO_HOST
*/
pub fn get_meteoinfo_host() -> Result<String, std::env::VarError> {
    std::env::var(consts::GLV_METEOINFO_HOST)
}

/**
 * Обобщаяющая узловая функция получения HTML контента для обработки.
 * Получить контент по URL если файла по filename не существует. 
 * Если файл по filename существует то попробуем сначала распрасить его
*/
pub fn get_content(url: String, filename: &str) -> Result<String, ureq::Error> {
    Ok(if std::fs::exists(filename)? {
        info!("parsing filename");
        get_content_file(filename, |e| error!("Error reading file: {}", e))
    } else {
        info!("parsing web content");
        get_content_web(url)?
    })
}

/**
 * Прочитать контент из файла с базовой обработкой 
 */
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

/**
 * Получить контент по сети по URL и вернуть его как текст
 */ 
pub fn get_content_web(url: String) -> Result<String, ureq::Error> {
    ureq::get(url)
        .call()?
        .body_mut()
        .read_to_string()
}

/**
 * Распознать ссылки на изображения 
 * из текста HTML страницы
*/
pub fn parse_images(content: String) -> Vec<String> {
    let document = Html::parse_document(&content);
    let sel = Selector::parse("a[href^='/images/media/satel/res']")
        .expect("Selector parsing failed!");
    get_images_paths(document.select(&sel))
}

/**
 * Сконструировать форматированную строку текущей даты и времени 
*/
pub fn datetime_filename_str() -> String {
    datetime_filename_str_raw(Utc::now())
}

/**
 * Сконструировать форматированную строку на основе 
 * поданной даты\времени.
 */
pub fn datetime_filename_str_raw(dt: DateTime<Utc>) -> String {
    dt.format("%Y%m%d_%H%M").to_string()
}

pub fn process_images_gif(host: &str, images: Vec<String>) -> Result<(), ureq::Error> {
    for (index, image_path) in images
        .iter()
        .enumerate()
    {
        info!("{}: {}", index, image_path);

        let image_content = get_content_bytes(String::from(host) + image_path)?;

        std::fs::write(
            format!("img_{}_{}.gif", index, 
                datetime_filename_str()
            ), 
            image_content
        )?;
    }
    Ok(())
}

pub fn get_synaptic_url(host: &String) -> String {
    format!("{}/mapsynop", host)
}

pub fn parse_synaptic(content: String) -> Vec<String> {
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

        std::fs::write(
            format!("syn_img_{}_{}.png", index, 
                datetime_filename_str()
            ), 
            image_content
        )?;
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

