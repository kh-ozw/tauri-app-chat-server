// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};
use tauri::Manager;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            let app_handle = app.handle();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// use std::sync::{Arc, Mutex};
// use tauri::Manager;
// use tokio::io::{AsyncReadExt, AsyncWriteExt};
// use tokio::net::{TcpListener, TcpStream};
// use tokio::sync::mpsc;

// #[tokio::main]
// async fn main() {
//     tauri::Builder::default()
//         .setup(|app| {
//             let app_handle = app.handle();
//             tokio::spawn(async move {
//                 let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
//                 println!("Server is running on 127.0.0.1:8080");

//                 let clients = Arc::new(Mutex::new(Vec::new()));

//                 loop {
//                     let (socket, _) = listener.accept().await.unwrap();
//                     let clients = clients.clone();
//                     let app_handle = app_handle.clone();
//                     tokio::spawn(async move {
//                         handle_client(socket, clients, app_handle).await;
//                     });
//                 }
//             });
//             Ok(())
//         })
//         .run(tauri::generate_context!())
//         .expect("error while running tauri application");
// }

// async fn handle_client(
//     socket: TcpStream,
//     clients: Arc<Mutex<Vec<mpsc::UnboundedSender<String>>>>,
//     app_handle: tauri::AppHandle,
// ) {
//     let (reader, mut writer) = tokio::io::split(socket);
//     let (tx, mut rx) = mpsc::unbounded_channel();

//     {
//         let mut clients = clients.lock().unwrap();
//         clients.push(tx);
//     }

//     let clients_clone = clients.clone();
//     tokio::spawn(async move {
//         let mut buf = vec![0; 1024];
//         let mut reader = tokio::io::BufReader::new(reader);
//         loop {
//             let n = match reader.read(&mut buf).await {
//                 Ok(n) if n == 0 => return,
//                 Ok(n) => n,
//                 Err(_) => return,
//             };

//             let received_data = String::from_utf8_lossy(&buf[..n]).to_string();
//             println!("Received: {}", received_data);

//             {
//                 let clients = clients_clone.lock().unwrap();
//                 for client in clients.iter() {
//                     client.send(received_data.clone()).unwrap();
//                 }
//             }

//             app_handle
//                 .emit_all("message-received", received_data.clone())
//                 .unwrap();
//         }
//     });

//     while let Some(msg) = rx.recv().await {
//         writer.write_all(msg.as_bytes()).await.unwrap();
//     }

//     {
//         let mut clients = clients.lock().unwrap();
//         clients.retain(|client| !client.is_closed());
//     }
// }
