use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Numbers {
    first: i32,
    second: i32,
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().body("I'm alive!")
}

#[get("/add")]
async fn add(numbers: web::Query<Numbers>) -> impl Responder {
    let sum = numbers.first + numbers.second;
    HttpResponse::Ok().body(format!("{} + {} = {}", numbers.first, numbers.second, sum))
}

#[get("/subtract")]
async fn subtract(numbers: web::Query<Numbers>) -> impl Responder {
    let difference = numbers.first - numbers.second;
    HttpResponse::Ok().body(format!(
        "{} - {} = {}",
        numbers.first, numbers.second, difference
    ))
}

#[get("/multiply")]
async fn multiply(numbers: web::Query<Numbers>) -> impl Responder {
    let product = numbers.first * numbers.second;
    HttpResponse::Ok().body(format!(
        "{} * {} = {}",
        numbers.first, numbers.second, product
    ))
}

#[get("/divide")]
async fn divide(numbers: web::Query<Numbers>) -> impl Responder {
    // TODO: Handle divide by zero
    let quotient = numbers.first / numbers.second;
    HttpResponse::Ok().body(format!(
        "{} / {} = {}",
        numbers.first, numbers.second, quotient
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let ip_address: &str = "0.0.0.0";
    let port_number: u16 = 8080;
    println!(
        "Starting calculator server at http://{}:{}",
        ip_address, port_number
    );
    HttpServer::new(|| {
        App::new()
            .service(health)
            .service(add)
            .service(subtract)
            .service(multiply)
            .service(divide)
    })
    .bind((ip_address, port_number))?
    .run()
    .await
}
