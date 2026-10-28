use std::collections::{BTreeSet, HashMap};

use chrono::{Datelike, Duration, Months, NaiveDate};
use serde::Serialize;

use crate::posts::{Post, pretty_date};

#[derive(Serialize)]
pub struct Day {
    pub date: String,
    pub label: String,
    pub count: usize,
    pub level: u8,
    pub pad: bool,
    pub future: bool,
    pub today: bool,
    pub href: String,
    pub titles: Vec<String>,
}

#[derive(Serialize)]
pub struct Week {
    pub days: Vec<Day>,
}

#[derive(Serialize)]
pub struct MonthLabel {
    pub name: String,
    pub col: usize,
}

#[derive(Serialize)]
pub struct Graph {
    pub weeks: Vec<Week>,
    pub months: Vec<MonthLabel>,
    pub columns: usize,
    pub total: usize,
    pub active_days: usize,
    pub current_streak: usize,
    pub longest_streak: usize,
    pub range_label: String,
}

fn streaks(dates: &BTreeSet<NaiveDate>, today: NaiveDate) -> (usize, usize) {
    let mut longest = 0;
    let mut run = 0;
    let mut prev: Option<NaiveDate> = None;
    for &d in dates {
        run = match prev {
            Some(p) if d - p == Duration::days(1) => run + 1,
            _ => 1,
        };
        longest = longest.max(run);
        prev = Some(d);
    }

    let mut cursor = if dates.contains(&today) {
        today
    } else {
        today - Duration::days(1)
    };
    let mut current = 0;
    while dates.contains(&cursor) {
        current += 1;
        cursor -= Duration::days(1);
    }
    (current, longest)
}

pub fn build(posts: &[Post], today: NaiveDate) -> Graph {
    let end = today;
    let start = today
        .checked_sub_months(Months::new(6))
        .map_or(today - Duration::days(182), |d| d + Duration::days(1));

    let grid_start = start - Duration::days(start.weekday().num_days_from_sunday() as i64);
    let grid_end = end + Duration::days(6 - end.weekday().num_days_from_sunday() as i64);

    let mut by_day: HashMap<NaiveDate, Vec<&Post>> = HashMap::new();
    for post in posts {
        by_day.entry(post.date).or_default().push(post);
    }

    let mut weeks = Vec::new();
    let mut months: Vec<MonthLabel> = Vec::new();
    let mut total = 0;
    let mut active_days = 0;
    let mut cursor = grid_start;
    let mut col = 0;

    while cursor <= grid_end {
        let mut days = Vec::with_capacity(7);
        for _ in 0..7 {
            let d = cursor;
            let pad = d < start || d > end;
            let future = d > today;
            let day_posts: &[&Post] = if pad || future {
                &[]
            } else {
                by_day.get(&d).map(|v| v.as_slice()).unwrap_or(&[])
            };
            let count = day_posts.len();
            total += count;
            if count > 0 {
                active_days += 1;
            }
            let href = match day_posts {
                [] => String::new(),
                [only] => format!("/posts/{}", only.slug),
                _ => format!("/on/{}", d.format("%Y-%m-%d")),
            };
            if !pad && d.day() == 1 {
                months.push(MonthLabel {
                    name: d.format("%b").to_string(),
                    col,
                });
            }
            days.push(Day {
                date: d.format("%Y-%m-%d").to_string(),
                label: pretty_date(d),
                count,
                level: count.min(4) as u8,
                pad,
                future,
                today: d == today,
                href,
                titles: day_posts.iter().map(|p| p.title.clone()).collect(),
            });
            cursor += Duration::days(1);
        }
        weeks.push(Week { days });
        col += 1;
    }

    if months.first().is_none_or(|m| m.col >= 3) {
        months.insert(
            0,
            MonthLabel {
                name: start.format("%b").to_string(),
                col: 0,
            },
        );
    }
    let cols = weeks.len();
    months.retain(|m| m.col + 2 <= cols);

    let dates: BTreeSet<NaiveDate> = posts.iter().map(|p| p.date).filter(|d| *d <= today).collect();
    let (current_streak, longest_streak) = streaks(&dates, today);


    Graph {
        columns: weeks.len(),
        weeks,
        months,
        total,
        active_days,
        current_streak,
        longest_streak,
        range_label: "in the last 6 months".to_string(),
    }
}
