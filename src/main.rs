use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn index(req: HttpRequest) -> impl Responder {
    let method = req.method().as_str();
    let uri = req.uri().to_string();
    println!("[METHOD][{}][{}]", method, uri);
    HttpResponse::Ok().body("hello, world")
}

async fn post_index(req: HttpRequest, body: web::Bytes) -> impl Responder {
    let method = req.method().as_str();
    let uri = req.uri().to_string();
    let body_str = String::from_utf8(body.to_vec()).unwrap();
    println!("[METHOD][{}][{}]", method, uri);
    println!("json:{}",body_str );
    HttpResponse::Created().body("salvando")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    HttpServer::new( || {
        App::new()
            .route("/", web::get().to(index))
            .route("/post", web::post().to(post_index))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
