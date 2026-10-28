use actix_web::http::StatusCode;
use actix_web::{HttpResponse, Responder, get, web};
use chrono::NaiveDate;

use crate::Site;
use crate::posts::{Post, pretty_date, slugify};

fn list_page(site: &Site, kicker: &str, heading: &str, posts: Vec<&Post>) -> HttpResponse {
    let mut ctx = tera::Context::new();
    ctx.insert("kicker", kicker);
    ctx.insert("heading", heading);
    ctx.insert("posts", &posts);
    site.render("list.html", &ctx, StatusCode::OK)
}

#[get("/posts/{slug}")]
pub async fn post(site: web::Data<Site>, path: web::Path<String>) -> impl Responder {
    site.refresh();
    let raw = path.into_inner();
    let slug = slugify(raw.strip_suffix(".md").unwrap_or(&raw));
    let posts = site.posts();
    let Some(idx) = posts.iter().position(|p| p.slug == slug) else {
        return not_found_page(&site);
    };
    let post = &posts[idx];

    let mut ctx = tera::Context::new();
    ctx.insert("post", post);
    ctx.insert("content", &post.content);
    ctx.insert("problem", &post.problem);
    ctx.insert("newer", &idx.checked_sub(1).and_then(|i| posts.get(i)));
    ctx.insert("older", &posts.get(idx + 1));
    site.render("post.html", &ctx, StatusCode::OK)
}

#[get("/tags/{tag}")]
pub async fn tag(site: web::Data<Site>, path: web::Path<String>) -> impl Responder {
    site.refresh();
    let slug = slugify(&path.into_inner());
    let posts = site.posts();
    let matching: Vec<&Post> = posts
        .iter()
        .filter(|p| p.tags.iter().any(|t| t.slug == slug))
        .collect();
    let Some(name) = matching
        .first()
        .and_then(|p| p.tags.iter().find(|t| t.slug == slug))
        .map(|t| t.name.clone())
    else {
        return not_found_page(&site);
    };
    list_page(&site, "tagged", &format!("#{name}"), matching)
}

#[get("/on/{date}")]
pub async fn day(site: web::Data<Site>, path: web::Path<String>) -> impl Responder {
    site.refresh();
    let Ok(date) = NaiveDate::parse_from_str(&path.into_inner(), "%Y-%m-%d") else {
        return not_found_page(&site);
    };
    let posts = site.posts();
    let matching: Vec<&Post> = posts.iter().filter(|p| p.date == date).collect();
    if matching.is_empty() {
        return not_found_page(&site);
    }
    list_page(&site, "posted on", &pretty_date(date), matching)
}

fn not_found_page(site: &Site) -> HttpResponse {
    site.render("404.html", &tera::Context::new(), StatusCode::NOT_FOUND)
}

pub async fn not_found(site: web::Data<Site>) -> impl Responder {
    not_found_page(&site)
}
