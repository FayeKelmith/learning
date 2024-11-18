
pub mod response;
pub mod model;
pub mod handler;

use handler::{
    healthchecker,todos_list_handler,create_todo_handler,get_todo_handler, edit_todo_handler,delete_todo_handler
};

#[macro_use]
extern crate rocket;



#[launch]
fn rocket() -> _{
    let app_data = model::AppState::init();
    rocket::build().manage(app_data).mount("/api",
     routes![
        healthchecker,
        todos_list_handler,
        create_todo_handler,
        get_todo_handler,
        edit_todo_handler,
        delete_todo_handler
        ])
}