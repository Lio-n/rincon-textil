// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use rincon_textil_lib;

#[tokio::main]
async fn main() {
    // Llama a la función asincrónica `run` y usa await
    rincon_textil_lib::run().await;
}
