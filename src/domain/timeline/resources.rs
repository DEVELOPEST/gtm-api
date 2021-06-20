use chrono::{DateTime, TimeZone, Offset};
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use schemars::JsonSchema;

#[derive(Serialize, JsonSchema)]
pub struct TimelineJson {
    pub id: i32,
    pub timestamp: i64,
    pub time: i64,
}

#[derive(Debug)]
pub struct Interval<Tz: TimeZone> {
    pub start: DateTime<Tz>,
    pub end: DateTime<Tz>,
    pub time: i64,
    pub users: Vec<String>,
}

#[derive(Debug)]
pub struct Activity {
    pub id: i32,
    pub label: String,
    pub time: i64,
    pub lines_added: i64,
    pub lines_removed: i64,
    pub users: HashSet<String>,
}

#[derive(Debug)]
pub struct SubdirLevelTimeline<Tz: TimeZone> {
    pub start: DateTime<Tz>,
    pub end: DateTime<Tz>,
    pub directories: HashMap<String, SubdirLevelTimelineEntry>,
}

#[derive(Debug)]
pub struct SubdirLevelTimelineEntry {
    pub path: String,
    pub time: i64,
    pub commits: HashSet<String>,
    pub lines_added: i64,
    pub lines_removed: i64,
    pub users: HashSet<String>,
}

impl<Tz: TimeZone> Interval<Tz> {
    pub fn attach(&self) -> IntervalJson {
        IntervalJson {
            start: format!("{:?}{}", self.start.naive_local(), self.start.offset().fix()),
            end: format!("{:?}{}", self.end.naive_local(), self.end.offset().fix()),
            time: (self.time as f64 / 60.0 / 60.0 * 10.0).round() / 10.0,
            users: self.users.len() as i32,
        }
    }
}

impl Activity {
    pub fn attach(&self) -> ActivityJson {
        ActivityJson {
            label: self.label.clone(),
            label_key: self.id,
            time: ((self.time as f64) / 60.0 / 60.0 * 10.0).round() / 10.0,
            lines_added: self.lines_added,
            lines_removed: self.lines_removed,
            users: self.users.len() as i32,
        }
    }
}

impl<Tz: TimeZone> SubdirLevelTimeline<Tz> {
    pub fn attach(&self) -> SubdirLevelTimelineJson {
        SubdirLevelTimelineJson {
            start: format!("{:?}{}", self.start.naive_local(), self.start.offset().fix()),
            end: format!("{:?}{}", self.end.naive_local(), self.end.offset().fix()),
            directories: HashMap::from_iter(self.directories.iter().map(|(k, v)| (k.clone(), v.attach()))),
        }
    }
}

impl SubdirLevelTimelineEntry {
    pub fn attach(&self) -> SubdirLevelTimelineJsonEntry {
        SubdirLevelTimelineJsonEntry {
            path: self.path.clone(),
            time: (self.time as f64 / 60.0 / 60.0 * 10.0).round() / 10.0,
            commits: self.commits.len() as i64,
            lines_added: self.lines_added,
            lines_removed: self.lines_removed,
            users: self.users.len() as i64
        }
    }
}

#[derive(Serialize, JsonSchema)]
pub struct IntervalJson {
    pub start: String,
    pub end: String,
    pub time: f64,
    pub users: i32,
}

#[derive(Serialize, JsonSchema)]
pub struct ActivityJson {
    pub label: String,
    pub label_key: i32,
    pub time: f64,
    pub lines_added: i64,
    pub lines_removed: i64,
    pub users: i32,
}

#[derive(Serialize, JsonSchema)]
pub struct SubdirLevelTimelineJson {
    pub start: String,
    pub end: String,
    pub directories: HashMap<String, SubdirLevelTimelineJsonEntry>,
}

#[derive(Serialize, Clone, JsonSchema)]
pub struct SubdirLevelTimelineJsonEntry {
    pub path: String,
    pub time: f64,
    pub commits: i64,
    pub lines_added: i64,
    pub lines_removed: i64,
    pub users: i64,
}

#[derive(Serialize, JsonSchema)]
pub struct SubdirLevelTimelineJsonWrapper {
    pub paths: Vec<String>,
    pub data: Vec<SubdirLevelTimelineJson>,
}

#[derive(Debug)]
pub struct TimelineComparisonEntry<Tz: TimeZone> {
    pub start: DateTime<Tz>,
    pub end: DateTime<Tz>,
    pub time: i64,
    pub lines_added: i64,
    pub lines_removed: i64,
    pub commits: HashSet<String>,
    pub users: HashSet<String>,
}

#[derive(Serialize, JsonSchema)]
pub struct ComparisonJsonWrapper {
    pub branches: Vec<String>,
    pub users: Vec<String>,
    pub repos: Vec<String>,
    pub time: ComparisonStatJson,
    pub commits: ComparisonStatJson,
    pub lines_added: ComparisonStatJson,
    pub lines_removed: ComparisonStatJson,
    pub timeline: Vec<TimelineComparisonJsonEntry>,
    pub filtered_timeline: Vec<TimelineComparisonJsonEntry>,
}

#[derive(Serialize, JsonSchema)]
pub struct ComparisonStatJson {
    pub total: f64,
    pub highlighted: f64,
    pub rank: i32,
    pub data: Vec<ComparisonStatJsonEntry>,
}

#[derive(Serialize, JsonSchema)]
pub struct ComparisonStatJsonEntry {
    pub rank: i32,
    pub value: f64,
}

#[derive(Serialize, JsonSchema)]
pub struct TimelineComparisonJsonEntry {
    pub start: String,
    pub end: String,
    pub time: f64,
    pub commits: i64,
    pub lines_added: i64,
    pub lines_removed: i64,
    pub users: i64,
}

impl<Tz: TimeZone> From<TimelineComparisonEntry<Tz>> for TimelineComparisonJsonEntry {
    fn from(e: TimelineComparisonEntry<Tz>) -> Self {
        TimelineComparisonJsonEntry {
            start: format!("{:?}{}", e.start.naive_local(), e.start.offset().fix()),
            end: format!("{:?}{}", e.end.naive_local(), e.end.offset().fix()),
            time: (e.time as f64 / 60.0 / 60.0 * 10.0).round() / 10.0,
            commits: e.commits.len() as i64,
            lines_added: e.lines_added,
            lines_removed: e.lines_removed,
            users: e.users.len() as i64,
        }
    }
}
