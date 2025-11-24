use rocket::{get, post, put, delete, serde::json::Json, State};
use crate::models::{Todo, CreateTodo, UpdateTodo};
use crate::db::DbPool;

#[get("/todos")]
pub async fn get_todos(pool: &State<DbPool>) -> Result<Json<Vec<Todo>>, String> {
    let todos = sqlx::query_as::<_, Todo>("SELECT id, title, completed FROM todos ORDER BY id")
        .fetch_all(pool.inner())
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(Json(todos))
}

#[post("/todos", data = "<todo>")]
pub async fn create_todo(pool: &State<DbPool>, todo: Json<CreateTodo>) -> Result<Json<Todo>, String> {
    let new_todo = sqlx::query_as::<_, Todo>(
        "INSERT INTO todos (title) VALUES ($1) RETURNING id, title, completed"
    )
    .bind(&todo.title)
    .fetch_one(pool.inner())
    .await
    .map_err(|e| e.to_string())?;

    Ok(Json(new_todo))
}

#[put("/todos/<id>", data = "<todo>")]
pub async fn update_todo(pool: &State<DbPool>, id: i32, todo: Json<UpdateTodo>) -> Result<Json<Todo>, String> {
    let mut query = String::from("UPDATE todos SET ");
    let mut i = 1;
    let mut args = Vec::new();

    if let Some(title) = &todo.title {
        query.push_str(&format!("title = ${}, ", i));
        args.push(title.clone()); // Clone to satisfy type checker for now, optimization later
        i += 1;
    }

    if let Some(completed) = todo.completed {
        query.push_str(&format!("completed = ${}, ", i));
        // We need to handle the boolean type. 
        // For simplicity in this dynamic query builder, let's just use a fixed query for now or a better builder.
        // Actually, let's keep it simple: always update both or just use a simpler logic.
    }
    
    // To avoid complex dynamic query building in this snippet, let's assume we patch whatever is sent.
    // But SQLx requires compile-time checking usually, or we use `sqlx::query`.
    
    // Let's rewrite to a simpler "toggle complete" or "update title" separate routes, OR just update everything.
    // For a simple CRUD, let's assume the user sends the full object or we just update specific fields.
    
    // Simplified approach: Fetch, update struct, save.
    let old_todo = sqlx::query_as::<_, Todo>("SELECT id, title, completed FROM todos WHERE id = $1")
        .bind(id)
        .fetch_optional(pool.inner())
        .await
        .map_err(|e| e.to_string())?;

    if let Some(existing) = old_todo {
        let new_title = todo.title.as_ref().unwrap_or(&existing.title);
        let new_completed = todo.completed.unwrap_or(existing.completed);

        let updated = sqlx::query_as::<_, Todo>(
            "UPDATE todos SET title = $1, completed = $2 WHERE id = $3 RETURNING id, title, completed"
        )
        .bind(new_title)
        .bind(new_completed)
        .bind(id)
        .fetch_one(pool.inner())
        .await
        .map_err(|e| e.to_string())?;

        Ok(Json(updated))
    } else {
        Err("Todo not found".to_string())
    }
}

#[delete("/todos/<id>")]
pub async fn delete_todo(pool: &State<DbPool>, id: i32) -> Result<String, String> {
    let result = sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(pool.inner())
        .await
        .map_err(|e| e.to_string())?;

    if result.rows_affected() == 0 {
        Err("Todo not found".to_string())
    } else {
        Ok("Deleted".to_string())
    }
}
