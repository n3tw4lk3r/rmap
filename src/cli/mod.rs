use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Args {
    pub target: String,
    #[arg(short, long, default_value = "1000")]
    pub timeout: u64,
}

impl Args {
    pub fn parse_args() -> Self {
        Parser::parse()
    }
}
