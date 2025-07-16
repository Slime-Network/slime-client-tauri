
use serde_json::json;
use std::fs;
use rusqlite::{Connection, Result};


#[tauri::command]
pub async fn get_install_status(app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    match get_install_status_impl(app, params).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in get_install_status: {}", e)),
    }
}

async fn get_install_status_impl(_app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {

    println!("get_install_status params: {:?}", params);
    
    Ok(serde_json::json!({
        "isDownloading": false,
        "isDownloaded": false,
        "isInstalling": false,
        "isInstalled": false,
        "hasPendingUpdate": false,
        "progress": 0,
        "isSeeding": false,
        "downloadRate": 0,
        "uploadRate": 0,
    }))
}


#[tauri::command]
pub async fn download_media(app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    match download_media_impl(app, params).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in download_media: {}", e)),
    }
}

async fn download_media_impl(_app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    println!("download_media params: {:?}", params);
    // let mut file = File::open(app.path_resolver().resolve_resource("./assets/slime-config.json").expect("failed to resolve resource")).map_err(|e| format!("Error opening file: {}", e))?;
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).map_err(|e| format!("Error reading file: {}", e))?;
    
    // let config: serde_json::Value = serde_json::from_str(&contents).map_err(|e| format!("Error parsing JSON: {}", e))?;

    Ok(json!({
        "status": "downloading",
        "message": "Downloading media"
    }))
}


#[tauri::command]
pub async fn delete_media(app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    match delete_media_impl(app, params).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in delete_media: {}", e)),
    }
}

async fn delete_media_impl(_app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    
    let conn = Connection::open("./assets/slime.db").map_err(|e| format!("Error opening database: {}", e))?;
    
    let mut stmt = conn.prepare("SELECT * FROM activeConfig WHERE id = 1").map_err(|e| format!("Error preparing statement: {}", e))?;
    let config = stmt.query_row([], |row| {
        let did: Option<String> = row.get(1)?;
        let active_proof: Option<String> = row.get(2)?;
        let marketplace_display_name: Option<String> = row.get(3)?;
        let marketplace_url: Option<String> = row.get(4)?;
        let torrent_client_port: Option<i32> = row.get(5)?;
        let languages: Option<String> = row.get(6)?;
        let install_path: Option<String> = row.get(7)?;
        let install_path_display_name: Option<String> = row.get(8)?;
        let torrent_path: Option<String> = row.get(9)?;
        let torrent_path_display_name: Option<String> = row.get(10)?;
        let minting_data_path: Option<String> = row.get(11)?;
        Ok(serde_json::json!({
            "did": did,
            "activeProof": active_proof,
            "marketplaceDisplayName": marketplace_display_name,
            "marketplaceUrl": marketplace_url,
            "torrentClientPort": torrent_client_port,
            "languages": languages,
            "installPath": install_path,
            "installPathDisplayName": install_path_display_name,
            "torrentPath": torrent_path,
            "torrentPathDisplayName": torrent_path_display_name,
            "mintingDataPath": minting_data_path
        }))
    }).unwrap();

    let torrents_path = format!("{}/{}", config["torrentsPath"], params["productId"].as_str().unwrap());
    let media_data_path = format!("{}/{}", config["mediaDataPath"], params["productId"].as_str().unwrap());

    fs::remove_dir_all(torrents_path).map_err(|e| format!("Error deleting directory: {}", e))?;
    fs::remove_dir_all(media_data_path).map_err(|e| format!("Error deleting directory: {}", e))?;

    Ok(json!({
        "status": "deleted",
        "message": "Media deleted"
    }))
}


#[tauri::command]
pub async fn install_media(app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    match install_media_impl(app, params).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in install_media: {}", e)),
    }
}

async fn install_media_impl(_app: tauri::AppHandle, _params: serde_json::Value) -> Result<serde_json::Value, String> {
    // let mut file = File::open(app.path_resolver().resolve_resource("./assets/slime-config.json").expect("failed to resolve resource")).map_err(|e| format!("Error opening file: {}", e))?;
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).map_err(|e| format!("Error reading file: {}", e))?;
    
    // let config: serde_json::Value = serde_json::from_str(&contents).map_err(|e| format!("Error parsing JSON: {}", e))?;

    // let torrents_path = format!("{}/{}", config["torrentsPath"], params["productId"].as_str().unwrap());

    // let file = File::open(&torrents_path).map_err(|e| format!("Error opening file: {}", e))?;
    // let mut archive = zip::ZipArchive::new(file).map_err(|e| format!("Error reading zip: {}", e))?;

    // for i in 0..archive.len() {
    //     let mut file = archive.by_index(i).map_err(|e| format!("Error reading file from zip: {}", e))?;
    //     let outpath = file.mangled_name();

    //     if (&*file.name()).ends_with('/') {
    //         fs::create_dir_all(&outpath).map_err(|e| format!("Error creating directory: {}", e))?;
    //     } else {
    //         if let Some(p) = outpath.parent() {
    //             if !p.exists() {
    //                 fs::create_dir_all(&p).map_err(|e| format!("Error creating directory: {}", e))?;
    //             }
    //         }
    //         let mut outfile = fs::File::create(&outpath).map_err(|e| format!("Error creating file: {}", e))?;
    //         io::copy(&mut file, &mut outfile).map_err(|e| format!("Error writing file: {}", e))?;
    //     }
    // }
    

    Ok(json!({
        "status": "installed",
        "message": "Media installed"
    }))
}


#[tauri::command]
pub async fn uninstall_media(app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    match uninstall_media_impl(app, params).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in uninstall_media: {}", e)),
    }
}

async fn uninstall_media_impl(_app: tauri::AppHandle, _params: serde_json::Value) -> Result<serde_json::Value, String> {
    // let mut file = File::open(app.path_resolver().resolve_resource("./assets/slime-config.json").expect("failed to resolve resource")).map_err(|e| format!("Error opening file: {}", e))?;
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).map_err(|e| format!("Error reading file: {}", e))?;
    
    // let config: serde_json::Value = serde_json::from_str(&contents).map_err(|e| format!("Error parsing JSON: {}", e))?;

    // let installs_path = format!("{}/{}", config["installsPath"], params["productId"].as_str().unwrap());
    
    // fs::remove_dir_all(installs_path).map_err(|e| format!("Error deleting directory: {}", e))?;
    // Ok(json!({
    //     "status": "uninstalled",
    //     "message": "Media uninstalled"
    // }))
    Ok(json!({
        "status": "success",
        "message": "Uninstall media is not implemented yet"
    }))
}


#[tauri::command]
pub async fn launch_media(app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    match launch_media_impl(app, params).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in launch_media: {}", e)),
    }
}

async fn launch_media_impl(_app: tauri::AppHandle, _params: serde_json::Value) -> Result<serde_json::Value, String> {
    // let mut file = File::open(app.path_resolver().resolve_resource("./assets/slime-config.json").expect("failed to resolve resource")).map_err(|e| format!("Error opening file: {}", e))?;
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).map_err(|e| format!("Error reading file: {}", e))?;
    
    // let config: serde_json::Value = serde_json::from_str(&contents).map_err(|e| format!("Error parsing JSON: {}", e))?;

    // let installs_path = format!("{}/{}", config["installsPath"], params["productId"].as_str().unwrap());

    // // needs the executable added to the path
    // if cfg!(target_os = "windows") {
    //     std::process::Command::new("cmd")
    //         .args(&["/C", "start", &installs_path])
    //         .spawn()
    //         .expect("failed to start application");
    // } else if cfg!(target_os = "linux") {
    //     std::process::Command::new("xdg-open")
    //         .args(&[&installs_path])
    //         .spawn()
    //         .expect("failed to start application");
    // } else {
    //     std::process::Command::new("open")
    //         .args(&[&installs_path])
    //         .spawn()
    //         .expect("failed to start application");
    // }

    // Ok(json!({
    //     "status": "playing",
    //     "message": "Media launched",
    //     "pid": "1234"
    // }))
    Ok(json!({
        "status": "success",
        "message": "Media launch is not implemented yet"
    }))
}



#[tauri::command]
pub async fn generate_torrent(app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    match generate_torrent_impl(app, params).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in generate_torrents: {}", e)),
    }
}

async fn generate_torrent_impl(_app: tauri::AppHandle, _params: serde_json::Value) -> Result<serde_json::Value, String> {
    // println!("generate_torrent params: {:?}", params);
    // let mut file = File::open(app.asset_resolver().get("./assets/slime-config.json").expect("failed to resolve resource")).map_err(|e| format!("Error opening file: {}", e))?;
    // let mut contents = String::new();
    // file.read_to_string(&mut contents).map_err(|e| format!("Error reading file: {}", e))?;
    
    // let config: serde_json::Value = serde_json::from_str(&contents).map_err(|e| format!("Error parsing JSON: {}", e))?;

    // let files = FileDialog::new()
    // .set_title("Select Game Folder")
    // .set_directory(&config["mediaDataPath"].as_str().unwrap())
    // .pick_folder();

    // let folder_path = match files {
    //     Some(path) => path,
    //     None => return Err("No folder selected".to_string()),
    // };

    // let full_path = folder_path.to_string_lossy().into_owned();

    // println!("full_path: {:?}", full_path);

    // println!("Params: {:?}", params["mediaFiles"]["name"]);

    // let torrent_path = format!("{}/{}", config["torrentsPath"].clone().as_str().unwrap(), params["mediaFiles"]["name"].as_str().unwrap());

    // println!("torrents_path: {:?}", torrent_path);

    // fs::create_dir_all(torrent_path.clone()).map_err(|e| format!("Error creating directory: {}", e))?;

	// let client = HttpClient::builder().build("http://localhost:5235").unwrap();

    // let file_name = format!("{}/{}-{}.zip", torrent_path, params["mediaFiles"]["name"].as_str().unwrap(), params["mediaFiles"]["version"].as_str().unwrap());

    // let params = rpc_params![params["mediaFiles"].clone(), full_path, file_name];
    // let response: Result<serde_json::Value, _> = client.request("generateTorrent",  params).await.map_err(|e| format!("Error generating torrent: {}", e));

    // let resp = response.unwrap();

    // Ok(json!({
    //     "torrent": resp["torrent"],
    //     "fileName": resp["fileName"],
    //     "size": resp["size"],
    //     "message": "Torrents generated"
    // }))
    Ok(json!({
        "status": "success",
        "message": "Torrent generation is not implemented yet"
    }))
}