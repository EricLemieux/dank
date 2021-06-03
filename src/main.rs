use handlebars::Handlebars;
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::fs::{create_dir, File};
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;
use dank;

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
    timeframe: reddit::Timeframe,
}

fn main() {
    let args: Cli = Cli::from_args();

    let options = dank::Options {
        subs: args.subs.clone(),
        directory: args.directory.clone(),
        timeframe: args.timeframe.clone(),
    };

    dank::download_memes(options)?
}
