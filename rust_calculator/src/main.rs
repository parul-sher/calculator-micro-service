use actix_web::{App, get, HttpResponse, HttpServer, Responder};

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("I'm alive!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ip_address: &str = "0.0.0.0";
    let port_number: u16 = 8080;
    println!("Starting calculator server at http://{}:{}", ip_address, port_number);
    HttpServer::new(|| {
        App::new().service(health)
    }).bind((ip_address, port_number))?.run().await
}
