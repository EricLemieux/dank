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
    /// Convert a [Timeframe] value into the string representation that the Reddit API understands.
    ///
    /// # Examples
    ///
    /// ```
    /// assert_eq!("hour", dank::reddit::Timeframe::Hour.to_string());
    /// ```
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

    /// Create a [Timeframe] from a String
    ///
    /// # Examples
    /// ```
    /// assert_eq!(dank::reddit::Timeframe::Hour, "hour".parse().unwrap());
    /// ```
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
    /// Construct an API instance with all required fields.
    ///
    /// # Examples
    ///
    /// ```
    /// let api = dank::reddit::Api::new(dank::reddit::Timeframe::All);
    /// assert_eq!("all", api.timeframe.to_string())
    /// ```
    pub fn new(timeframe: Timeframe) -> Api {
        Api { timeframe }
    }

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
    fn create_api() {
        let api = Api::new(Timeframe::Day);
        assert_eq!(Timeframe::Day, api.timeframe);
    }

    #[test]
    fn timeframe_to_string() {
        assert_eq!("hour", Timeframe::Hour.to_string());
        assert_eq!("day", Timeframe::Day.to_string());
        assert_eq!("week", Timeframe::Week.to_string());
        assert_eq!("month", Timeframe::Month.to_string());
        assert_eq!("year", Timeframe::Year.to_string());
        assert_eq!("all", Timeframe::All.to_string());
    }

    #[test]
    fn timeframe_from_string() {
        assert_eq!(Timeframe::Hour, "hour".parse().unwrap());
        assert_eq!(Timeframe::Day, "day".parse().unwrap());
        assert_eq!(Timeframe::Week, "week".parse().unwrap());
        assert_eq!(Timeframe::Month, "month".parse().unwrap());
        assert_eq!(Timeframe::Year, "year".parse().unwrap());
        assert_eq!(Timeframe::All, "all".parse().unwrap());
    }
}
