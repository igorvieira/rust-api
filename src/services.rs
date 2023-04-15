use crate::{
    model::TaskModel,
    schema::{ CreateTaskSchema, FilterOptions, 
        //UpdateTaskSchema 
    },
    AppState,
};
use actix_web::{ get, post, web::{
    Data,
    Json,
    scope,
    Query,
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

#[get("/tasks")]
pub async fn get_all_tasks(
    opts: Query<FilterOptions>,
    data: Data<AppState>,
) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let query_result = sqlx::query_as!(
        TaskModel,
        "SELECT * FROM tasks ORDER by id LIMIT $1 OFFSET $2",
        limit as i32,
        offset as i32
        
    )
    .fetch_all(&data.db)
    .await;


    if query_result.is_err() {
        let error_message = "Something bad happened while fetching all note tasks";

        return HttpResponse::InternalServerError()
            .json(json!({ "status": "error", "message": error_message }));
    }

    let tasks = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "result": tasks.len(),
        "tasks": tasks
    });

    HttpResponse::Ok().json(json_response)
}

pub fn config(conf: &mut ServiceConfig) {
    let scope = scope("/api")
        .service(health_checker)
        .service(create_task)
        .service(get_all_tasks);

    conf.service(scope);
}
