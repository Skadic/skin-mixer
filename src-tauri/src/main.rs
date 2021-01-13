#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use elem_type::ElemType;
use err::InvalidElemTypeError;
use tauri::Webview;
use cmd::{ChooseSkinElementResponse, Cmd};

mod cmd;
mod elem_type;
mod skinutils;
mod err;

#[allow(unused)]
mod globals;

fn main() {
    tauri::AppBuilder::new()
        .invoke_handler(|webview, arg| {
            match serde_json::from_str(arg) {
                Err(e) => {
                    println!("{}", e.to_string());
                    Err(e.to_string())
                }
                Ok(command) => handle_commands(command, webview, arg)
            }
        })
        .build()
        .run();
}

fn handle_commands(command: Cmd, webview: &mut Webview, _arg: &str) -> Result<(), String> {
    use cmd::Cmd::*;
    match command {
        // definitions for your custom commands from Cmd here
        ChooseSkinElement { elem_type, callback, error} => {
            //  your command code

            tauri::execute_promise(
                webview,
                move || {
                    let elem = ElemType::from_name(&elem_type).ok_or(InvalidElemTypeError::new(&elem_type))?;
                    let map =  skinutils::skin_elements_by_type(elem).expect("Error getting skin elements");
                    println!("Sending map with {} elements", map.len());
                    return Ok(ChooseSkinElementResponse::new(map));
                },
                callback,
                error,
            )
        }
    }
    Ok(())
}