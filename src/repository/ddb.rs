use crate::model::task::{Task, TaskState};
use aws_config::Config;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use log::error;
use std::collections::HashMap;
use std::str::FromStr;

pub struct DDBRepository {
    client: Client,
    table_name: String,
}

pub struct DDBError;

fn required_item_value(
    key: &str,
    item: &HashMap<String, AttributeValue>,
) -> Result<String, DDBError> {
    match item_value(key, item) {
        Ok(Some(value)) => Ok(value),
        Ok(None) => Err(DDBError),
        Err(DDBError) => Err(DDBError),
    }
}

fn item_value(
    key: &str,
    item: &HashMap<String, AttributeValue>,
) -> Result<Option<String>, DDBError> {
    match item.get(key) {
        Some(value) => match value.as_s() {
            Ok(val) => Ok(Some(val.clone())),
            Err(_) => Err(DDBError),
        },
        None => Ok(None),
    }
}

fn item_to_task(item: &HashMap<String, AttributeValue>) -> Result<Task, DDBError> {
    let state: TaskState = match TaskState::from_str(required_item_value("state", item)?.as_str()) {
        Ok(value) => value,
        Err(_) => return Err(DDBError)
    };

    let result_file = item_value("result_file", item)?;

    Ok(Task {
        user_uuid: required_item_value("pK", item)?,
        task_uuid: required_item_value("sK", item)?,
        task_type: required_item_value("task_type")?,
        state,
        source_file: required_item_value("source_file", item)?,
        result_file
    })
}
