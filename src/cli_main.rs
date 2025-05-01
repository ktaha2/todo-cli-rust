use sqlx::{postgres::PgPoolOptions, Row};
use std::env;
use dotenv::dotenv;
use uuid::Uuid;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();  // load env variables from .env file

    // get the database URL from the environment variable
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // connect to postgres db 
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    println!("📋 Welcome to RustyTasks!\n");

    loop {
        println!("--- MENU ---");
        println!("1. View all tasks");
        println!("2. Add a new task");
        println!("3. Mark task as completed");
        println!("4. Delete all completed tasks");
        println!("5. Exit");

        print!("Choose an option: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("❌ Invalid input, please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                let rows = sqlx::query("SELECT id, title, completed FROM tasks")
                    .fetch_all(&pool)
                    .await?;

                println!("\n📄 All Tasks:");
                for row in rows {
                    let id: Uuid = row.get("id");
                    let title: String = row.get("title");
                    let completed: bool = row.get("completed");
                    println!("- [{}] {} ({})", if completed { "x" } else { " " }, title, id);
                }
                println!();
            }

            2 => {
                print!("Enter the task title: ");
                io::stdout().flush().unwrap();
                let mut title = String::new();
                io::stdin().read_line(&mut title).unwrap();

                let new_id = Uuid::new_v4();
                sqlx::query("INSERT INTO tasks (id, title, completed) VALUES ($1, $2, $3)")
                    .bind(new_id)
                    .bind(title.trim())
                    .bind(false)
                    .execute(&pool)
                    .await?;

                println!("📝 Inserted task: {} with ID: {}", title.trim(), new_id);
            }

            3 => {
                print!("Enter the task ID to mark as completed: ");
                io::stdout().flush().unwrap();
                let mut id_str = String::new();
                io::stdin().read_line(&mut id_str).unwrap();

                let id: Uuid = match id_str.trim().parse() {
                    Ok(id) => id,
                    Err(_) => {
                        println!("❌ Invalid UUID format.");
                        continue;
                    }
                };

                sqlx::query("UPDATE tasks SET completed = true WHERE id = $1")
                    .bind(id)
                    .execute(&pool)
                    .await?;

                println!("✅ Task with ID: {} marked as completed.", id);
            }

            4 => {
                let deleted = sqlx::query("DELETE FROM tasks WHERE completed = true")
                    .execute(&pool)
                    .await?
                    .rows_affected();

                println!("🗑️ Deleted {} completed task(s).", deleted);
            }

            5 => {
                println!("👋 Goodbye!");
                break;
            }

            _ => println!("❌ Invalid choice, please try again."),
        }
    }

    Ok(())
}
