// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct MessageInfo {
    message: String,
    user: String,
}

#[tauri::command]
fn emit_all_message_info(message: MessageInfo) -> MessageInfo {
    message
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![emit_all_message_info,])
        .setup(|app| {
            let app_handle = app.handle();
            let addr = "127.0.0.1:8080";

            // TCP接続があるたび別スレッドで受け付け
            tauri::async_runtime::spawn(async move {
                let listener = TcpListener::bind(addr).await;
                match listener {
                    Ok(listener) => {
                        println!("start server: {}", addr);
                        while let Ok((stream, _addr)) = listener.accept().await {
                            let ah = app_handle.clone();
                            tauri::async_runtime::spawn(async {
                                accept_connection(stream, ah).await
                            });
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

// TCP通信の各処理
async fn accept_connection(mut stream: TcpStream, app_handle: AppHandle) {
    let addr = stream
        .peer_addr()
        .expect("connected streams should have a peer address");
    println!("addr: {}", addr);

    // ソケットを読み込み部と書き込み部に分割
    let (reader, mut writer) = stream.split();

    // 受信したら画面に表示する
    let mut buf_reader = BufReader::new(reader);
    let mut line = String::new();
    loop {
        match buf_reader.read_line(&mut line).await {
            Ok(_) => {}
            Err(e) => {
                println!("{e}");
                line = "Invalid UTF-8 detected\n".to_string();
            }
        }
        let message = if line.len() == 0 {
            "接続が切断されました。\n".to_string()
        } else {
            format!("{}", line.trim())
        };
        println!("message: {}: {}", addr, &message);

        let message_info: MessageInfo = MessageInfo {
            message: message.clone(),
            user: format!("{addr}"),
        };

        app_handle
            .emit_all("emit_all_message_info", &message_info)
            .expect("emit_all_error");

        if line.len() == 0 {
            break;
        } else {
            // ソケットへの書き込み（クライアントへの返信）
            writer
                .write_all(&message_info.message.as_bytes())
                .await
                .unwrap();
            line.clear();
        }
    }
}

// npm run tauri dev
// https://zenn.dev/kumassy/books/6e518fe09a86b2/viewer/1dbeeb
// https://zenn.dev/yongikim/articles/rust-chat-server-2
// https://rust-lang.github.io/async-book/07_workarounds/03_send_approximation.html
// https://programwiz.org/2022/05/16/tauri-state-variable/
