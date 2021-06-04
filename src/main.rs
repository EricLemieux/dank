use dank;
use std::path::PathBuf;
use structopt::StructOpt;

mod reddit;

#[derive(Debug, StructOpt, Clone)]
struct Cli {
    /// Subreddits to download images from
    #[structopt(long, default_value = "memes", required = false, use_delimiter = true)]
    subs: Vec<String>,
    /// Directory to download images into
    #[structopt(parse(from_os_str), default_value = "/tmp/dank", required = false)]
    directory: PathBuf,
    #[structopt(default_value = "day", required = false)]
    timeframe: dank::reddit::Timeframe,
}

fn main() {
    let args: Cli = Cli::from_args();

    let options = dank::Options {
        subs: args.subs.clone(),
        directory: args.directory.clone(),
        timeframe: args.timeframe.clone(),
    };

    match dank::download_memes(options) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e)
        }
    }
}
