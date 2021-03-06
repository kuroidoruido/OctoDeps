use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "OctoDeps",
    about = "Simple tool to track projects dependencies.",
)]
pub struct OctodepsOpt {
    /// Config: the config file path or http url.
    #[structopt(short, long)]
    config: String,
}