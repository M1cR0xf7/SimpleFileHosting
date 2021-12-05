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

async fn handle_post(req: HttpRequest, d: web::Form<File>) -> impl Responder {
    println!("Handling request: {:?}", req);

    // TODO:
    // 1. generate a file name (to get it in the future by /<name>/)
    // 2. add the name to the database along with the date and time in which the file
    // was created
    // 3. write a file to the disk as <name>
    // 4. send a response with the http://127.0.0.1:8080/<name>/

    HttpResponse::Ok()
       .content_type("text/plain")
       .body(format!("{}\n---------\n{}\n", utils::gen_rand_id(16), d.data))
       // .body(format!("{}", data.title))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
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
                    .route(web::post().to(handle_post))
                )
            .service(
                web::resource("/upload")
                    .route(web::post().to(utils::file_save))
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
