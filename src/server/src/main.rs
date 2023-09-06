use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct MyObj {
    game_id: String,
    platform_id: String,
}

#[get("/version")]
async fn version() -> impl Responder {
    HttpResponse::Ok().body("2.0.0")
}

#[get("/getGameMetaData/{platform_id}/{game_id}/{_}/token")]
async fn get_game_meta_data(my_obj: web::Path<MyObj>) -> Result<impl Responder> {
    Ok(web::Json(my_obj.into_inner()))
}

// #[get("/getGameMetaData/<platform_id>/<game_id>/<_>/token")]
// fn get_game_meta_data(platform_id: &str, game_id: &str) -> &'static str {
//     "Hello, world!"
// }

// #[get("/getLastChunkInfo/<platform_id>/<game_id>/<_>/token")]
// fn get_last_chunk_info(platform_id: &str, game_id: &str) -> &'static str {
//     "Hello, world!"
// }

// #[get("/getGameDataChunk/<platform_id>/<game_id>/<chunk_id>/token")]
// fn get_game_data_chunk(platform_id: &str, game_id: &str, chunk_id: &str) -> &'static str {
//     "Hello, world!"
// }

// #[get("/getKeyFrame/<platform_id>/<game_id>/<keyframe_id>/token")]
// fn get_key_frame(platform_id: &str, game_id: &str, keyframe_id: &str) -> &'static str {
//     "Hello, world!"
// }

// #[launch]
// fn rocket() -> _ {
//     rocket::build()
//         .mount(
//             "/observer-mode/rest/consumer",
//             routes![version, get_game_meta_data, get_last_chunk_info, get_game_data_chunk, get_key_frame]
//         )
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/observer-mode/rest/consumer")
                .service(version)
                .service(get_game_meta_data),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
