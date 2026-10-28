use actix_web::http::StatusCode;
use actix_web::{HttpResponse, Responder, get, web};
use chrono::Local;

use crate::{Site, graph, posts};

#[get("/")]
pub async fn index(site: web::Data<Site>) -> impl Responder {
    site.refresh();
    let posts = site.posts();
    let today = Local::now().date_naive();
    let graph = graph::build(&posts, today);

    let mut ctx = tera::Context::new();
    ctx.insert("posts", posts.as_slice());
    ctx.insert("tags", &posts::tag_counts(&posts));
    ctx.insert("graph", &graph);
    site.render("home.html", &ctx, StatusCode::OK)
}

#[get("/api/graph")]
pub async fn graph_json(site: web::Data<Site>) -> impl Responder {
    site.refresh();
    let posts = site.posts();
    let graph = graph::build(&posts, Local::now().date_naive());
    HttpResponse::Ok().json(graph)
}
