use std::net::TcpListener;
use std::path::PathBuf;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.first().map(String::as_str) == Some("export") {
        let out = PathBuf::from(args.get(1).map(String::as_str).unwrap_or("dist"));
        let site = bigmojourney::Site::new(bigmojourney::site_root(), false)
            .map_err(std::io::Error::other)?;
        return bigmojourney::export::export(&site, &out).map_err(std::io::Error::other);
    }

    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let listener = TcpListener::bind(format!("0.0.0.0:{port}"))?;
    log::info!("listening on http://localhost:{port}");
    bigmojourney::start_blog(listener)?.await
}
