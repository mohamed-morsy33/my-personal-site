use chrono::NaiveDate;
use serde_json::{Value, json};

use crate::posts::{self, Post};
use crate::{SITE_URL, graph};

const AUTHOR: &str = "Mohamed Morsy";

fn person() -> Value {
    json!({
        "@type": "Person",
        "name": AUTHOR,
        "url": SITE_URL,
        "sameAs": ["https://github.com/mohamed-morsy33"],
        "knowsAbout": ["Mathematics", "Physics", "Calculus", "Vector Calculus", "Mechanics", "Mathematical Proofs"]
    })
}

fn insert_seo(
    ctx: &mut tera::Context,
    title: String,
    description: String,
    path: &str,
    kind: &str,
    noindex: bool,
    schema: Option<Value>,
) {
    // "<" is escaped so post text can never close the JSON-LD script tag
    let schema = schema.map(|s| s.to_string().replace('<', "\\u003c"));
    ctx.insert("seo", &json!({
        "title": title,
        "description": description,
        "url": format!("{SITE_URL}{path}"),
        "kind": kind,
        "noindex": noindex,
        "schema": schema,
    }));
}

pub fn not_found() -> tera::Context {
    let mut ctx = tera::Context::new();
    insert_seo(&mut ctx, format!("Page not found | {AUTHOR}"), "This page could not be found.".into(), "/404", "website", true, None);
    ctx
}

pub fn home(posts: &[Post], today: NaiveDate) -> tera::Context {
    let mut ctx = tera::Context::new();
    ctx.insert("posts", posts);
    ctx.insert("tags", &posts::tag_counts(posts));
    ctx.insert("graph", &graph::build(posts, today));
    let schema = json!({
        "@context": "https://schema.org",
        "@graph": [
            { "@type": "WebSite", "name": "Mohamed Morsy", "alternateName": "thebigmo.dev", "url": SITE_URL },
            person()
        ]
    });
    insert_seo(
        &mut ctx,
        format!("{AUTHOR} | Math Problems & Physics Solutions"),
        format!("{AUTHOR}'s blog of worked math problems and physics solutions, covering calculus, proofs, vector calculus, mechanics and simulations."),
        "/",
        "website",
        false,
        Some(schema),
    );
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
    let path = format!("/posts/{}/", post.slug);
    let keywords: Vec<&str> = post.tags.iter().map(|t| t.name.as_str()).collect();
    let schema = json!({
        "@context": "https://schema.org",
        "@type": "BlogPosting",
        "headline": post.title,
        "description": post.description,
        "datePublished": post.posted,
        "url": format!("{SITE_URL}{path}"),
        "mainEntityOfPage": format!("{SITE_URL}{path}"),
        "keywords": keywords,
        "author": person()
    });
    insert_seo(
        &mut ctx,
        format!("{} | {AUTHOR}", post.title),
        post.description.clone(),
        &path,
        "article",
        post.noindex,
        Some(schema),
    );
    ctx
}

pub fn list(kicker: &str, heading: &str, path: &str, posts: &[&Post]) -> tera::Context {
    let mut ctx = tera::Context::new();
    ctx.insert("kicker", kicker);
    ctx.insert("heading", heading);
    ctx.insert("posts", posts);
    // date archives only repeat posts, so only tag pages are worth indexing
    let is_tag = path.starts_with("/tags/");
    let name = heading.trim_start_matches('#');
    let (title, description) = if is_tag {
        (
            format!("{name} Problems & Solutions | {AUTHOR}"),
            format!("Worked {name} problems and step-by-step solutions by {AUTHOR}."),
        )
    } else {
        (
            format!("Problems posted on {name} | {AUTHOR}"),
            format!("Math and physics problems {AUTHOR} posted on {name}."),
        )
    };
    insert_seo(&mut ctx, title, description, path, "website", !is_tag, None);
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
