use actix_web::{web, App, HttpResponse, HttpServer, Responder, HttpRequest};

async fn index(req: HttpRequest) -> impl Responder {
    let method = req.method().as_str();
    let uri = req.uri().to_string();
    println!("[METHOD][{}][{}]", method, uri);
    HttpResponse::Ok().body("hello, world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    HttpServer::new( || {
        App::new()
            .route("/", web::get().to(index))
            .route("/", web::post().to(index))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
