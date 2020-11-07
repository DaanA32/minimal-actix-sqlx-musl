
mod assets;

use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, middleware, web};
use dotenv::dotenv;
use serde_json::json;
use std::env;
use env_logger::Env;
use sqlx::{Pool, MySql, mysql::{MySqlPoolOptions}};
type DbPool = Pool<MySql>;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Model {
    demo: Option<String>
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(database_url.as_str()).await;

    let pool = pool.unwrap();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(move || {
        App::new()
                .wrap(middleware::Logger::default())
                .data(pool.clone())
                .route("/api/get", web::get().to(test))
                .service(assets::get)
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

async fn test(_req: HttpRequest, pool: web::Data<DbPool> ) -> impl Responder {
    
    let result = get_all(&pool).await;

    match result {
        Ok(vec) => HttpResponse::Ok().body(json!(vec)),
        Err(_) => HttpResponse::NotFound().body(""),
    }
}

async fn get_all(
    pool: &DbPool,
) -> Result<Vec<Model>, sqlx::Error> {
    sqlx::query_as!(
        Model,
        r#"
SELECT *
FROM musl
        "#
    )
    .fetch_all(pool)
    .await
}
