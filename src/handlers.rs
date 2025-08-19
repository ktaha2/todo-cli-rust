use actix_web::{get, post, put, delete, web, HttpResponse, Responder, Result};
use uuid::Uuid;
use serde_json::json;

use rusty_tasks_api::{
    models::{NewTask, TaskResponse, DeleteResponse},
    database::TaskService,
    error::AppError
};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(health_check)
        .service(get_tasks)
        .service(add_task)
        .service(complete_task)
        .service(delete_completed_tasks);
}

#[get("/")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "healthy",
        "message": "✅ RustyTasks API is running"
    }))
}

#[get("/tasks")]
async fn get_tasks(task_service: web::Data<TaskService>) -> Result<impl Responder, AppError> {
    let tasks = task_service.get_all_tasks().await?;
    Ok(HttpResponse::Ok().json(tasks))
}

#[post("/tasks")]
async fn add_task(
    task_service: web::Data<TaskService>,
    form: web::Json<NewTask>,
) -> Result<impl Responder, AppError> {
    if form.title.trim().is_empty() {
        return Err(AppError::InvalidInput("Task title cannot be empty".to_string()));
    }

    let task = task_service.create_task(&form).await?;
    let response = TaskResponse {
        status: "success".to_string(),
        task: Some(task),
    };
    
    Ok(HttpResponse::Created().json(response))
}

#[put("/tasks/{id}/complete")]
async fn complete_task(
    task_service: web::Data<TaskService>,
    path: web::Path<Uuid>
) -> Result<impl Responder, AppError> {
    let task_id = path.into_inner();
    task_service.toggle_task(task_id).await?;
    
    let response = TaskResponse {
        status: "success".to_string(),
        task: None,
    };
    
    Ok(HttpResponse::Ok().json(response))
}

#[delete("/tasks/completed")]
async fn delete_completed_tasks(
    task_service: web::Data<TaskService>,
) -> Result<impl Responder, AppError> {
    let deleted_count = task_service.delete_completed_tasks().await?;
    
    let response = DeleteResponse {
        status: "deleted".to_string(),
        count: deleted_count,
    };
    
    Ok(HttpResponse::Ok().json(response))
}