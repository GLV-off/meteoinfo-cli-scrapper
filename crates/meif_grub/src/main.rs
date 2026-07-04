mod config;

use clap::Parser;
use log::{debug, error, info};
use std::path::Path;
use meif_lib::consts::GLV_METEOINFO_HOST_DEFAULT;

/* main logging configuration filename */
pub const LOG_CFG: &str = "log.yaml";

/*
    Entry point
*/
#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    setup_log();

    match GrubConfig::try_parse() {
        Ok(app) => {
            let mut env = GrubEnv::new(&app.work_dir);
            env.set_root_path(get_application_full_path());

            match create_dir_if_not_exist(&env.work_dir) {
                Err(error) => error!("create_dir_if_not_exist: {}", error),
                _ => info!("CLI config parsed"),
            }

            let hst = String::from(GLV_METEOINFO_HOST_DEFAULT);
            let dir = app.work_dir;

            match download_images(hst, &dir) {
                Err(error) => error!("download_images: {}", error),
                _ => info!("images download finished"),
            }
        }
        Err(error) => error!("GrubConfig::try_parse: {}", error),
    }

    Ok(())
}

/*

*/
pub fn get_application_full_path() -> String {
    let variables: Vec<String> = std::env::args().collect();
    variables[0].clone()
}

/*
    Initialization of logging
*/
pub fn setup_log() {
    match log4rs::init_file(LOG_CFG, Default::default()) {
        Err(error) => error!("setup_application: error: {}", error),
        _ => info!("logging initialized"),
    }
}

/*
    Create directory at path if this directory
    not exist's at specified path.
*/
fn create_dir_if_not_exist<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    if !path
        .as_ref()
        .exists()
    {
        std::fs::create_dir(path)?;
    }
    Ok(())
}

/*
    Downloading images to specified directory
    and from host's
*/
pub fn download_images<P: AsRef<Path>>(
    host: String,
    path: P,
) -> anyhow::Result<()> {
    /* get current date in format YYYY_MM_DD = `fmt` */
    let fmt = get_current_date();
    debug!("fmt = {}", fmt);
    debug!("path = {:?}", path.as_ref());
    

    /* check, if directory with this mask exist's */
    // debug!("p = {}", p);
    // let p = format!("{}\\{}", path, fmt);
    // let pp: std::path::Path = std::path::Path::from(path);
    // debug!("pp = {:?}", pp);
    // create_dir_if_not_exist(pp)?;

    // match create_dir_if_not_exist() {

    // }

    /* create subdirectory in `fmt` if not exist */

    /* Create posible image list */

    /* for all images */

    /* evaluate expected filename on this filepath */

    /* check, if image url and file downloaded
    has up to date. If image file older than 1 hour
    we download new file image */

    Ok(())
}

pub fn get_date_str(lcl: chrono::DateTime<chrono::Local>) -> String {
    lcl.format("%Y_%m_%d")
        .to_string()
}

pub fn get_current_date() -> String {
    let l = chrono::Local::now();
    get_date_str(l)
}


#[cfg(test)]
mod tests {
    #[test]
    fn current_date_constructed_as_expected() {
        // let secs = 0;
        // let dt = chrono::DateTime::from_timestamp_secs(secs)
            // .unwrap();
        // let d = get_date_str(dt);
        // assert_eq!("2026_06_06", d);
        assert!(false, "not implemeneted!");
    }
}