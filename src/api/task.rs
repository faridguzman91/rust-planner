se crate::model::task::Task;
use crate::model::task::{Task, TaskState};
use crate::repository::ddb::DDBRepository;
use actix_web::{
    get, 
    post, 
    put,
    error::ResponseError,
    web::Path,
    web::Json,
    web::Data,
    HttpResponse,
    http::{header::ContentType, StatusCode}
};
use serde::{Serialize, Deserialize};
use derive_more::Display;
//use std::fmt::{Display, Debug};

#[derive(Deserialize, Serialize)]
pub struct TaskIdentifier {
    task_global_id: String,
}

#[derive(Deserialize)]
pub struct TaskCompletionRequest {
    result_file: String
}

#[derive(Deserialize)]
pub struct SubmitTaskRequest {
    user_id: String,
    task_type: String,
    source_file: String
}

#[derive(Debug, Display)]
pub enum TaskError {
    TaskNotFound,
    TaskUpdateFailure,
    TaskCreationFailure,
    BadTaskRequest
}

impl ResponseError for TaskError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            TaskError::TaskNotFound => StatusCode::NOT_FOUND,
            TaskError::TaskUpdateFailure => StatusCode::FAILED_DEPENDENCY,
            TaskError::TaskCreationFailure => StatusCode::FAILED_DEPENDENCY,
            TaskError::BadTaskRequest => StatusCode::BAD_REQUEST
        }
    }
}

#[get("/task/{task_global_id}")]
pub async fn get_task(
    ddb_repo: Data<DDBRepository>,
    task_identifier: Path<TaskIdentifier>
) -> Result<Json<TaskIdentifier>, TaskError> {
    let task = Task::new(request.user_id.clone(), request.task_type.clone(), request.source_file.clone(),
    );

    dbg!(&task);

    let task_identifier = task.get_global_id();
    match ddb_repo.put_track(task).await {
        Ok(()) => Ok(Json(TaskIdentifier { task_global_id: task_identifier })),
        Err(_) => Err(TaskError::TaskCreationFailure)
    }
}

#[post("/task")]
pub async fn submit_task(
    ddb_repo: Data<DDBRepository>,
    request: Json<SubmitTaskRequest>
) -> Result<Json<TaskIdentifier>, TaskError> {
    let task = Task::new(
        request.user_id.clone(),
        request.task_type.clone(),
        request.source_file.clone()
    );

    dbg!(&task);

    let task_identifier = task.get_global_id();
    match ddb_repo.put_task(task).await {
        Ok(()) => Ok(Json(TaskIdentifier { task_global_id: task_identifier })),
        Err(_) => Err(TaskError::TaskCreationFailure)
    }
}

