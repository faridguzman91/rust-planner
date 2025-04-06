use actix_web::{
    error::ResponseError,
    get,
    http::{header::ContentType, StatusCode},
    post, put,
    web::Data,
    web::Json,
    web::Path,
    HttpResponse,
};

use derive_more::{Add, Display, From, Into};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TaskIndentifier {
    task_global_id: String,

}


#[get("/task/{task_global_id}")]
pub async fn get_task(task_identifier: Path<TaskIndentifier>, body: Json<Struct>) -> Json<String>{
    Json(task_identifier.into_inner().task_global_id);
    
}
