use structopt::StructOpt;
use serde::Deserialize;
use std::fs::{File, create_dir};
use std::path::{PathBuf};
use regex::Regex;
use rayon::prelude::*;
use std::io::Write;

#[derive(Debug, StructOpt)]
struct Cli {
    /// Subreddits to download images from
    #[structopt(long, default_value = "memes", required = false, use_delimiter = true)]
    subs: Vec<String>,
    /// Directory to download images into
    #[structopt(parse(from_os_str), default_value = "/tmp/dank", required = false)]
    directory: PathBuf,
}

fn main() {
    let args = Cli::from_args();

    // Create the output directory if it doesn't exist
    if !args.directory.is_dir() {
        create_dir(args.directory.to_str().unwrap()).unwrap()
    }

    args.subs.par_iter().for_each(|sub| {
        println!("Downloading from: {}", sub);
        let res = get_top_links_from_sub(String::from(sub)).unwrap();

        res.par_iter().for_each(|link| {
            download_image(&String::from(link), &args.directory).unwrap();
        });
    });
}

#[derive(Deserialize, Debug)]
struct Data {
    children: Option<Vec<Wrapper>>,
    url: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Wrapper {
    data: Data,
}

/// Get the links of the top rated images for the day from a single subreddit.
fn get_top_links_from_sub(sub: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let url = format!("https://reddit.com/r/{}/top.json?t=day", sub);

    let res= reqwest::blocking::get(url)?
        .json::<Wrapper>()?;

    // TODO: Filter out posts that we don't care about / are a pain in the ass / are not images
    let link_list = res.data.children.unwrap().iter().map(|child| {
      return child.data.url.as_ref().unwrap().to_string();
    }).collect();

    return Ok(link_list);
}


fn download_image(image_url: &String, download_directory: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    let re = Regex::new(r"^.*/(?P<file_name>[^/]*)$").unwrap();
    let caps = re.captures(image_url).unwrap();

    let file_name = &caps["file_name"];
    let path = download_directory.join(file_name);

    println!("Url: {:?}, file_name: {:?}", image_url, file_name);

    if path.is_file() {
        println!("File already exists, not downloading again");
        return Ok(())
    }

    let res= reqwest::blocking::get(image_url)?
        .bytes()?;

    let mut file = File::create(path.to_str().unwrap()).unwrap();
    file.write_all(&*res).unwrap();

    return Ok(());
}
