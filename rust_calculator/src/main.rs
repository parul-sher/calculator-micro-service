use actix_web::{App, get, HttpResponse, HttpServer, Responder};

#[get("/health_check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("I'm alive!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ip_address: &str = "127.0.0.1";
    let port_number: u16 = 8080;
    println!("Starting server at http://{}:{}", ip_address, port_number);
    HttpServer::new(|| {
        App::new().service(health_check)
    }).bind((ip_address, port_number))?.run().await
}
