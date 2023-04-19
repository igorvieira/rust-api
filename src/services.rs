use crate::{
    model::TaskModel,
    schema::{ CreateTaskSchema, FilterOptions, UpdateTaskSchema },
    AppState,
};
use actix_web::{
    get,
    post,
    web::{ Data, Json, scope, Query, Path, ServiceConfig },
    HttpResponse,
    Responder,
    patch,
    delete,
};
use serde_json::json;

#[get("/healthchecker")]
async fn health_checker() -> impl Responder {
    const MESSAGE: &str = "The API is running smoothly.";
    HttpResponse::Ok().json(json!({"status": "success", "message": MESSAGE }))
}

#[post("/task")]
async fn create_task(body: Json<CreateTaskSchema>, data: Data<AppState>) -> impl Responder {
    match
        sqlx
            ::query_as!(
                TaskModel,
                "INSERT INTO tasks (title, content) VALUES ($1, $2)
         RETURNING *
        ",
                body.title.to_string(),
                body.content.to_string()
            )
            .fetch_one(&data.db).await
    {
        Ok(task) => {
            let note_response =
                serde_json::json!({
                "status": "success",
                "task": serde_json::json!({
                    "task": task
                })
            });

            return HttpResponse::Ok().json(note_response);
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(
                serde_json::json!({"status": "error","message": format!("{:?}", error)})
            );
        }
    }
}

#[get("/tasks")]
pub async fn get_all_tasks(opts: Query<FilterOptions>, data: Data<AppState>) -> impl Responder {
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    match
        sqlx
            ::query_as!(
                TaskModel,
                "SELECT * FROM tasks ORDER by id LIMIT $1 OFFSET $2",
                limit as i32,
                offset as i32
            )
            .fetch_all(&data.db).await
    {
        Ok(tasks) => {
            let json_response =
                serde_json::json!({
                "status": "success",
                "result": tasks.len(),
                "tasks": tasks
            });

            HttpResponse::Ok().json(json_response)
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(
                serde_json::json!({"status": "error","message": format!("{:?}", error)})
            );
        }
    }
}

#[get("/tasks/{id}")]
async fn get_task_by_id(path: Path<uuid::Uuid>, data: Data<AppState>) -> impl Responder {
    let task_id = path.into_inner();

    let query_result = sqlx
        ::query_as!(TaskModel, "SELECT * FROM tasks WHERE id = $1", task_id)
        .fetch_one(&data.db).await;

    match query_result {
        Ok(task) => {
            let note_response =
                serde_json::json!({
                "status": "success",
                "data": serde_json::json!({
                    "task": task
                })
            });

            HttpResponse::Ok().json(note_response)
        }
        Err(error) => {
            return HttpResponse::InternalServerError().json(
                serde_json::json!({"status": "error","message": format!("{:?}", error)})
            );
        }
    }
}

#[patch("/tasks/{id}")]
async fn edit_task_by_id(
    path: Path<uuid::Uuid>,
    body: Json<UpdateTaskSchema>,
    data: Data<AppState>
) -> impl Responder {
    let task_id = path.into_inner();

    match
        sqlx
            ::query_as!(TaskModel, "SELECT * FROM tasks WHERE id = $1", task_id)
            .fetch_one(&data.db).await
    {
        Ok(task) => {
            match
                sqlx
                    ::query_as!(
                        TaskModel,
                        "UPDATE tasks SET title = $1, content = $2 WHERE id = $3 RETURNING *",
                        body.title.to_owned().unwrap_or(task.title),
                        body.title.to_owned().unwrap_or(task.content),
                        task_id
                    )
                    .fetch_one(&data.db).await
            {
                Ok(task) => {
                    let task_response =
                        serde_json::json!({"status": "success","data": serde_json::json!({
                        "task": task
                    })});

                    return HttpResponse::Ok().json(task_response);
                }
                Err(err) => {
                    let message = format!("Error: {:?}", err);
                    return HttpResponse::InternalServerError().json(
                        serde_json::json!({
                            "status": "error",
                            "message": message
                        })
                    );
                }
            }
        }
        Err(err) => {
            let message = format!("Internal server error: {:?}", err);
            return HttpResponse::NotFound().json(
                serde_json::json!({"status": "fail","message": message})
            );
        }
    }
}

#[delete("/tasks/{id}")]
async fn delete_task_by_id(path: Path<uuid::Uuid>, data: Data<AppState>) -> impl Responder {
    let task_id = path.into_inner();

    match sqlx::query!("DELETE FROM tasks WHERE id = $1", task_id).execute(&data.db).await {
        Ok(_) => { HttpResponse::NoContent().finish() }
        Err(err) => {
            let message = format!("Internal server error: {:?}", err);
            return HttpResponse::NotFound().json(
                serde_json::json!({"status": "fail","message": message})
            );
        }
    }
}

pub fn config(conf: &mut ServiceConfig) {
    let scope = scope("/api")
        .service(health_checker)
        .service(create_task)
        .service(get_all_tasks)
        .service(get_task_by_id)
        .service(edit_task_by_id)
        .service(delete_task_by_id);

    conf.service(scope);
}
