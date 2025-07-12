use sha2::{Sha256, Digest};
use reqwest::Client;


#[tauri::command]
pub async fn get_url_data_hash(app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    match get_url_data_hash_impl(app, params).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in get_install_status: {}", e)),
    }
}

async fn get_url_data_hash_impl(_app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    // ...

    let url = params["url"].as_str().ok_or("Missing URL parameter")?;

    println!("url: {}", url);

    
    let response = reqwest::get(url).await.unwrap().bytes().await.unwrap();

    let mut hasher = Sha256::new();
    hasher.update(response);
    let hash = format!("{:x}", hasher.finalize());

    println!("hash: {}", hash);


    Ok(serde_json::json!({
        "hash": hash,
    }))
}

#[tauri::command]
pub async fn relay_post_to_sidecar(app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    match relay_post_to_sidecar_impl(app, params).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in relay_post_to_sidecar: {}", e)),
    }
}
        

async fn relay_post_to_sidecar_impl(_app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    println!("Relaying to sidecar: {:?}", params);

    let port = params["port"].as_u64().ok_or("Missing port parameter")?;
    let method = params["method"].as_str().ok_or("Missing method parameter")?;
    println!("port: {} method: {}", port, method);

    if port < 5200 || port > 5300{
        return Err("Invalid port number".to_string());
    }

    let url = format!("http://localhost:{}/{}", port, method);
    let client = Client::new();
    let response = client.post(&url)
        .json(&params["data"])
        .send()
        .await.map_err(|e| format!("Error sending request: {}", e))?;

    println!("response: {:?}", response);

    let response_json: serde_json::Value = response.json()
        .await.map_err(|e| format!("Error parsing response: {}", e))?;

    println!("response_json \n\n: {:?}", response_json);

    Ok(response_json)
}

#[tauri::command]
pub async fn relay_get_to_sidecar(app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    match relay_get_to_sidecar_impl(app, params).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in relay_get_to_sidecar: {}", e)),
    }
}

async fn relay_get_to_sidecar_impl(_app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    println!("Relaying to sidecar: {:?}", params);

    let port = params["port"].as_u64().ok_or("Missing port parameter")?;
    let method = params["method"].as_str().ok_or("Missing method parameter")?;

    if port < 5200 || port > 5300{
        return Err("Invalid port number".to_string());
    }

    let url = format!("http://localhost:{}/{}", port, method);
    let client = Client::new();
    let response = client.get(&url)
        .send()
        .await.map_err(|e| format!("Error sending request: {}", e))?;

    let response_json: serde_json::Value = response.json()
        .await.map_err(|e| format!("Error parsing response: {}", e))?;

    println!("response_json: {:?}", response_json);
    Ok(response_json)
}