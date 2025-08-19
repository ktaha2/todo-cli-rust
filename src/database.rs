use crate::models::{Task, NewTask};
use crate::error::AppError;
use sqlx::{PgPool, Row};
use uuid::Uuid;

#[derive(Clone)]
pub struct TaskService {
    pool: PgPool,
}

impl TaskService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_all_tasks(&self) -> Result<Vec<Task>, AppError> {
        let rows = sqlx::query("SELECT id, title, completed FROM tasks")
            .fetch_all(&self.pool)
            .await?;

        let tasks = rows
            .into_iter()
            .map(|row| Task {
                id: row.get("id"),
                title: row.get("title"),
                completed: row.get("completed"),
            })
            .collect();

        Ok(tasks)
    }

    pub async fn create_task(&self, new_task: &NewTask) -> Result<Task, AppError> {
        let task_id = Uuid::new_v4();
        
        sqlx::query("INSERT INTO tasks (id, title, completed) VALUES ($1, $2, $3)")
            .bind(task_id)
            .bind(&new_task.title)
            .bind(false)
            .execute(&self.pool)
            .await?;

        Ok(Task {
            id: task_id,
            title: new_task.title.clone(),
            completed: false,
        })
    }

    pub async fn toggle_task(&self, task_id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query("UPDATE tasks SET completed = NOT completed WHERE id = $1")
            .bind(task_id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Task with id {} not found", task_id)));
        }

        Ok(())
    }

    pub async fn complete_task(&self, task_id: Uuid) -> Result<(), AppError> {
        let result = sqlx::query("UPDATE tasks SET completed = true WHERE id = $1")
            .bind(task_id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(AppError::NotFound(format!("Task with id {} not found", task_id)));
        }

        Ok(())
    }

    pub async fn delete_completed_tasks(&self) -> Result<u64, AppError> {
        let result = sqlx::query("DELETE FROM tasks WHERE completed = true")
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }

    pub async fn get_task_by_id(&self, task_id: Uuid) -> Result<Task, AppError> {
        let row = sqlx::query("SELECT id, title, completed FROM tasks WHERE id = $1")
            .bind(task_id)
            .fetch_optional(&self.pool)
            .await?;

        match row {
            Some(row) => Ok(Task {
                id: row.get("id"),
                title: row.get("title"),
                completed: row.get("completed"),
            }),
            None => Err(AppError::NotFound(format!("Task with id {} not found", task_id))),
        }
    }
}