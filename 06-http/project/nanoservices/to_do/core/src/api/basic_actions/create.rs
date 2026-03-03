use crate::structs::ToDoItem;
#[cfg(feature = "json-file-storage")]
use to_do_dal::json_file::save_one;
use glue::errors::NanoServiceError;

pub async fn create(item: ToDoItem) -> Result<ToDoItem, NanoServiceError> {
    let _ = save_one(&item.title.to_string(), &item)?;
    Ok(item)
}
