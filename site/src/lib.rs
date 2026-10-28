use std::net::TcpListener;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use actix_files::Files;
use actix_web::http::StatusCode;
use actix_web::{App, HttpResponse, HttpServer, dev::Server, middleware, web};
use tera::Tera;

pub mod graph;
pub mod handlers;
pub mod posts;

use posts::Post;

pub struct Site {
    pub root: PathBuf,
    pub live_reload: bool,
    tera: RwLock<Tera>,
    posts: RwLock<Arc<Vec<Post>>>,
}

fn load_templates(root: &PathBuf) -> Result<Tera, String> {
    let glob = root.join("templates").join("**").join("*.html");
    let glob = glob.to_string_lossy().replace('\\', "/");
    let mut tera = Tera::new();
    tera.autoescape_on(vec![".html"]);
    let version = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    tera.global_context().insert("v", &version);
    tera.load_from_glob(&glob).map_err(|e| format!("{e:?}"))?;
    Ok(tera)
}

impl Site {
    pub fn new(root: PathBuf, live_reload: bool) -> Result<Self, String> {
        let tera = load_templates(&root)?;
        let posts = posts::load_posts(&root, live_reload);
        log::info!("loaded {} posts from {}", posts.len(), root.display());
        Ok(Self {
            root,
            live_reload,
            tera: RwLock::new(tera),
            posts: RwLock::new(Arc::new(posts)),
        })
    }

    pub fn refresh(&self) {
        if !self.live_reload {
            return;
        }
        match load_templates(&self.root) {
            Ok(t) => *self.tera.write().unwrap() = t,
            Err(e) => log::error!("template reload failed: {e}"),
        }
        *self.posts.write().unwrap() = Arc::new(posts::load_posts(&self.root, true));
    }

    pub fn posts(&self) -> Arc<Vec<Post>> {
        self.posts.read().unwrap().clone()
    }

    pub fn render(&self, name: &str, ctx: &tera::Context, status: StatusCode) -> HttpResponse {
        match self.tera.read().unwrap().render(name, ctx) {
            Ok(body) => HttpResponse::build(status)
                .content_type("text/html; charset=utf-8")
                .body(body),
            Err(e) => {
                log::error!("render of {name} failed: {e:?}");
                HttpResponse::InternalServerError()
                    .content_type("text/html; charset=utf-8")
                    .body("<p>Something went wrong</p>")
            }
        }
    }
}

pub fn site_root() -> PathBuf {
    std::env::var("SITE_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(env!("CARGO_MANIFEST_DIR")))
}

pub fn start_blog(listener: TcpListener) -> Result<Server, std::io::Error> {
    let live_reload = std::env::var("LIVE_RELOAD")
        .map(|v| v != "0")
        .unwrap_or(cfg!(debug_assertions));
    let site = Site::new(site_root(), live_reload).map_err(std::io::Error::other)?;
    let static_dir = site.root.join("static");
    let data = web::Data::new(site);

    let srv = HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .route("/health", web::get().to(HttpResponse::Ok))
            .service(handlers::index)
            .service(handlers::post)
            .service(handlers::tag)
            .service(handlers::day)
            .service(handlers::graph_json)
            .service(Files::new("/static", static_dir.clone()))
            .default_service(web::to(handlers::not_found))
    })
    .listen(listener)?
    .run();

    Ok(srv)
}
