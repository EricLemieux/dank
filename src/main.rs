use handlebars::Handlebars;
use rayon::prelude::*;
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::{create_dir, File};
use std::io::Write;
use std::path::PathBuf;
use structopt::StructOpt;

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
    let args: Cli = Cli::from_args();

    // Create the output directory if it doesn't exist
    if !args.directory.is_dir() {
        create_dir(args.directory.to_str().unwrap()).unwrap()
    }

    let images: Vec<String> = args
        .subs
        .par_iter()
        .map(|sub| {
            eprintln!("Downloading from: {}", sub);
            let res = get_top_links_from_sub(String::from(sub)).unwrap();

            let sub_images: Vec<String> = res
                .par_iter()
                .map(|link| {
                    return download_image(&String::from(link), &args.directory).unwrap();
                })
                .collect();

            return sub_images;
        })
        .reduce(
            || vec![],
            |a: Vec<String>, b: Vec<String>| {
                return a.into_iter().chain(b.into_iter()).collect();
            },
        );

    let html = generate_html(images);

    let html_path = args.directory.join("index.html");
    let mut html_file = File::create(html_path.to_str().unwrap()).unwrap();
    html_file.write_all(html.as_ref()).unwrap();
}

#[derive(Deserialize, Debug)]
struct Data {
    children: Option<Vec<Wrapper>>,
    url: Option<String>,
    is_video: Option<bool>,
    post_hint: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Wrapper {
    data: Data,
}

/// Get the links of the top rated images for the day from a single subreddit.
fn get_top_links_from_sub(sub: String) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let url = format!("https://reddit.com/r/{}/top.json?t=day", sub);

    let res = reqwest::blocking::get(url)?.json::<Wrapper>()?;

    let link_list = res
        .data
        .children
        .unwrap()
        .par_iter()
        .map(|child| {
            return &child.data;
        })
        .filter(|a| {
            return !a.is_video.unwrap() && a.post_hint.as_ref().unwrap() == "image";
        })
        .map(|a| {
            return a.url.as_ref().unwrap().to_string();
        })
        .collect();

    return Ok(link_list);
}

fn download_image(
    image_url: &String,
    download_directory: &PathBuf,
) -> Result<String, Box<dyn std::error::Error>> {
    let re = Regex::new(r"^.*/(?P<file_name>[^/]*)$").unwrap();
    let caps = re.captures(image_url).unwrap();

    let file_name = &caps["file_name"];
    let path = download_directory.join(file_name);

    eprintln!("Url: {:?}, file_name: {:?}", image_url, file_name);

    if path.is_file() {
        eprintln!("File already exists, not downloading again");
        return Ok(extract_file_name(path));
    }

    let res = reqwest::blocking::get(image_url)?.bytes()?;

    let mut file = File::create(path.to_str().unwrap()).unwrap();
    file.write_all(&*res).unwrap();

    return Ok(extract_file_name(path));
}

fn extract_file_name(path: PathBuf) -> String {
    return path
        .file_name()
        .unwrap()
        .to_os_string()
        .into_string()
        .unwrap();
}

fn generate_html(images: Vec<String>) -> String {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string("html", include_str!("templates/index.hbs"))
        .unwrap();

    let mut template_data = HashMap::new();
    template_data.insert("images", images);

    let html = handlebars.render("html", &template_data).unwrap();

    return html;
}
