use std::{collections::HashMap, convert::Infallible, str::FromStr};

use chrono::{format::parse, DateTime, Utc};
use regex::Regex;

#[derive(Debug, Clone)]
pub enum ReservationConflictInfo {
    Parsed(ReservationConflict),
    Unparsed(String),
}

#[derive(Debug, Clone)]
pub struct ReservationConflict {
    a: ReservationWindow,
    b: ReservationWindow,
}

#[derive(Debug, Clone)]
pub struct ReservationWindow {
    rid: String,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

impl FromStr for ReservationConflictInfo {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(conflict) = s.parse() {
            Ok(ReservationConflictInfo::Parsed(conflict))
        } else {
            Ok(ReservationConflictInfo::Unparsed(s.to_string()))
        }
    }
}

impl FromStr for ReservationConflict {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

impl TryFrom<ParsedInfo> for ReservationConflict {
    type Error = ();

    fn try_from(value: ParsedInfo) -> Result<Self, Self::Error> {
        Ok(Self {
            a: value.a.try_into()?,
            b: value.b.try_into()?,
        })
    }
}

impl TryFrom<HashMap<String, String>> for ReservationWindow {
    type Error = ();

    fn try_from(value: HashMap<String, String>) -> Result<Self, Self::Error> {
        let mut split = value.get("timespan").ok_or(())?.splitn(2, ',');
        let start = split.next().ok_or(())?.trim().parse().map_err(|_| ())?;
        let end = split.next().ok_or(())?.trim().parse().map_err(|_| ())?;
        Ok(Self {
            rid: value.get("resource_id").ok_or(())?.to_string(),
            start,
            end,
        })
    }
}
struct ParsedInfo {
    a: HashMap<String, String>,
    b: HashMap<String, String>,
}

impl FromStr for ParsedInfo {
    type Err = ();

    // "Key (resource_id, timespan)=(ocean-view-room-713, [\"2022-12-26 22:00:00+00\",\"2022-12-30 19:00:00+00\")) conflicts with existing key (resource_id, timespan)=(ocean-view-room-713, [\"2022-12-25 22:00:00+00\",\"2022-12-28 19:00:00+00\"))."
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // use regular expression to parse the string
        let re = Regex::new(r"\((?P<k1>[a-zA-Z0-9_-]+)\s*,\s*(?P<k2>[a-zA-Z0-9_-]+)\)=\((?P<v1>[a-zA-Z0-9-_]+)\s*,\s*\[(?P<v2>)[^\)\]]+\)").unwrap();
        let mut maps = vec![];
        for cap in re.captures_iter(s) {
            let mut map = HashMap::new();
            map.insert(cap["k1"].to_string(), cap["v1"].to_string());
            map.insert(cap["k2"].to_string(), cap["v2"].to_string());
            maps.push(Some(map));
        }
        if maps.len() != 2 {
            return Err(());
        }
        Ok(ParsedInfo {
            a: maps[0].take().unwrap(),
            b: maps[1].take().unwrap(),
        })
    }
}
