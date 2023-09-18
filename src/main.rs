use actix_files as fs;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn clicked() -> impl Responder {
    // print!("Clicked! : {} ", req_body);
    HttpResponse::Ok().body("<h2>Clicked!</h2>")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| 
        App::new()
            .service(fs::Files::new("/static", "./src/static").show_files_listing())
            .service(hello)
            .service(echo)
            .route("/clicked", web::post().to(clicked))
            .route("/hey", web::get().to(manual_hello))
    )
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
