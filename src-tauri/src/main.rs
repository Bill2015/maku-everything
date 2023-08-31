// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use category::application::dto::CategoryResDto;
use category::application::dto::CategoryError;
use category::application;
use common::repository;
use category::application::service::{CATEGORY_SERVICE};

mod common;
mod resource;
mod category;
mod tag;
mod subject;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn db_test() -> String {
    format!("Hello, {}! You've been greeted from Rust!", "abc0")
}

#[tauri::command]
async fn connect_db() -> String {
    match repository::env::connent_db().await {
        Ok(value) => {
            println!("Connection DB: {}", value);
        }
        Err(err) => {
            println!("Connection DB Failed");
            println!("{}", err);
        },
        _ => {

        }
    }
    String::from("Connect")
}

#[tauri::command]
async fn db_test2() -> Vec<CategoryResDto> {

    CATEGORY_SERVICE.create_category(
        String::from("Hi"), 
        String::from("goodbye")
    ).await;

    let r: Result<Vec<CategoryResDto>, CategoryError> = CATEGORY_SERVICE.get_all_category().await;


    r.unwrap()
    // let resource_repo = &RESOURSE_REPOSITORY;

    // let new_resource = resource::ResourceAggregate::new(
    //     String::from("Test Title"), 
    //     String::from("Test Des"), 
    //     String::from("Test File Id"),
    //     String::from("Test File Name"),
    //     String::from("Test File Path"),
    //     String::from("Test File Type"),
    //     false);

    // let result = resource_repo.save(new_resource).await;

    // result.unwrap()
    

    // let category_repo = &CATEGORY_REPOSITORY;

    // let new_category = CategoryAggregate::new(
    //     String::from("測試分類"),
    //     String::from("我是描述"),
    // );

    // let category_res = category_repo.save(new_category).await;
    // let category_data = category_res.unwrap();
    // dbg!(&category_data);

    // let subject_repo = &SUBJECT_REPOSITORY;

    // let new_subject = SubjectAggregate::new(
    //     String::from("測試主題"),
    //     String::from("我是主題描述"),
    //     category_data.id,
    //     false,
    // );

    // let subject_res = subject_repo.save(new_subject).await;
    // dbg!(&subject_res);

    // let new_resource = ResourceAggregate::new(
    //     String::from("測試 Resource"),
    //     String::from("描述"),
    //     category_data.id,
    //     String::from("File Id"),
    //     String::from("File Name"),
    //     String::from("File PAth"),
    //     String::from("File Type"),
    //     false,
    // );

    // let resource_res = RESOURSE_REPOSITORY.save(new_resource).await;

    // let all_category = CATEGORY_QUERY_REPOSITORY.get_all().await;
    // dbg!(&resource_res);

    
    // resource_res.unwrap()

}

fn main() {

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet, 
            connect_db, 
            db_test2,
            category::application::create_category,
            category::application::update_category,
            category::application::get_all_category,
            category::application::get_category_by_id,
            subject::application::create_subject,
            subject::application::update_subject,
            subject::application::get_all_subject,
            subject::application::get_subject_by_id,
            resource::application::create_resource,
            resource::application::update_resource,
            resource::application::get_all_resource,
            resource::application::get_resource_by_id,
            resource::application::explore_the_file,
            tag::application::create_tag,
            tag::application::update_tag,
            tag::application::get_all_tag,
            tag::application::get_tag_by_id,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
