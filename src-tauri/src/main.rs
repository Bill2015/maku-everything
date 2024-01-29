// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use tauri::api::process::Command;
use modules::common::repository;
use modules::{category, tag, subject, resource};

mod modules;
mod utils;


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
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
    }
    String::from("Connect")
}


fn main() {

    let (mut rx, mut child) = Command::new_sidecar("surreal")
        .expect("failed to create `surread db` binary command")
        .args(["start", "-A", "--user", "root", "--pass", "root"])
        .spawn()
        .expect("Failed to spawn server");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            connect_db, 
            category::application::create_category,
            category::application::update_category,
            category::application::import_category,
            category::application::export_category,
            category::application::get_all_category,
            category::application::get_category_by_id,
            subject::application::create_subject,
            subject::application::update_subject,
            subject::application::get_all_subject,
            subject::application::get_subject_by_id,
            subject::application::list_subjects,
            resource::application::create_resource,
            resource::application::update_resource,
            resource::application::get_all_resource,
            resource::application::get_resource_by_id,
            resource::application::explore_the_file,
            resource::application::add_resource_tag,
            resource::application::remove_resource_tag,
            resource::application::get_resource_detail,
            resource::application::list_resource,
            resource::application::querying_by_string,
            tag::application::create_tag,
            tag::application::update_tag,
            tag::application::get_all_tag,
            tag::application::get_tag_by_id,
            tag::application::list_tags,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

}
