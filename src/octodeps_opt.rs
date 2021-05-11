use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "OctoDeps",
    about = "Simple tool to track projects dependencies."
)]
pub struct OctodepsOpt {
    /// Config: the config file path
    #[structopt(short, long)]
    pub config: String,
}
