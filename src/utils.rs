use std::fs;
use std::io::{Write};

use actix_web::{web, HttpResponse, HttpRequest, Error, Result};
use actix_web::http::{StatusCode};
use actix_multipart::Multipart;
use futures_util::TryStreamExt as _;

use rand::{distributions::Alphanumeric, Rng};

use actix_files as a_fs;

use crate::settings;


pub async fn file_save(req: HttpRequest, mut payload: Multipart) -> Result<HttpResponse, Error> {
    // iterate over multipart stream
    while let Some(mut field) = payload.try_next().await? {
        let filepath = format!("{}/{}", settings::UPLOAD_FOLDER, gen_rand_id(16));

        // File::create is blocking operation, use threadpool
        let mut f = web::block(|| fs::File::create(filepath)).await?;

        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.try_next().await? {
            // filesystem operations are blocking, we have to use threadpool
            f = web::block(move || f.write_all(&chunk).map(|_| f)).await?;
        }
    }

    println!("Handling multipart POST request:\n{:?}", req);

    Ok(HttpResponse::Ok().into())
}


/// 404 handler
pub async fn p404() -> Result<a_fs::NamedFile> {
    Ok(a_fs::NamedFile::open("frontend/static/404.html")?.set_status_code(StatusCode::NOT_FOUND))
}

// Generate a random identifier.
pub fn gen_rand_id(n: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(n)
        .map(char::from)
        .collect()
}

// pub async fn write_file(path: String, content: String) -> Result<(), Error> {
//     let mut output = web::block(File::create(path)).await?;
//     web::block(write!(output, "{}", content)).await?;
// 
