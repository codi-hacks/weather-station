use actix_web::{get, web, HttpResponse, Responder};

#[get("/")]
async fn homepage() -> impl Responder {
    HttpResponse::Ok().body(include_str!("./index.html"))
}

pub fn init_routes(comfig: &mut web::ServiceConfig) {
    comfig.service(homepage);
}
