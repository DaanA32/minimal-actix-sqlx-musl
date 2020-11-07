use actix_files::NamedFile;
use actix_web::{Error, HttpRequest};
use std::{path::{Path, PathBuf}};
use actix_web::get;

#[get("/{filename:.*}")]
async fn get(req: HttpRequest) -> Result<NamedFile, Error> {
    let path: std::path::PathBuf = match req.match_info().query("filename") {
        "" => PathBuf::from("index.html"),
        c => c.parse().unwrap(),
    };
    let file = NamedFile::open(Path::new("static/").join(path))?;
    Ok(file
        .use_last_modified(true))
}
