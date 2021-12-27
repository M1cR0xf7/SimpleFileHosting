use actix_web::{
    middleware,
    web,
    App,
    guard,
    HttpRequest,
    HttpResponse,
    HttpServer,
    Responder,
    Result,
};

use actix_files as a_fs;

use std::fs::read_to_string;
use env_logger::Env;
use serde::{Deserialize};



mod settings;
mod utils;

use settings::*;

#[derive(Deserialize)]
struct File {
    data: String,
}

async fn index(req: HttpRequest) -> Result<HttpResponse> {
    println!("{:?}", req);

    let content = read_to_string("frontend/index.html")
        .expect("error reading the file");

    Ok(HttpResponse::Ok()
       .content_type("text/html; charset=utf-8")
       .body(content))
}


async fn run_server() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    println!("serving on: {}:{}", HOST, PORT);

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            // .wrap(middleware::Logger::new("%a %[User-Agent]i"))
            // .route("/post", web::post().to(handle_post))
            .service(
                web::resource("/post")
                    .app_data(web::FormConfig::default().limit(4097))
                    .route(web::post().to(utils::file_save))
                )
            .service(
                a_fs::Files::new("/get", "./up/")
                )


            // serve files
            .service(
                    a_fs::Files::new("/static", "./frontend/static")
                )
                        // default
            .default_service(
                // 404 for GET request
                web::resource("")
                    .route(web::get().to(utils::p404))
                    // all requests that are not `GET`
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(HttpResponse::MethodNotAllowed),
                    ),
            )

            .route("/", web::get().to(index))
    })
    .bind([HOST, PORT].join(":"))?
    .run()
    .await

}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if !utils::exists(UPLOAD_DIR) {
        println!("upload folder is missing: expected {}", UPLOAD_DIR);
        std::process::exit(1);
    }
    if !utils::exists(FRONTEND_DIR) {
        println!("frontend folder is missing: expected {}", FRONTEND_DIR);
        std::process::exit(1);
    }


    run_server().await
}
