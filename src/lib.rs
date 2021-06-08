//! Download memes from reddit in parallel so that you can more efficiently waste time.

use handlebars::Handlebars;
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::fs::{create_dir, File};
use std::io::Write;
use std::path::PathBuf;

pub mod reddit;

/// Options for what the user would like to download, such as the subreddits, and where those files
/// should download to.
pub struct Options {
    /// Subreddits that the top posts should be downloaded from.
    pub subs: Vec<String>,
    /// Where on the file system the images should be downloaded to.
    pub directory: PathBuf,
    /// The search timeframe in which top rated posts are being calculated.
    pub timeframe: reddit::Timeframe,
}

/// Download memes from reddit
pub fn download_memes(options: Options) -> Result<(), String> {
    // Create the output directory if it doesn't exist
    if !options.directory.is_dir() {
        create_dir(options.directory.to_str().unwrap()).unwrap()
    }

    let api = reddit::Api::new(options.timeframe.clone());

    let images: Vec<String> = options
        .subs
        .par_iter()
        .map(|sub| {
            eprintln!("Downloading from: {}", sub);
            let res = match api.get_top_posts_from_sub(&sub) {
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
                .map(
                    |link| match download_image(&link, &options.directory.clone()) {
                        Ok(a) => Some(a),
                        Err(_) => None,
                    },
                )
                .filter(|value| value.is_some())
                .map(|value| value.unwrap())
                .collect();

            sub_images
        })
        .reduce(Vec::new, |a: Vec<String>, b: Vec<String>| {
            a.into_iter().chain(b.into_iter()).collect()
        });

    let html = generate_html(images);

    let html_path = options.directory.join("index.html");
    let mut html_file = File::create(html_path.to_str().unwrap()).unwrap();
    html_file.write_all(html.as_ref()).unwrap();

    Ok(())
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
/// let result = dank::extract_file_name_from_url("example.com/foo/bar/baz/example.jpg");
/// assert_eq!(result, "example.jpg");
/// ```
pub fn extract_file_name_from_url(image_url: &str) -> String {
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
