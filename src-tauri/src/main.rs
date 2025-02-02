// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod torrents;
mod slime_nostr;
mod util;

use tauri::api::process::{Command, CommandEvent};
use tauri::{CustomMenuItem, Manager, State, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};
use std::fs::{self, File};
use std::io::Read;
use std::fs::write;
use std::path::Path;

use std::sync::Mutex;


#[tauri::command]
fn get_operating_system() -> Result<serde_json::Value, String> {
    match get_operating_system_impl() {
        Ok(os) => Ok(serde_json::json!({"os": os})),
        Err(e) => Err(format!("Error in get_operating_system: {}", e)),
    }
}

fn get_operating_system_impl() -> Result<String, String> {
    let os = std::env::consts::OS;
    Ok(os.to_string())
}


#[tauri::command]
async fn open_app(app: tauri::AppHandle, app_name: String, title: String, url: String) {
    tauri::WindowBuilder::new(
            &app,
            app_name.clone(), /* set the window label to the app name */
            tauri::WindowUrl::App(
                url.into()
            )
        )
        .title(title) /* set the window title to the app name */
        .inner_size(1200.0, 700.0)
        .build()
        .expect("failed to build window");
}



#[tauri::command]
async fn get_local_media_metadata(app: tauri::AppHandle, product_id: serde_json::Value) -> Result<serde_json::Value, String> {
    match get_local_media_metadata_impl(app, product_id).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in get_local_media_metadata: {}", e)),
    }
}

async fn get_local_media_metadata_impl(app: tauri::AppHandle, product_id: serde_json::Value) -> Result<serde_json::Value, String> {
    println!("get_local_media_metadata_impl params: {:?}", product_id);
    let mut file = File::open(app.path_resolver().resolve_resource(format!("../resources/localmediametadata/{}.json", product_id.as_str().unwrap())).expect("failed to resolve resource")).map_err(|e| format!("Error opening file: {}", e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| format!("Error reading file: {}", e))?;
    
    // Parse the contents as JSON
    let config: serde_json::Value = serde_json::from_str(&contents).map_err(|e| format!("Error parsing JSON: {}", e))?;
    
    Ok(config)
}


#[tauri::command]
async fn save_local_media_metadata(app: tauri::AppHandle, media: serde_json::Value) -> Result<serde_json::Value, String> {
    match save_local_media_metadata_impl(app, media).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in save_local_media_metadata: {}", e)),
    }
}

async fn save_local_media_metadata_impl(app: tauri::AppHandle, media: serde_json::Value) -> Result<serde_json::Value, String> {
    let contents = serde_json::to_string(&media).map_err(|e| format!("Error serializing media: {}", e))?;
    println!("media contents: {:?}", contents);
    let path = app.path_resolver().resolve_resource(format!("../resources/localmediametadata/{}.json", media["productId"].as_str().unwrap())).expect("failed to resolve resource");
    let path = Path::new(&path);
    if !path.exists() {
        fs::create_dir_all(path.parent().unwrap()).map_err(|e| format!("Error creating directory: {}", e))?;
    }
    write(path, contents).map_err(|e| format!("Error writing file: {}", e))?;
    Ok(serde_json::json!({"status": "saved", "message": "Config saved"}))
}




struct SlimeConfig(Mutex<serde_json::Value>);

#[tauri::command]
fn get_config(app: tauri::AppHandle, config: State<SlimeConfig>) -> Result<serde_json::Value, String> {
    match get_config_impl(app, config) {
        Ok(config) => Ok(serde_json::json!({"result": config, "valid": true, "message": "Config loaded"})),
        Err(e) => Err(format!("Error in get_config: {}", e)),
    }
}

fn get_config_impl(app: tauri::AppHandle, config: State<SlimeConfig>) -> Result<serde_json::Value, String> {
    println!("Loading config");
    let mut conf = config.0.lock().unwrap();
    println!("conf: {:?}", conf);
    if conf.is_null() {
        println!("Loading config");
        let mut file = File::open(app.path_resolver().resolve_resource("../resources/slime-config.json").expect("failed to resolve resource")).map_err(|e| format!("Error opening file: {}", e))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|e| format!("Error reading file: {}", e))?;
        *conf = serde_json::from_str(&contents).map_err(|e| format!("Error parsing JSON: {}", e))?;
    }
    Ok(conf.clone())
}

#[tauri::command]
fn save_config(app: tauri::AppHandle, config: State<SlimeConfig>, new_config: serde_json::Value) -> Result<serde_json::Value, String> {
    match save_config_impl(app, config, new_config.clone()) {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in save_config: {}", e)),
    }
}

fn save_config_impl(app: tauri::AppHandle, config: State<SlimeConfig>, new_config: serde_json::Value) -> Result<serde_json::Value, String> {
    println!("Saving config");
    let mut conf = config.0.lock().unwrap();
    *conf = new_config.clone();
    let contents = serde_json::to_string(&new_config).map_err(|e| format!("Error serializing config: {}", e))?;
    write(app.path_resolver().resolve_resource("../resources/slime-config.json").expect("failed to resolve resource"), contents).map_err(|e| format!("Error writing file: {}", e))?;

    Ok(serde_json::json!({"message": "Config saved"}))
}


async fn start() {

    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let show = CustomMenuItem::new("show".to_string(), "Open");
    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    let (mut rx, mut _child) = Command::new_sidecar("torrentclient")
        .expect("failed to create `torrentclient` binary command")
        .args(&["../resources/slime-config.json"])
        .spawn()
        .expect("Failed to spawn sidecar");

    tauri::async_runtime::spawn(async move {
        // read events such as stdout
        while let Some(event) = rx.recv().await {
            println!("python event: {:?}", event);
            if let CommandEvent::Stdout(line) = event {
                println!("stdout: {}", line);
            } else if let CommandEvent::Stderr(line) = event {
                println!("stderr: {}", line);
            }
        }
        });

    tauri::Builder::default()
        .manage(SlimeConfig(Mutex::new(serde_json::Value::Null)))
        .system_tray(SystemTray::new().with_menu(tray_menu))
        .on_system_tray_event(|app, event| match event {
        SystemTrayEvent::LeftClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a left click");
            let windows = app.windows();
            for (_, window) in &windows {
                window.show().unwrap();
            }
        }
        SystemTrayEvent::RightClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a right click");
        }
        SystemTrayEvent::DoubleClick {
            position: _,
            size: _,
            ..
        } => {
            println!("system tray received a double click");
        }
        SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
            "quit" => {
                std::process::exit(0);
            }
            "hide" => {
                let windows = app.windows();
                for (_, window) in &windows {
                    window.hide().unwrap();
                }
            }
            "show" => {
                let windows = app.windows();
                for (_, window) in &windows {
                    window.show().unwrap();
                }
            }
            _ => {}
            }
        }
        _ => {}
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                if event.window().label() == "main" {
                    event.window().hide().unwrap();
                    api.prevent_close();
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            open_app,
            get_config,
            save_config,
            get_operating_system,
            slime_nostr::add_nostr_keypair,
            slime_nostr::has_nostr_private_key,
            slime_nostr::sign_nostr_message,
            get_local_media_metadata,
            save_local_media_metadata,
            torrents:: get_install_status,
            torrents::download_media,
            torrents::delete_media,
            torrents::install_media,
            torrents::uninstall_media,
            torrents::launch_media,
            torrents:: generate_torrent,
            util::get_url_data_hash,
            util::relay_post_to_sidecar,
            util::relay_get_to_sidecar
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn main() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("couldn't set up tokio runtime")
        .block_on(start())
}