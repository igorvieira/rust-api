use crate:: { AppState };
use actix_web::{ get, post, web::{ Data, Json, ServiceConfig, scope }, HttpResponse, Responder };
use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use serde_json::json;
use sqlx::{ self, FromRow };


#[derive(Deserialize)]
struct CreateTaskBody {
    title: String,
    content: String,
}

#[derive(Serialize, FromRow)]
struct Task {
    title: String,
    content: String,
    created_at: Option<NaiveDateTime>,
}


#[get("/healthchecker")]
async fn  health_checker() -> impl Responder {
    const MESSAGE: &str = "The API is running smoothly.";
    HttpResponse::Ok().json(json!({"status": "success", "message": MESSAGE }))
}

#[post("/task")]
async fn create_task(
    state: Data<AppState>,
    body: Json<CreateTaskBody>
) -> impl Responder {
    let task: CreateTaskBody = body.into_inner();


    match 
        sqlx
            ::query_as::<_, Task>(
                "INSERT INTO tasks (title, content)
                VALUES ($1, $2)
                RETURNING id, title, content, created_at
                "
            )
            .bind(task.title)
            .bind(task.content)
            .fetch_one(&state.db).await 
            {
                Ok(tasks) => HttpResponse::Ok().json(tasks),
                Err(error) => HttpResponse::InternalServerError().json(format!("{:?}", error)),
            }
            
}


pub fn config(conf: &mut ServiceConfig) {
    let scope = scope("/api").service(health_checker).service(create_task);

    conf.service(scope);
}
