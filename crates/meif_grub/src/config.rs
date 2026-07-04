use clap::Parser;

/*
    Main CLI application command line argument's
*/
#[derive(Debug, Parser)]
pub struct GrubConfig {
    #[arg(short, long)]
    pub work_dir: String,

    #[arg(short, long)]
    pub cfg: String,
}