use chrono::NaiveDate;

use crate::graph;
use crate::posts::{self, Post};

pub fn home(posts: &[Post], today: NaiveDate) -> tera::Context {
    let mut ctx = tera::Context::new();
    ctx.insert("posts", posts);
    ctx.insert("tags", &posts::tag_counts(posts));
    ctx.insert("graph", &graph::build(posts, today));
    ctx
}

pub fn post(posts: &[Post], idx: usize) -> tera::Context {
    let post = &posts[idx];
    let mut ctx = tera::Context::new();
    ctx.insert("post", post);
    ctx.insert("content", &post.content);
    ctx.insert("problem", &post.problem);
    ctx.insert("newer", &idx.checked_sub(1).and_then(|i| posts.get(i)));
    ctx.insert("older", &posts.get(idx + 1));
    ctx
}

pub fn list(kicker: &str, heading: &str, posts: &[&Post]) -> tera::Context {
    let mut ctx = tera::Context::new();
    ctx.insert("kicker", kicker);
    ctx.insert("heading", heading);
    ctx.insert("posts", posts);
    ctx
}

pub fn tag_posts<'a>(posts: &'a [Post], slug: &str) -> Option<(String, Vec<&'a Post>)> {
    let matching: Vec<&Post> = posts
        .iter()
        .filter(|p| p.tags.iter().any(|t| t.slug == slug))
        .collect();
    let name = matching
        .first()?
        .tags
        .iter()
        .find(|t| t.slug == slug)?
        .name
        .clone();
    Some((format!("#{name}"), matching))
}

pub fn day_posts(posts: &[Post], date: NaiveDate) -> Vec<&Post> {
    posts.iter().filter(|p| p.date == date).collect()
}
