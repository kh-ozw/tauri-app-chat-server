// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::fmt::format;
use std::sync::{Arc, Mutex};
use tauri::Manager;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn push_btn_1() {
    println!("push btn_1");
}

#[tauri::command]
fn command_with_message(message: String) -> String {
    format!("hello {}", message)
}

#[derive(Debug, Serialize, Deserialize)]
struct MyMessage {
    field_str: String,
    field_u32: u32,
}

#[tauri::command]
fn command_with_object(message: MyMessage) -> MyMessage {
    let MyMessage {
        field_str,
        field_u32,
    } = message;

    MyMessage {
        field_str: format!("hello {}", field_str),
        field_u32: field_u32 + 1,
    }
}

#[tauri::command]
fn command_with_error(arg: u32) -> Result<String, String> {
    if arg % 2 == 0 {
        Ok(format!("even value {}", arg))
    } else {
        Err(format!("odd value {}", arg))
    }
}

#[tauri::command]
fn command_with_async(arg: u32) -> String {
    "hello".into()
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            push_btn_1,
            command_with_message,
            command_with_object,
            command_with_error,
            command_with_async,
        ])
        .setup(|app| {
            let app_handle = app.handle();
            let addr = "127.0.0.1:8080";
            
            tauri::async_runtime::spawn(async move {
                let listener = TcpListener::bind(addr).await;
                match listener {
                    Ok(listener) => {
                        println!("start server: {}", addr);
                        // app_handle
                        //     .emit_all("back-to-front", "ping frontend".to_string())
                        //     .unwrap();
                        // std::thread::sleep(std::time::Duration::from_secs(5))
                        while let Ok((stream, _addr)) = listener.accept().await {
                            tauri::async_runtime::spawn(async { accept_connection(stream).await });
                        }
                    }
                    Err(e) => {
                        println!("{}", e);
                    }
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn accept_connection(mut stream: TcpStream) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    println!("addr: {}", addr);

    // ソケットを読み込み部と書き込み部に分割
    let (reader, mut writer) = stream.split();

    // 文字列への読み込み
    let mut buf_reader = BufReader::new(reader);
    let mut line = String::new();
    loop {
        match buf_reader.read_line(&mut line).await {
            Ok(bytes) => {
                if bytes == 0 {
                    println!("Close connection: {}", addr);
                    break;
                }
            }
            Err(e) => {
                println!("{e}");
                line = "Invalid UTF-8 detected\n".to_string();
            }
        }

        let line2 = line.trim();
        println!("{}", line2);

        // ソケットへの書き込み（クライアントへの返信）
        writer.write_all(line2.as_bytes()).await.unwrap();
        line.clear();
    }
}

// npm run tauri dev
// https://zenn.dev/kumassy/books/6e518fe09a86b2/viewer/1dbeeb

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
