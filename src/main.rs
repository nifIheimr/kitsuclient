use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use kitsu_app::{client::kitsu::get_animes, model::anime::Anime}; 

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(anime_trending)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/anime/trending")]
async fn anime_trending() -> web::Json<Vec<Anime>> {
    let animes: Vec<Anime> = get_animes().await.unwrap().data;
    web::Json(animes)
}
