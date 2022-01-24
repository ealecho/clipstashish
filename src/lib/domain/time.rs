use std::str::FromStr;
use chrono::{DateTime, NaiveDateTime, Utc};
use derive_more::From;
use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, From, Deserialize, Serialize)]
pub struct Time(DateTime<Utc>);

impl Time{
    pub fn into_inner(self) -> DateTime<Utc> {
        self.0
    }

    pub fn timestamp(&self) -> i64 {
        self.0.timestamp()
    }

    pub fn from_naive_utc(datetime: NaiveDateTime) -> Self {
        Time(DateTime::from_utc(datetime, Utc))
    }
}

impl FromStr for Time {
    type Err = chrono::ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err>{
        // Converts a date in the format 2022-05-22 into a DateTime Structure the z indicates that this is a utc timezone time
        match format!("{}T00:00:00Z", s).parse::<DateTime<Utc>>() {
            Ok(time) => Ok(time.into()),
            Err(e) => Err(e)
        }
    }
}