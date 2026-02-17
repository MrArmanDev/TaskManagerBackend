use crate::dto::{CreateTaskDto, UpdateTaskDto};
use crate::errors::AppError;
use crate::models::Task;
use crate::repository::TaskRepository;

pub struct TaskService {
    task_repo: TaskRepository,
}

impl TaskService {
    pub fn new(task_repo: TaskRepository) -> Self {
        Self { task_repo }
    }

    pub async fn create_task(&self, task:CreateTaskDto, user_id: i32 ) -> Result<Task, AppError> {
        self.task_repo.create_task(task, user_id).await
    }

    pub async fn get_task(&self, user_id: i32) -> Result<Vec<Task>, AppError> {
        self.task_repo.get_task(user_id).await
    }

    pub async fn update_task(&self, id: i32, task: UpdateTaskDto) -> Result<Task, AppError> {
        self.task_repo.update_task(id, task).await
    }

    pub async fn delete_task(&self, id: i32, user_id: i32) -> Result<(), AppError> {
        self.task_repo.delete_task(id, user_id).await
    }
}