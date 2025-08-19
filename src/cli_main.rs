use sqlx::postgres::PgPoolOptions;
use std::io::{self, Write};
use uuid::Uuid;
use dotenv::dotenv;

use rusty_tasks_api::{
    config::Config,
    database::TaskService,
    models::NewTask
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Load configuration
    let config = Config::from_env()?;

    // Connect to database
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    // Create task service
    let task_service = TaskService::new(pool);

    println!("📋 Welcome to RustyTasks!\n");

    loop {
        display_menu();
        
        match get_user_choice() {
            Ok(choice) => {
                if let Err(e) = handle_menu_choice(choice, &task_service).await {
                    eprintln!("❌ Error: {}", e);
                }
            }
            Err(e) => {
                eprintln!("❌ {}", e);
                continue;
            }
        }
    }
}

fn display_menu() {
    println!("--- MENU ---");
    println!("1. View all tasks");
    println!("2. Add a new task");
    println!("3. Mark task as completed");
    println!("4. Delete all completed tasks");
    println!("5. Exit");
    print!("Choose an option: ");
    io::stdout().flush().unwrap();
}

fn get_user_choice() -> Result<u32, String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|e| format!("Failed to read input: {}", e))?;
    
    input.trim().parse().map_err(|_| "Invalid input, please enter a number.".to_string())
}

async fn handle_menu_choice(choice: u32, task_service: &TaskService) -> Result<(), Box<dyn std::error::Error>> {
    match choice {
        1 => view_all_tasks(task_service).await,
        2 => add_new_task(task_service).await,
        3 => mark_task_completed(task_service).await,
        4 => delete_completed_tasks(task_service).await,
        5 => {
            println!("👋 Goodbye!");
            std::process::exit(0);
        }
        _ => {
            println!("❌ Invalid choice, please try again.");
            Ok(())
        }
    }
}

async fn view_all_tasks(task_service: &TaskService) -> Result<(), Box<dyn std::error::Error>> {
    let tasks = task_service.get_all_tasks().await?;

    println!("\n📄 All Tasks:");
    if tasks.is_empty() {
        println!("No tasks found.");
    } else {
        for task in tasks {
            let status = if task.completed { "x" } else { " " };
            println!("- [{}] {} ({})", status, task.title, task.id);
        }
    }
    println!();
    Ok(())
}

async fn add_new_task(task_service: &TaskService) -> Result<(), Box<dyn std::error::Error>> {
    print!("Enter the task title: ");
    io::stdout().flush().unwrap();
    
    let mut title = String::new();
    io::stdin().read_line(&mut title)?;
    
    let title = title.trim();
    if title.is_empty() {
        println!("❌ Task title cannot be empty.");
        return Ok(());
    }

    let new_task = NewTask {
        title: title.to_string(),
    };

    let task = task_service.create_task(&new_task).await?;
    println!("📝 Created task: {} with ID: {}", task.title, task.id);
    Ok(())
}

async fn mark_task_completed(task_service: &TaskService) -> Result<(), Box<dyn std::error::Error>> {
    print!("Enter the task ID to mark as completed: ");
    io::stdout().flush().unwrap();
    
    let mut id_str = String::new();
    io::stdin().read_line(&mut id_str)?;

    let id: Uuid = id_str.trim().parse()
        .map_err(|_| "Invalid UUID format")?;

    task_service.complete_task(id).await?;
    println!("✅ Task with ID: {} marked as completed.", id);
    Ok(())
}

async fn delete_completed_tasks(task_service: &TaskService) -> Result<(), Box<dyn std::error::Error>> {
    let deleted = task_service.delete_completed_tasks().await?;
    println!("🗑️ Deleted {} completed task(s).", deleted);
    Ok(())
}
