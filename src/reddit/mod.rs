//! Reddit API wrapper that makes it easier to grab and utilize data returned by the API.

use rayon::prelude::*;
use serde::Deserialize;
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

/// Wrapper element that is both the top level object returned by the reddit api as well as the
/// wrapping element for each child element (posts) returned in the data.
#[derive(Deserialize, Debug)]
pub struct Wrapper {
    pub data: Data,
}

#[derive(Deserialize, Debug)]
pub struct Data {
    pub children: Option<Vec<Wrapper>>,
    pub url: Option<String>,
    pub is_video: Option<bool>,
    pub post_hint: Option<String>,
}

/// Different timeframes which you can use to sort top rated posts.
/// Used in a rolling window, so they are the top rated posts from now minus the timeframe given.
/// As an example the top posts for [Timeframe::Day], are the top posts within the past 24 hours,
/// rather than a specific day.
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum Timeframe {
    Hour,
    Day,
    Week,
    Month,
    Year,
    All,
}

impl Display for Timeframe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Timeframe::Hour => write!(f, "hour"),
            Timeframe::Day => write!(f, "day"),
            Timeframe::Week => write!(f, "week"),
            Timeframe::Month => write!(f, "month"),
            Timeframe::Year => write!(f, "year"),
            Timeframe::All => write!(f, "all"),
        }
    }
}

impl FromStr for Timeframe {
    type Err = String;

    fn from_str(day: &str) -> Result<Self, Self::Err> {
        match day {
            "hour" => Ok(Timeframe::Hour),
            "day" => Ok(Timeframe::Day),
            "week" => Ok(Timeframe::Week),
            "month" => Ok(Timeframe::Month),
            "year" => Ok(Timeframe::Year),
            "all" => Ok(Timeframe::All),
            _ => Err("Not found".parse().unwrap()),
        }
    }
}

/// API implementation object, this is the main entrypoint for using the api.
pub struct Api {
    pub timeframe: Timeframe,
}

impl Api {
    /// Get the top image links for a single subreddit.
    pub fn get_top_posts_from_sub(&self, sub: &str) -> Result<Vec<String>, String> {
        let url = format!(
            "https://reddit.com/r/{}/top.json?t={}",
            sub,
            self.timeframe.to_string()
        );

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

    #[test]
    fn timeframe_to_string() {
        assert_eq!("day", Timeframe::Day.to_string());
        assert_eq!("week", Timeframe::Week.to_string());
        assert_eq!("month", Timeframe::Month.to_string());
        assert_eq!("all", Timeframe::All.to_string());
    }

    #[test]
    fn timeframe_from_string() {
        assert_eq!(Timeframe::Day, Timeframe::from_str("day").unwrap());
        assert_eq!(Timeframe::Week, Timeframe::from_str("week").unwrap());
        assert_eq!(Timeframe::Month, Timeframe::from_str("month").unwrap());
        assert_eq!(Timeframe::All, Timeframe::from_str("all").unwrap());
    }
}