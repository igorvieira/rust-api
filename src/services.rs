use actix_web::{
    web::{
        scope,
        Json,
        Data,
        ServiceConfig,
        Query
    },
    get,
    post,
    HttpResponse,
    Responder
};

use serde_json::json;

use crate::{schema::{CreateTaskSchema, FilterOptions}, model::TaskModel, AppState};


#[get("/healthchecker")]
async fn health_checker() -> impl Responder {
    const MESSAGE: &str = "Health check: API is up and running smoothly.";

    HttpResponse::Ok().json(json!({
        "status": "success",
        "message": MESSAGE
    }))
}

#[post("/task")]
async fn create_task(
    body: Json<CreateTaskSchema>,
    data: Data<AppState>
) -> impl Responder {

    match 
        sqlx::query_as!(
            TaskModel,
            "INSERT INTO tasks (title, content) VALUES ($1, $2)
            RETURNING * ",
            body.title.to_string(),
            body.content.to_string()
        )
        .fetch_one(&data.db)
        .await {
            Ok(task) => {
                let note_response = json!({
                    "status": "success",
                    "task": json!({
                        "task": task,
                    })
                });

                return HttpResponse::Ok().json(note_response);
            }
            Err(error) => {

                return HttpResponse::InternalServerError().json(
                    json!({
                        "status": "error",
                        "message": format!("{:?}", error)
                    })
                )
            }

        }

}


#[get("/tasks")]
async fn get_all_tasks(
    opts: Query<FilterOptions>,
    data: Data<AppState>

) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1)- 1) * limit;


    match
        sqlx::query_as!(
        TaskModel,
            "SELECT * FROM tasks ORDER by id LIMIT $1 OFFSET $2",
            limit as i32,
            offset as i32,
        )
        .fetch_all(&data.db)    
        .await {
            Ok(tasks) => {
                let json_response = json!({
                    "status": "success",
                    "result":  tasks.len(),
                    "tasks": tasks
                });

                return HttpResponse::Ok().json(json_response);
            }
            Err(error) => {

                return HttpResponse::InternalServerError().json(
                    json!({
                        "status": "error",
                        "message": format!("{:?}", error)
                    })
                )
            }

        }

    
}

pub fn config(conf:  &mut ServiceConfig) {
    let scope = scope("/api")
            .service(health_checker)
            .service(create_task)
            .service(get_all_tasks);


    conf.service(scope);
}
