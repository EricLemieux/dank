use handlebars::Handlebars;
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::fs::{create_dir, File};
use std::io::Write;
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
    timeframe: reddit::Timeframe,
}

fn main() {
    let args: Cli = Cli::from_args();

    // Create the output directory if it doesn't exist
    if !args.directory.is_dir() {
        create_dir(args.directory.to_str().unwrap()).unwrap()
    }

    let api = reddit::Api {
        timeframe: args.clone().timeframe,
    };

    let images: Vec<String> = args
        .subs
        .par_iter()
        .map(|sub| {
            eprintln!("Downloading from: {}", sub);
            let res = match api.get_top_posts_from_sub(&String::from(sub)) {
                Ok(a) => a,
                Err(e) => {
                    eprintln!(
                        "Unable to get the top links from the sub {:?}, {:?}",
                        sub, e
                    );

                    Vec::new()
                }
            };

            let sub_images: Vec<String> = res
                .par_iter()
                .map(|link| match download_image(&link, &args.directory.clone()) {
                    Ok(a) => Some(a),
                    Err(_) => None,
                })
                .filter(|value| value.is_some())
                .map(|value| value.unwrap())
                .collect();

            sub_images
        })
        .reduce(Vec::new, |a: Vec<String>, b: Vec<String>| {
            a.into_iter().chain(b.into_iter()).collect()
        });

    let html = generate_html(images);

    let html_path = args.directory.join("index.html");
    let mut html_file = File::create(html_path.to_str().unwrap()).unwrap();
    html_file.write_all(html.as_ref()).unwrap();
}

/// Download an image from the provided url into the provided directory.
fn download_image(image_url: &str, download_directory: &PathBuf) -> Result<String, String> {
    let file_name = extract_file_name_from_url(image_url);
    let path = download_directory.join(&file_name);

    if path.is_file() {
        eprintln!(
            "Url: {:?}, file_name: {:?}, File already exists, not downloading again",
            image_url, file_name
        );
        return Ok(file_name);
    }
    eprintln!("Url: {:?}, file_name: {:?}", image_url, file_name);

    let res = match reqwest::blocking::get(image_url) {
        Ok(data) => match data.bytes() {
            Ok(bytes) => bytes,
            Err(e) => {
                return Err(format!(
                    "Unable to extract bytes from {} due to error {}",
                    image_url, e
                ));
            }
        },
        Err(e) => {
            return Err(format!(
                "Unable to download image due to the error: {:?}",
                e
            ));
        }
    };

    let mut file = File::create(path.to_str().unwrap()).unwrap();
    file.write_all(&*res).unwrap();

    Ok(file_name)
}

/// Extract a file's name from it's URL.
///
/// # Examples
///
/// ```
/// let result = extract_file_name_from_url("example.com/foo/bar/baz/example.jpg");
/// assert_eq!(result, "example.jpg");
/// ```
fn extract_file_name_from_url(image_url: &str) -> String {
    let re = Regex::new(r"^.*/(?P<file_name>[^/]*)$").unwrap();
    let caps = re.captures(image_url).unwrap();

    String::from(&caps["file_name"])
}

/// Generate an html page that contains all of the downloaded images.
fn generate_html(images: Vec<String>) -> String {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string("html", include_str!("templates/index.hbs"))
        .unwrap();

    let mut template_data = HashMap::new();
    template_data.insert("images", images);

    handlebars.render("html", &template_data).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_file_name_from_url_ok() {
        let result = extract_file_name_from_url("example.com/foo/bar/baz/example.jpg");
        assert_eq!(result, "example.jpg");
    }
}
