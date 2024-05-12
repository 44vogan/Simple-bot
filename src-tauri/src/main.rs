// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use enigo::*;
use tauri::Manager;
//use at windows
#[cfg(windows)]
use winapi::um::wingdi::GetPixel;
#[cfg(windows)]
use winapi::um::winuser::{GetDC, ReleaseDC};

//use at mac
#[cfg(target_os = "macos")]
use image::GenericImageView; // to allow calling .pixels()
#[cfg(target_os = "macos")]
use screenshots::Screen; // import the Screen type
#[cfg(target_os = "macos")]
extern crate screenshots;
#[cfg(target_os = "macos")]
use std::error::Error;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[derive(Clone, serde::Serialize)]
struct Payload {
    args: Vec<String>,
    cwd: String,
}

#[tauri::command]
fn longpress_keydown(key: &str) -> Result<String, String> {
    let mut enigo = Enigo::new();
    match key {
        "Space" => {
            enigo.key_down(Key::Space);
        }
        "Alt" => {
            enigo.key_down(Key::Alt);
        }
        "Windows" | "Super" | "Command" => {
            enigo.key_down(Key::Meta);
        }
        "Control" => {
            enigo.key_down(Key::Control);
        }
        "Shift" => {
            enigo.key_down(Key::Shift);
        }
        "UpArrow" => {
            enigo.key_down(Key::UpArrow);
        }
        "DownArrow" => {
            enigo.key_down(Key::DownArrow);
        }
        "LeftArrow" => {
            enigo.key_down(Key::LeftArrow);
        }
        "RightArrow" => {
            enigo.key_down(Key::RightArrow);
        }
        "Option" => {
            enigo.key_down(Key::Option);
        }
        "MouseLeft" => {
            enigo.mouse_down(MouseButton::Left);
        }
        "MouseRight" => {
            enigo.mouse_down(MouseButton::Right);
        }
        _ => return Err("no key".to_string()),
    }
    // Sleep for the given duration
    // Return Ok with "ok" as the value
    Ok("key down ok".to_string())
}

#[tauri::command]
fn longpress_keyup(key: &str) -> Result<String, String> {
    let mut enigo = Enigo::new();
    match key {
        "Space" => {
            enigo.key_up(Key::Space);
        }
        "Alt" => {
            enigo.key_up(Key::Alt);
        }
        "Windows" | "Super" | "Command" => {
            enigo.key_up(Key::Meta);
        }
        "Control" => {
            enigo.key_up(Key::Control);
        }
        "Shift" => {
            enigo.key_up(Key::Shift);
        }
        "UpArrow" => {
            enigo.key_up(Key::UpArrow);
        }
        "DownArrow" => {
            enigo.key_up(Key::DownArrow);
        }
        "LeftArrow" => {
            enigo.key_up(Key::LeftArrow);
        }
        "RightArrow" => {
            enigo.key_up(Key::RightArrow);
        }
        "Option" => {
            enigo.key_up(Key::Option);
        }
        "MouseLeft" => {
            enigo.mouse_up(MouseButton::Left);
        }
        "MouseRight" => {
            enigo.mouse_up(MouseButton::Right);
        }
        _ => return Err("no key".to_string()),
    }
    // Sleep for the given duration
    // Return Ok with "ok" as the value
    Ok("key up ok".to_string())
}

#[tauri::command]
fn tap_key(key: char) -> Result<String, String> {
    let mut enigo = Enigo::new();
    enigo.key_click(Key::Layout(key));
    Ok("ok".to_string())
}

#[tauri::command]
fn normal_key_down(key: char) -> Result<String, String> {
    let mut enigo = Enigo::new();
    enigo.key_down(Key::Layout(key));
    Ok("ok".to_string())
}

#[tauri::command]
fn normal_key_up(key: char) -> Result<String, String> {
    let mut enigo = Enigo::new();
    enigo.key_up(Key::Layout(key));
    Ok("ok".to_string())
}

#[tauri::command]
fn special_key_down(key: &str) -> Result<String, String> {
    let mut enigo = Enigo::new();
    match key {
        "MouseButtonLeft" => {
            enigo.mouse_down(MouseButton::Left);
        }
        "MouseButtonRight" => {
            enigo.mouse_down(MouseButton::Right);
        }
        "Windows" | "Super" | "Command" => {
            enigo.key_down(Key::Meta);
        }
        "Esc" => {
            enigo.key_down(Key::Escape);
        }
        "Tab" => {
            enigo.key_down(Key::Tab);
        }
        "CapsLock" => {
            enigo.key_down(Key::CapsLock);
        }
        "Shift" => {
            enigo.key_down(Key::Shift);
        }
        "Control" => {
            enigo.key_down(Key::Control);
        }
        "Alt" => {
            enigo.key_down(Key::Alt);
        }
        "Space" => {
            enigo.key_down(Key::Space);
        }
        "Delete" => {
            enigo.key_down(Key::Delete);
        }
        "Backspace" => {
            enigo.key_down(Key::Backspace);
        }
        "Return" | "Enter" => {
            enigo.key_down(Key::Return);
        }
        "Home" => {
            enigo.key_down(Key::Home);
        }
        "End" => {
            enigo.key_down(Key::End);
        }
        "PageDown" => {
            enigo.key_down(Key::PageDown);
        }
        "PageUp" => {
            enigo.key_down(Key::PageUp);
        }
        "UpArrow" => {
            enigo.key_down(Key::UpArrow);
        }
        "DownArrow" => {
            enigo.key_down(Key::DownArrow);
        }
        "LeftArrow" => {
            enigo.key_down(Key::LeftArrow);
        }
        "RightArrow" => {
            enigo.key_down(Key::RightArrow);
        }
        "F1" => {
            enigo.key_down(Key::F1);
        }
        "F2" => {
            enigo.key_down(Key::F2);
        }
        "F3" => {
            enigo.key_down(Key::F3);
        }
        "F4" => {
            enigo.key_down(Key::F4);
        }
        "F5" => {
            enigo.key_down(Key::F5);
        }
        "F6" => {
            enigo.key_down(Key::F6);
        }
        "F7" => {
            enigo.key_down(Key::F7);
        }
        "F8" => {
            enigo.key_down(Key::F8);
        }
        "F9" => {
            enigo.key_down(Key::F9);
        }
        "F10" => {
            enigo.key_down(Key::F10);
        }
        "F11" => {
            enigo.key_down(Key::F11);
        }
        "F12" => {
            enigo.key_down(Key::F12);
        }
        _ => return Err("key match error".to_string()),
    }
    Ok("ok".to_string())
}

#[tauri::command]
fn special_key_up(key: &str) -> Result<String, String> {
    let mut enigo = Enigo::new();
    match key {
        "MouseButtonLeft" => {
            enigo.mouse_up(MouseButton::Left);
        }
        "MouseButtonRight" => {
            enigo.mouse_up(MouseButton::Right);
        }
        "Windows" | "Super" | "Command" => {
            enigo.key_up(Key::Meta);
        }
        "Esc" => {
            enigo.key_up(Key::Escape);
        }
        "Tab" => {
            enigo.key_up(Key::Tab);
        }
        "CapsLock" => {
            enigo.key_up(Key::CapsLock);
        }
        "Shift" => {
            enigo.key_up(Key::Shift);
        }
        "Control" => {
            enigo.key_up(Key::Control);
        }
        "Alt" => {
            enigo.key_up(Key::Alt);
        }
        "Space" => {
            enigo.key_up(Key::Space);
        }
        "Delete" => {
            enigo.key_up(Key::Delete);
        }
        "Backspace" => {
            enigo.key_up(Key::Backspace);
        }
        "Return" | "Enter" => {
            enigo.key_up(Key::Return);
        }
        "Home" => {
            enigo.key_up(Key::Home);
        }
        "End" => {
            enigo.key_up(Key::End);
        }
        "PageDown" => {
            enigo.key_up(Key::PageDown);
        }
        "PageUp" => {
            enigo.key_up(Key::PageUp);
        }
        "UpArrow" => {
            enigo.key_up(Key::UpArrow);
        }
        "DownArrow" => {
            enigo.key_up(Key::DownArrow);
        }
        "LeftArrow" => {
            enigo.key_up(Key::LeftArrow);
        }
        "RightArrow" => {
            enigo.key_up(Key::RightArrow);
        }
        "F1" => {
            enigo.key_up(Key::F1);
        }
        "F2" => {
            enigo.key_up(Key::F2);
        }
        "F3" => {
            enigo.key_up(Key::F3);
        }
        "F4" => {
            enigo.key_up(Key::F4);
        }
        "F5" => {
            enigo.key_up(Key::F5);
        }
        "F6" => {
            enigo.key_up(Key::F6);
        }
        "F7" => {
            enigo.key_up(Key::F7);
        }
        "F8" => {
            enigo.key_up(Key::F8);
        }
        "F9" => {
            enigo.key_up(Key::F9);
        }
        "F10" => {
            enigo.key_up(Key::F10);
        }
        "F11" => {
            enigo.key_up(Key::F11);
        }
        "F12" => {
            enigo.key_up(Key::F12);
        }
        _ => return Err("key match error".to_string()),
    }
    Ok("ok".to_string())
}

#[tauri::command]
fn tab_special_key(key: &str) -> Result<String, String> {
    let mut enigo = Enigo::new();
    match key {
        "Windows" | "Super" | "Command" => {
            enigo.key_click(Key::Meta);
        }
        "Tab" => {
            enigo.key_click(Key::Tab);
        }
        "Shift" => {
            enigo.key_click(Key::Shift);
        }
        "Control" => {
            enigo.key_click(Key::Control);
        }
        "Alt" => {
            enigo.key_click(Key::Alt);
        }
        "Esc" => {
            enigo.key_click(Key::Escape);
        }
        "CapsLock" => {
            enigo.key_click(Key::CapsLock);
        }
        "Space" => {
            enigo.key_click(Key::Space);
        }
        "Delete" => {
            enigo.key_click(Key::Delete);
        }
        "Backspace" => {
            enigo.key_click(Key::Backspace);
        }
        "Return" | "Enter" => {
            enigo.key_click(Key::Return);
        }
        "End" => {
            enigo.key_click(Key::End);
        }
        "UpArrow" => {
            enigo.key_click(Key::UpArrow);
        }
        "DownArrow" => {
            enigo.key_click(Key::DownArrow);
        }
        "LeftArrow" => {
            enigo.key_click(Key::LeftArrow);
        }
        "RightArrow" => {
            enigo.key_click(Key::RightArrow);
        }
        "F1" => {
            enigo.key_click(Key::F1);
        }
        "F2" => {
            enigo.key_click(Key::F2);
        }
        "F3" => {
            enigo.key_click(Key::F3);
        }
        "F4" => {
            enigo.key_click(Key::F4);
        }
        "F5" => {
            enigo.key_click(Key::F5);
        }
        "F6" => {
            enigo.key_click(Key::F6);
        }
        "F7" => {
            enigo.key_click(Key::F7);
        }
        "F8" => {
            enigo.key_click(Key::F8);
        }
        "F9" => {
            enigo.key_click(Key::F9);
        }
        "F10" => {
            enigo.key_click(Key::F10);
        }
        "F11" => {
            enigo.key_click(Key::F11);
        }
        "F12" => {
            enigo.key_click(Key::F12);
        }
        _ => return Err("no key".to_string()),
    }
    Ok("ok".to_string())
}

#[tauri::command]
fn move_mouse(x: i32, y: i32) -> Result<String, String> {
    let mut enigo = Enigo::new();
    enigo.mouse_move_to(x, y);
    Ok("ok".to_string())
}

#[tauri::command]
fn move_mouse_and_click(x: i32, y: i32, m: &str) -> Result<String, String> {
    let mut enigo = Enigo::new();
    enigo.mouse_move_to(x, y);
    if m == "right" {
        enigo.mouse_click(MouseButton::Right);
    } else {
        enigo.mouse_click(MouseButton::Left);
    }
    Ok("ok".to_string())
}

#[tauri::command]
fn input_text(text: &str) -> Result<String, String> {
    let mut enigo = Enigo::new();
    enigo.key_sequence(text);
    Ok("ok".to_string())
}

#[tauri::command]
fn get_pixel_color(x: i32, y: i32) -> (u8, u8, u8) {
    //根据给的x，y坐标，获取rgb颜色
    // windows
    #[cfg(target_os = "windows")]
    {
        // Run Windows-specific code
        println!("Running on Windows!");
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
    //mac
    #[cfg(target_os = "macos")]
    {
        // Run Mac-specific code
        println!("Running on Mac!");
        let color = get_result_of_pixel_color_mac(x, y);
        color
    }
}

#[tauri::command]
fn get_cursor_position() -> (i32, i32) {
    //获取鼠标的坐标
    // let mut point = POINT { x: 0, y: 0 };
    // unsafe { GetCursorPos(&mut point) };
    // println!("Cursor position: ({}, {})", point.x, point.y);
    // (point.x, point.y)
    let enigo = Enigo::new();
    let (x, y) = enigo.mouse_location();
    println!("The mouse cursor is at ({}, {})", x, y);
    (x, y)
}

#[tauri::command]
fn get_cursor_position_and_color() -> ((i32, i32), (u8, u8, u8)) {
    // windows
    #[cfg(target_os = "windows")]
    {
        // get the cursor position
        let (x, y) = get_cursor_position();
        // get the pixel color of that position
        let (r, g, b) = get_pixel_color(x, y);
        // return both results as a tuple
        ((x, y), (r, g, b))
    }
    //mac
    #[cfg(target_os = "macos")]
    {
        // get the cursor position
        let (x, y) = get_cursor_position();
        // get the pixel color of that position
        let (r, g, b) = get_result_of_pixel_color_mac(x, y);
        // return both results as a tuple
        ((x, y), (r, g, b))
    }
}

#[cfg(target_os = "macos")]
fn get_pixel_color_mac(x: i32, y: i32) -> Result<(u8, u8, u8), Box<dyn Error>> {
    // define the function that returns a result with a color tuple or an error
    let screens = Screen::all()?; // get all screens and propagate the error if any
    let screen = screens[0]; // get the first screen
    println!("{:?}", screen);
    println!("scale_factor{:?}", screen.display_info.scale_factor);
    let scale = screen.display_info.scale_factor;
    let real_x = (x as f32 / scale) as i32;
    let real_y = (y as f32 / scale) as i32;
    let image = screen.capture_area(real_x, real_y, 1, 1)?; // capture a 1x1 pixel area at (x, y) and propagate the error if any
    let buffer = image.buffer(); // get the buffer as a slice of u8
                                 // println!("buffer{:?}", buffer);
                                 //  保存为文件
                                 // fs::write(format!("target/pixel.png"), buffer).unwrap();

    // Load an image from the buffer
    let pixelimage = image::load_from_memory(buffer)?;
    // Get the dimensions of the image
    let (width, height) = pixelimage.dimensions();
    println!("{},{}", width, height);
    // Iterate over the pixels and print them
    // for pixel in pixelimage.pixels() {
    //     println!("pixel{:?}", pixel)
    // }

    let pixel = pixelimage.get_pixel(0, 0);

    // let rgbcolor = pixel.to_rgb();

    // println!("The pixel color is: {:?}", rgbcolor);
    // Ok((rgbcolor[0], rgbcolor[1], rgbcolor[2])) // return the color as a result
    Ok((pixel[0], pixel[1], pixel[2])) // return the color as a result
}

#[cfg(target_os = "macos")]
fn get_result_of_pixel_color_mac(x: i32, y: i32) -> (u8, u8, u8) {
    match get_pixel_color_mac(x, y) {
        // call the function with coordinates and handle the result
        Ok(color) => color, // print the color if successful
        Err(e) => {
            println!("{:?}", e);
            (0, 0, 0)
        } // print the error if failed
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
            println!("{}, {argv:?}, {cwd}", app.package_info().name);

            app.emit_all("single-instance", Payload { args: argv, cwd })
                .unwrap();
        }))
        .invoke_handler(tauri::generate_handler![
            // greet,
            get_cursor_position,
            get_pixel_color,
            move_mouse,
            move_mouse_and_click,
            tap_key,
            tab_special_key,
            input_text,
            get_cursor_position_and_color,
            longpress_keydown,
            longpress_keyup,
            normal_key_down,
            normal_key_up,
            special_key_down,
            special_key_up
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
