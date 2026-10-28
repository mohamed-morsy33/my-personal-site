use actix_web::http::StatusCode;
use actix_web::{HttpResponse, Responder, get, web};
use chrono::Local;

use crate::{Site, graph, pages};

#[get("/")]
pub async fn index(site: web::Data<Site>) -> impl Responder {
    site.refresh();
    let posts = site.posts();
    let ctx = pages::home(&posts, Local::now().date_naive());
    site.render("home.html", &ctx, StatusCode::OK)
}

#[get("/api/graph")]
pub async fn graph_json(site: web::Data<Site>) -> impl Responder {
    site.refresh();
    let posts = site.posts();
    let graph = graph::build(&posts, Local::now().date_naive());
    HttpResponse::Ok().json(graph)
}
