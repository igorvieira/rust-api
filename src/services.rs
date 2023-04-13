use actix_web::{ get, web, HttpResponse, Responder };
use serde_json::json;


#[get("/healthchecker")]
async fn  health_checker() -> impl Responder {
    const MESSAGE: &str = "The API is running smoothly.";
    HttpResponse::Ok().json(json!({"status": "success", "message": MESSAGE }))
}


pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api").service(health_checker);

    conf.service(scope);
}
