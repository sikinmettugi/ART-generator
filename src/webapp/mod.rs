use actix_web::{ Error, HttpRequest, HttpResponse, Responder, get };

use crate::collections::{ ItunesSearchRes, ItunesCollectionInfo };

#[get("/hello")]
fn get_review_template() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}
