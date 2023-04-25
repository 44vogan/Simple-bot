// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use enigo::*;
// use std::fs;
use winapi::shared::windef::POINT;
use winapi::um::wingdi::GetPixel;
use winapi::um::winuser::{GetCursorPos, GetDC, ReleaseDC};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }

fn tap_key(key: char) {
    let mut enigo = Enigo::new();
    enigo.key_click(Key::Layout(key));
}
fn move_mouse_and_click(x: i32, y: i32, m: &str) {
    let mut enigo = Enigo::new();
    enigo.mouse_move_to(x, y);
    if m == "right" {
        enigo.mouse_click(MouseButton::Right);
    } else {
        enigo.mouse_click(MouseButton::Left);
    }
}
fn input_some(some_string: &str) {
    let mut enigo = Enigo::new();
    enigo.key_sequence(some_string);
}

fn cmd_to_action(cmd: &str) {
    let cmd_split = cmd.split(",,");
    let cmd_split: Vec<&str> = cmd_split.collect();
    let action_type = cmd_split[5];
    println!("{}", action_type);
    match action_type {
        "右键点击" => {
            let x: i32 = match cmd_split[6].parse::<i32>() {
                Ok(_s) => _s,
                Err(_err) => 0,
            };
            let y: i32 = match cmd_split[7].parse::<i32>() {
                Ok(_s) => _s,
                Err(_err) => 0,
            };
            println!("right_click x {},y {}", x, y);
            move_mouse_and_click(x, y, "right");
        }
        "左键点击" => {
            let x: i32 = match cmd_split[6].parse::<i32>() {
                Ok(_s) => _s,
                Err(_err) => 0,
            };
            let y: i32 = match cmd_split[7].parse::<i32>() {
                Ok(_s) => _s,
                Err(_err) => 0,
            };
            println!("left_click x {},y {}", x, y);
            move_mouse_and_click(x, y, "left");
        }
        "触发按键" => {
            let key: char = match cmd_split[8].trim().parse::<char>() {
                Ok(_s) => _s,
                Err(_err) => 'q',
            };
            println!("tap {}", key);
            tap_key(key);
        }
        "input" => {
            let words = cmd_split[9];
            println!("input_some {}", words);
            input_some(words);
        }
        // all other numbers
        _ => {
            println!("没有找到对应的action_type")
        }
    }
}

#[tauri::command]
fn get_pixel_color(x: i32, y: i32) -> (u8, u8, u8) {
    //根据给的x，y坐标，获取rgb颜色
    println!("get pixel color of {},{}", x, y);
    unsafe {
        let hdc = GetDC(std::ptr::null_mut());
        let pixel = GetPixel(hdc, x, y);
        ReleaseDC(std::ptr::null_mut(), hdc);
        let r = (pixel & 0x0000ff) as u8;
        let g = ((pixel & 0x00ff00) >> 8) as u8;
        let b = ((pixel & 0xff0000) >> 16) as u8;
        println!("RGB: ({}, {}, {})", r, g, b);
        (r, g, b)
    }
}

fn match_pixel_color(cmd: &str) -> bool {
    let cmd_split = cmd.split(",,");
    let cmd_split: Vec<&str> = cmd_split.collect();
    let x = match cmd_split[0].trim().parse::<i32>() {
        Ok(_s) => _s,
        Err(_err) => 0,
    };
    let y: i32 = match cmd_split[1].trim().parse::<i32>() {
        Ok(_s) => _s,
        Err(_err) => 0,
    };
    // let r: u8 = cmd_split[2].parse().expect("r 颜色转换u8失败");
    let r = match cmd_split[2].trim().parse::<u8>() {
        Ok(_s) => _s,
        Err(_err) => 0,
    };
    let g = match cmd_split[3].trim().parse::<u8>() {
        Ok(_s) => _s,
        Err(_err) => 0,
    };
    let b = match cmd_split[4].trim().parse::<u8>() {
        Ok(_s) => _s,
        Err(_err) => 0,
    };
    let pixel_color = get_pixel_color(x, y);
    let color_tuple = (r, g, b);
    let color_match = pixel_color == color_tuple;
    println!("color_match {}", color_match);
    return color_match;
}

#[tauri::command]
fn get_cursor_position() -> (i32, i32) {
    //获取鼠标的坐标
    let mut point = POINT { x: 0, y: 0 };
    unsafe { GetCursorPos(&mut point) };
    println!("Cursor position: ({}, {})", point.x, point.y);
    (point.x, point.y)
}

#[tauri::command]
fn text_to_commands_to_actions(text: String) -> Result<String, String> {
    println!("text:{}", text);
    if text.len() < 6 {
        return Ok("规则有问题".into());
    }
    let user_commands = text.split("\r\n");
    let user_commands: Vec<&str> = user_commands.collect();
    // dbg!(user_commands);
    println!("vec commands:{:?}", user_commands);
    for cmd in user_commands {
        if match_pixel_color(cmd) {
            cmd_to_action(cmd);
        }
    }
    // If something fails
    // Err("This failed!".into())
    // If it worked
    Ok("This worked!".into())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            // greet,
            text_to_commands_to_actions,
            get_cursor_position,
            get_pixel_color
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
