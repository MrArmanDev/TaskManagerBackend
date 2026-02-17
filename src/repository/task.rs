use crate::dto::UpdateTaskDto;
use crate::{dto::CreateTaskDto, errors::AppError};
use crate::models::Task;
use sqlx::{PgPool, FromRow};

#[derive(Debug, FromRow)]
pub struct TaskRepository {
    pub pool: PgPool,
}

impl TaskRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_task(&self, task: CreateTaskDto, user_id: i32) -> Result<Task, AppError> {
        let task = sqlx::query_as::<_, Task>(
            "INSERT INTO tasks (user_id, title, description, status) VALUES ($1, $2, $3, $4) RETURNING id, user_id, title, description, status, created_at, updated_at"
        )
        .bind(user_id)
        .bind(task.title)
        .bind(task.description)
        .bind(task.status)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::DatabaseError)?;
        Ok(task)
    }

    pub async fn get_task(&self, user_id: i32) -> Result<Vec<Task>, AppError> {
        let tasks = sqlx::query_as::<_, Task>(
            "SELECT id, user_id, title, description, status, created_at, updated_at 
             FROM tasks 
             WHERE user_id = $1 
             ORDER BY created_at DESC" 
        )
        .bind(user_id)
        .fetch_all(&self.pool) 
        .await
        .map_err(AppError::DatabaseError)?;
    
        Ok(tasks)
    }

    pub async fn update_task(&self, user_id: i32, task: UpdateTaskDto) -> Result<Task, AppError> {
        let result = sqlx::query_as::<_, Task>(
            "UPDATE tasks 
             SET title = $1, description = $2, status = $3, updated_at = NOW() 
             WHERE id = $4 AND user_id = $5 
             RETURNING id, user_id, title, description, status, created_at, updated_at"
        )
        .bind(&task.title)
        .bind(&task.description)
        .bind(&task.status)
        .bind(task.id) 
        .bind(user_id) 
        .fetch_optional(&self.pool) 
        .await
        .map_err(AppError::DatabaseError)?;
    
        result.ok_or_else(|| AppError::NotFound("task is not found or you are not the owner of the task".to_string()))
    }

    pub async fn delete_task(&self, id: i32, user_id: i32) -> Result<(), AppError> {
        let result = sqlx::query("DELETE FROM tasks WHERE id = $1 AND user_id = $2")
            .bind(id)
            .bind(user_id)
            .execute(&self.pool)
            .await
            .map_err(AppError::DatabaseError)?;
    
        
        if result.rows_affected() == 0 {
            return Err(AppError::NotFound("task is not found or you are not the owner of the task".to_string()));
        }
    
        Ok(())
    }
}