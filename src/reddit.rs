use rayon::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Data {
    pub children: Option<Vec<Wrapper>>,
    pub url: Option<String>,
    pub is_video: Option<bool>,
    pub post_hint: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Wrapper {
    pub data: Data,
}

// TODO: Should be able to define time frame, and minimum point threshold.
pub struct Api {}

impl Api {
    /// Get the daily top image links for a single subreddit.
    pub fn get_top_posts_from_sub(&self, sub: &String) -> Result<Vec<String>, String> {
        let url = format!("https://reddit.com/r/{}/top.json?t=day", sub);

        let res = match reqwest::blocking::get(url) {
            Ok(a) => a,
            Err(e) => return Err(e.to_string()),
        };
        let data = match res.json::<Wrapper>() {
            Ok(a) => a,
            Err(e) => return Err(e.to_string()),
        };

        let link_list = data
            .data
            .children
            .unwrap()
            .par_iter()
            .map(|child| &child.data)
            .filter(|a| {
                return !a.is_video.unwrap()
                    && a.post_hint.is_some()
                    && a.post_hint.as_ref().unwrap() == "image";
            })
            .map(|a| {
                return a.url.as_ref().unwrap().to_string();
            })
            .collect();

        Ok(link_list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
