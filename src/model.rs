use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[allow(non_snake_case)]
#[derive(Debug,Serialize, Deserialize, Clone)]
pub struct Todo{
    pub id: Option<String>,
    pub title: String,
    pub content: String,
    pub completed : Option<bool>,
    pub createdAt: Option<DateTime<Utc>>,
    pub updatedAt: Option<DateTime<Utc>>,
}

#[allow(non_snake_case)]
pub struct AppState{
    pub todoDB: Arc<Mutex<Vec<Todo>>>,
}

impl AppState{

    pub fn init() -> AppState{
        AppState{
            todoDB: Arc::new(Mutex::new(Vec::new()))
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateTodoSchema{
    pub title: Option<String>,
    pub content: Option<String>,
    pub completed: Option<bool>,
}
