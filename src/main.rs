use actix_web::{HttpServer, App, Responder, get, HttpResponse, web};
use dsouls_lib::sentences::{generate_dynamic_sentence, Sentence};

/// Endpoint for custom sentence
/// 
#[get("/custom/{sentence}")]
async fn get_sentence(sentence: web::Path<(String,)>) -> impl Responder {
    let sentence = generate_dynamic_sentence(Sentence::new(&sentence.0));
    HttpResponse::Ok().body(sentence)
}

#[get("/")]
async fn index() -> impl Responder {
    let res = generate_dynamic_sentence(Sentence::new("{adjective} {noun}"));
    HttpResponse::Ok().body(res)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(get_sentence)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
