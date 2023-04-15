use crate::{
    model::TaskModel,
    schema::{ CreateTaskSchema, 
        //FilterOptions, 
        //UpdateTaskSchema 
    },
    AppState,
};
use actix_web::{ get, post, web::{
    Data,
    Json,
    scope,
    ServiceConfig
}, HttpResponse, Responder };
use serde_json::json;


#[get("/healthchecker")]
async fn  health_checker() -> impl Responder {
    const MESSAGE: &str = "The API is running smoothly.";
    HttpResponse::Ok().json(json!({"status": "success", "message": MESSAGE }))
}


#[post("/task")]
async fn create_task(
    body: Json<CreateTaskSchema>,
    data: Data<AppState>,
) -> impl Responder {
  match sqlx::query_as!(
        TaskModel,
        "INSERT INTO tasks (title, content) VALUES ($1, $2)
         RETURNING *
        ",
        body.title.to_string(),
        body.content.to_string()
    )
    .fetch_one(&data.db)
    .await {
        Ok(task) => {
            let note_response = serde_json::json!({
                "status": "success",
                "task": serde_json::json!({
                    "task": task
                })
            });

            return HttpResponse::Ok().json(note_response);
        }
        Err(error) => {
            return HttpResponse::InternalServerError()
                .json(serde_json::json!({"status": "error","message": format!("{:?}", error)}));
        }
    }

}


pub fn config(conf: &mut ServiceConfig) {
    let scope = scope("/api").service(health_checker).service(create_task);

    conf.service(scope);
}
