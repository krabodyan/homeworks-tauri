use chrono::NaiveDate;
use sqlx::migrate::MigrateDatabase;
use sqlx::prelude::FromRow;
use sqlx::{Sqlite, SqlitePool};
use tauri::async_runtime::Mutex;
use tauri::{Manager, State};

#[derive(serde::Serialize, FromRow)]
struct Task {
	id: i64,
	done: bool,
	title: String,
	deadline: NaiveDate,
}

#[tauri::command]
async fn load(
	state: State<'_, Mutex<SqlitePool>>,
) -> Result<Vec<Task>, String> {
	sqlx::query_as!(Task, "SELECT * FROM homeworks")
		.fetch_all(&*state.lock().await)
		.await
		.map_err(|err| err.to_string())
}

#[tauri::command]
async fn add_task(
	title: String,
	state: State<'_, Mutex<SqlitePool>>,
) -> Result<i64, String> {
	let now = chrono::Local::now().date_naive();
	sqlx::query_scalar!("INSERT INTO homeworks (done, title, deadline) VALUES ($1, $2, $3) RETURNING id", false, title, now)
		.fetch_one(&*state.lock().await)
		.await.map_err(|err| err.to_string())
}

#[tauri::command]
async fn delete_task(
	id: i64,
	state: State<'_, Mutex<SqlitePool>>,
) -> Result<(), String> {
	sqlx::query!("DELETE FROM homeworks WHERE id = $1", id)
		.execute(&*state.lock().await)
		.await
		.map_err(|e| e.to_string())?;
	Ok(())
}

#[tauri::command]
async fn update_task(
	id: i64,
	title: String,
	deadline: NaiveDate,
	state: State<'_, Mutex<SqlitePool>>,
) -> Result<(), String> {
	sqlx::query!(
		"UPDATE homeworks SET title = $1, deadline = $2 WHERE id = $3",
		title,
		deadline,
		id
	)
	.execute(&*state.lock().await)
	.await
	.map_err(|e| e.to_string())?;
	Ok(())
}

#[tauri::command]
async fn toggle_task(
	id: i64,
	state: State<'_, Mutex<SqlitePool>>,
) -> Result<(), String> {
	sqlx::query!(
		"UPDATE homeworks SET done = NOT done WHERE id = $1",
		id
	)
	.execute(&*state.lock().await)
	.await
	.map_err(|e| e.to_string())?;
	Ok(())
}

fn setup(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
	let mut path = app.path().app_data_dir()?;
	if !path.exists() {
		std::fs::create_dir(&path).expect("failed to create dir");
	}
	path.push("db.sqlite");

	let addr = format!(
		"sqlite://{}",
		path.to_str().expect("path should be something")
	);
	let pool = tauri::async_runtime::block_on(async move {
		Sqlite::create_database(&addr)
			.await
			.expect("failed to create database");

		let pool = sqlx::SqlitePool::connect(&addr)
			.await
			.expect("failed to connect");

		sqlx::migrate!("./migrations")
			.run(&pool)
			.await
			.expect("failed to migrate");

		pool
	});
	app.manage(Mutex::new(pool));
	Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
	tauri::Builder::default()
		.setup(setup)
		.invoke_handler(tauri::generate_handler![
			load,
			add_task,
			delete_task,
			toggle_task,
			update_task
		])
		.run(tauri::generate_context!())
		.expect("error while running tauri application");
}
