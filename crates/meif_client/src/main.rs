use meif_lib::{*, consts};

#[tokio::main]
async fn main() -> Result<(), ureq::Error> {
    setup_logging();

    let host = get_host();
    let url_images = get_images_url(&host);
    let content = get_content(url_images, consts::FILENAME_IMAGES)?;
    let images_paths = parse_images(content);
    process_images_gif(&host, images_paths)?;

    let url_synaptic = get_synaptic_url(&host);
    let content = get_content(url_synaptic, consts::FILENAME_SYNAPT)?;
    let synaptic_paths = parse_synaptic_from_content(content);
    process_images_png(synaptic_paths)?;
 
    Ok(())
}
