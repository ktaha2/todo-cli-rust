use actix_web::{get, post, put, delete, web, App, HttpServer, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use sqlx::{postgres::PgPoolOptions};
use std::env;
use dotenv::dotenv;
use uuid::Uuid;
use actix_cors::Cors;
use serde_json::json;


#[derive(Serialize)]
struct Task {
    id: Uuid,
    title: String,
    completed: bool,
}


#[derive(Deserialize)]
struct NewTask {
    title: String,
}

#[get("/")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("✅ RustyTasks API is running")
}

#[get("/tasks")]
async fn get_tasks(db: web::Data<sqlx::PgPool>) -> impl Responder {
    let rows = sqlx::query!("SELECT id, title, completed FROM tasks")
        .fetch_all(db.get_ref())
        .await;

    match rows {
        Ok(rows) => {
            let tasks: Vec<Task> = rows.into_iter().map(|row| Task {
                id: row.id,
                title: row.title,
                completed: row.completed,
            }).collect();

            HttpResponse::Ok().json(tasks)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/tasks")]
async fn add_task(
    db: web::Data<sqlx::PgPool>,
    form: web::Json<NewTask>,
) -> impl Responder {
    let new_id = Uuid::new_v4();
    let result = sqlx::query!(
        "INSERT INTO tasks (id, title, completed) VALUES ($1, $2, $3)",
        new_id,
        form.title,
        false
    )
    .execute(db.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Created().json(json!({ "status": "success" })),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}


#[put("/tasks/{id}/complete")]
async fn complete_task(
    db: web::Data<sqlx::PgPool>,
    path: web::Path<Uuid>
) -> impl Responder {
    let id = path.into_inner(); // ✅ this extracts the UUID
    let result = sqlx::query!(
        "UPDATE tasks SET completed = NOT completed WHERE id = $1",
        id
    )
    .execute(db.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json(json!({ "status": "success" })),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[delete("/tasks/completed")]
async fn delete_completed_tasks(
    db: web::Data<sqlx::PgPool>,
) -> impl Responder {
    let result = sqlx::query!(
        "DELETE FROM tasks WHERE completed = true"
    )
    .execute(db.get_ref())
    .await;

    match result {
        Ok(r) => HttpResponse::Ok().json(json!({
            "status": "deleted",
            "count": r.rows_affected()
        })),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}




#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("❌ Failed to connect to the database");

    println!("🚀 Starting server at http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive()) // ← this allows all origins during dev
            .app_data(web::Data::new(pool.clone()))
            .service(health_check)
            .service(get_tasks)
            .service(add_task)
            .service(complete_task)
            .service(delete_completed_tasks)

    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
