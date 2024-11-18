use crate::{
    model::{AppState, Todo, UpdateTodoSchema},
    response::{ GenericResponse, SingleTodoResponse, TodoData, TodoListResponse},
};
use chrono::prelude::*;
use rocket::{ delete, get, http::Status, patch, post, response::status::Custom, serde::json::Json, State};
use uuid::Uuid;


#[get("/healthchecker")]
pub async fn healthchecker() -> Result<Json<GenericResponse>, Status>{
    const MESSAGE : &str = "Building simple API with Rust and Rocket";

    let response = GenericResponse{
        status: "success".to_string(),
        message: MESSAGE.to_string()
    };

    Ok(Json(response))
}


#[get("/todos?<page>&<limit>")]
pub async fn todos_list_handler(page: Option<usize>,
    limit: Option<usize>,
    data: &State<AppState>) -> Result<Json<TodoListResponse>,Status>{
        let vec = data.todoDB.lock().unwrap();

        let limit = limit.unwrap_or(10);
        let offset = (page.unwrap_or(1) -1)*limit;

        let todos : Vec<Todo>  = vec.clone().into_iter().skip(offset).take(limit).collect();

        let response : TodoListResponse = TodoListResponse{
            status : "success".to_string(),
            results: todos.len(),
            todos,
        };

        Ok(Json(response))

    }

    #[post("/todos", data = "<body>")]
    pub async fn create_todo_handler(
        mut body: Json<Todo>,
        data: &State<AppState>,
    ) -> Result<Json<SingleTodoResponse>,Custom<Json<GenericResponse>>>{

        let mut vec = data.todoDB.lock().unwrap();

        for todo in vec.iter(){
            if todo.title == body.title{
                let error_response = GenericResponse{
                    status: "error".to_string(),
                    message: "Title already exists".to_string(),
                };

                return Err(Custom(Status::Conflict, Json(error_response)));
            }
        }

        let uuid_id = Uuid::new_v4();
        let datetime = Utc::now();


        body.id = Some(uuid_id.to_string());
        body.createdAt = Some(datetime);
        body.updatedAt = Some(datetime);
        body.completed = Some(false);

        let todo = body.to_owned();

        vec.push(body.into_inner());

        let response = SingleTodoResponse{
            status: "success".to_string(),
            data: TodoData{
                data: todo.into_inner(),
            },
        };

        Ok(Json(response))

    }


    #[get("/todos/<id>")]
    pub async fn get_todo_handler(id: String, data: &State<AppState>) -> Result<Json<SingleTodoResponse>, Custom<Json<GenericResponse>>>{

        let vec = data.todoDB.lock().unwrap();

        for todo in vec.iter(){
            if todo.id == Some(id.to_owned()){
                let response = SingleTodoResponse{
                    status : "success".to_string(),
                    data: TodoData{
                        data: todo.clone(),
                    }
                };

                return Ok(Json(response));
            }
        }

        let error_response = GenericResponse{
            status: "error".to_string(),
            message: "Todo not found".to_string(),
        };

        Err(Custom(Status::NotFound, Json(error_response)))
    }


#[patch("/todos/<id>", data = "<body>")]
pub async fn edit_todo_handler(
    id: String,
    body: Json<UpdateTodoSchema>,
    data: &State<AppState>
) -> Result<Json<SingleTodoResponse>, Custom<Json<GenericResponse>>> {

    let mut vec = data.todoDB.lock().unwrap();

    for todo in vec.iter_mut(){

        if todo.id == Some(id.clone()){
            let datetime = Utc::now();
            let title = body.title.to_owned().unwrap_or(todo.title.to_owned());
            let content = body.content.to_owned().unwrap_or(todo.content.to_owned());

            let payload = Todo{
                id : todo.id.to_owned(),
                title : if title.is_empty(){
                    title
                }else{
                    todo.title.to_owned()
                },
                content : if content.is_empty(){
                    content
                }else{
                    todo.content.to_owned()
                },
                completed : if body.completed.is_some(){
                    body.completed
                }else{
                    todo.completed
                },
                createdAt : todo.createdAt,
                updatedAt: Some(datetime)
            };

            *todo = payload;

            let response = SingleTodoResponse{
                status : "success".to_string(),
                data : TodoData{
                    data: todo.clone()
                }
            };

            return Ok(Json(response));
        }
    }

    let error_response = GenericResponse{
        status : "error".to_string(),
        message : "Todo not found".to_string(),
    };

    Err(Custom(Status::NotFound, Json(error_response)))
}


#[delete("/todos/<id>")]
pub async fn delete_todo_handler(
    id: String,
    data : &State<AppState>
) -> Result<Status, Custom<Json<GenericResponse>>>{

    let mut vec = data.todoDB.lock().unwrap();

    for todo in vec.iter_mut(){
        if todo.id == Some(id.clone()){
            vec.retain(|todo| todo.id != Some(id.to_owned()));
            return Ok(Status::NoContent);
        }
    }

    let error_response = GenericResponse{
        status: "fail".to_string(),
        message: format!("Todo with id: {id} not found")
    };

    return Err(Custom(Status::NotFound, Json(error_response)));
}

