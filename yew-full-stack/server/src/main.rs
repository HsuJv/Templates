use std::sync::Arc;

use actix_files as fs;
use actix_session::CookieSession;
use actix_web::http::StatusCode;
use actix_web::*;
use log::*;
use rand::Rng;

const STATIC_DIR: &str = "./static/";
const PAGE_INDEX: &str = "./static/index.html";
const PAGE_NOT_FOUND: &str = "./static/p404.html";
const LISTEN_ADDR: &str = "0.0.0.0:8080";

pub struct AppData {
    // session: CookieSession,
}

impl AppData {
    pub fn new() -> Self {
        Self {}
    }
}

impl Default for AppData {
    fn default() -> Self {
        Self::new()
    }
}

fn setup_logger() {
    let logger = femme::pretty::Logger::new();
    async_log::Logger::wrap(logger, || 12)
        .start(log::LevelFilter::Info)
        .unwrap();
}

#[get("/")]
async fn index(_: HttpRequest) -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open(PAGE_INDEX)?)
}

async fn p404() -> Result<fs::NamedFile> {
    Ok(fs::NamedFile::open(PAGE_NOT_FOUND)?.set_status_code(StatusCode::NOT_FOUND))
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    setup_logger();

    info!("Server starts at http://{}", LISTEN_ADDR);
    let private_key = rand::thread_rng().gen::<[u8; 32]>();
    let app_data = Arc::new(AppData::new());
    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(CookieSession::signed(&private_key).secure(false))
            .wrap(middleware::Compress::default())
            .service(index)
            .service(
                fs::Files::new("/static", STATIC_DIR)
                    .prefer_utf8(true)
                    .index_file(PAGE_INDEX)
                    .use_etag(true)
                    .default_handler(web::route().to(p404)),
            )
            .default_service(web::route().to(p404))
    })
    .bind(LISTEN_ADDR)?
    .run()
    .await?;

    Ok(())
}
