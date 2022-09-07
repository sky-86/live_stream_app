use actix_web::{middleware::Logger, web::{get, Data}, App, Error as ActError, HttpResponse, HttpServer, Result};
use dotenv::dotenv;
use fs::NamedFile;
use std::{env, path::PathBuf};
use actix_files as fs;

async fn index() -> Result<NamedFile> {
    let path = PathBuf::from("./static/index.html");
    let file = NamedFile::open(path)?;
    Ok(file)
}

#[actix_web::main]
async fn main() -> Result<(), ActError> {
    env_logger::init();
    dotenv().ok();
    let url = env::var("URL").unwrap();

    // SERVER BUILDER
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(fs::Files::new("/static", "./static"))
            .route("/", get().to(index))
    })
    .workers(1)
    .bind(url)?
    .run();

    server.await?;

    Ok(())
}
