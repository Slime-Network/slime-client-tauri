// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod torrents;
mod slime_nostr;
mod util;

use tauri::api::process::{Command, CommandEvent};
use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem};

use rusqlite::{Connection, Result, params};


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
        .inner_size(1500.0, 1200.0)
        .build()
        .expect("failed to build window");
}


#[tauri::command]
async fn get_local_media_metadata(app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    match get_local_media_metadata_impl(app, params).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in get_local_media_metadata: {}", e)),
    }
}

async fn get_local_media_metadata_impl(_app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    let conn = Connection::open("../resources/slime.db").map_err(|e| format!("Error opening database: {}", e))?;
    
    let mut stmt = conn.prepare("SELECT * FROM media WHERE productId = ?").map_err(|e| format!("Error preparing statement: {}", e))?;

    let media = stmt.query_row([params["productId"].as_str().unwrap()], |row| {
        let product_id: String = row.get(0)?;
        let content_ratings: String = row.get(1)?;
        let descriptions: String = row.get(2)?;
        let credits: String = row.get(3)?;
        let child_products: String = row.get(4)?;
        let last_updated: i64 = row.get(5)?;
        let last_updated_content: i64 = row.get(6)?;
        let media_type: String = row.get(7)?;
        let nostr_event_id: String = row.get(8)?;
        let images: String = row.get(9)?;
        let videos: String = row.get(10)?;
        let donation_address: String = row.get(11)?;
        let parent_product_id: String = row.get(12)?;
        let publisher_did: String = row.get(13)?;
        let release_status: String = row.get(14)?;
        let support_contact: String = row.get(15)?;
        let tags: String = row.get(16)?;
        let titles: String = row.get(17)?;
        let files: String = row.get(18)?;

        Ok(serde_json::json!({
            "productId": product_id,
            "contentRatings": content_ratings,
            "descriptions": descriptions,
            "credits": credits,
            "childProducts": child_products,
            "lastUpdated": last_updated,
            "lastUpdatedContent": last_updated_content,
            "mediaType": media_type,
            "nostrEventId": nostr_event_id,
            "images": images,
            "videos": videos,
            "donationAddress": donation_address,
            "parentProductId": parent_product_id,
            "publisherDid": publisher_did,
            "releaseStatus": release_status,
            "supportContact": support_contact,
            "tags": tags,
            "titles": titles,
            "files": files
        }))
    });

    Ok(match media {
        Ok(media) => {
            println!("Media loaded: {:?}", media);
            media
        }
        Err(e) => {
            println!("Error loading media: {}", e);
            serde_json::json!({"error": "Failed to load media"})
        }
    })
}


#[tauri::command]
async fn save_local_media_metadata(app: tauri::AppHandle, media: serde_json::Value) -> Result<serde_json::Value, String> {
    match save_local_media_metadata_impl(app, media).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in save_local_media_metadata: {}", e)),
    }
}

async fn save_local_media_metadata_impl(_app: tauri::AppHandle, media: serde_json::Value) -> Result<serde_json::Value, String> {
    let conn = Connection::open("../resources/slime.db").map_err(|e| format!("Error opening database: {}", e))?;
    let _ = conn.execute(
        "INSERT OR REPLACE INTO media (productId, contentRatings, descriptions, credits, childProducts, lastUpdated, lastUpdatedContent, mediaType, nostrEventId, images, videos, donationAddress, parentProductId, publisherDid, releaseStatus, supportContact, tags, titles, files)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        [
            media["productId"].as_str().unwrap(),
            media["contentRatings"].as_str().unwrap(),
            media["descriptions"].as_str().unwrap(),
            media["credits"].as_str().unwrap(),
            media["childProducts"].as_str().unwrap(),
            media["lastUpdated"].as_str().unwrap(),
            media["lastUpdatedContent"].as_str().unwrap(),
            media["mediaType"].as_str().unwrap(),
            media["nostrEventId"].as_str().unwrap(),
            media["images"].as_str().unwrap(),
            media["videos"].as_str().unwrap(),
            media["donationAddress"].as_str().unwrap(),
            media["parentProductId"].as_str().unwrap(),
            media["publisherDid"].as_str().unwrap(),
            media["releaseStatus"].as_str().unwrap(),
            media["supportContact"].as_str().unwrap(),
            media["tags"].as_str().unwrap(),
            media["titles"].as_str().unwrap(),
            media["files"].as_str().unwrap()
        ],
    ).map_err(|e| format!("Error executing statement: {}", e))?;

    Ok(serde_json::json!({"status": "saved", "message": "Config saved"}))
}


#[tauri::command]
fn get_config(app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    match get_config_impl(app) {
        Ok(config) => Ok(serde_json::json!({"result": config, "valid": true, "message": "Config loaded"})),
        Err(e) => Err(format!("Error in get_config: {}", e)),
    }
}

fn get_config_impl(_app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    let conn = Connection::open("../resources/slime.db").map_err(|e| format!("Error opening database: {}", e))?;
    
    let mut stmt = conn.prepare("SELECT * FROM activeConfig WHERE id = 1").map_err(|e| format!("Error preparing statement: {}", e))?;
    let conf = stmt.query_row([], |row| {
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
    });

    Ok(match conf {
        Ok(config) => {
            println!("Config loaded: {:?}", config);
            config
        }
        Err(e) => {
            println!("Error loading config: {}", e);
            serde_json::json!({"error": "Failed to load config"})
        }
    })
}


#[tauri::command]
fn get_minting_config(app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    match get_minting_config_impl(app, params) {
        Ok(config) => Ok(serde_json::json!({"result": config, "valid": true, "message": "Config loaded"})),
        Err(e) => Err(format!("Error in get_minting_config: {}", e)),
    }
}

fn get_minting_config_impl(_app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    let conn = Connection::open("../resources/slime.db").map_err(|e| format!("Error opening database: {}", e))?;
    
    let mut stmt = conn.prepare("SELECT * FROM mintingConfig WHERE id = ?").map_err(|e| format!("Error preparing statement: {}", e))?;
    let conf = stmt.query_row([params["id"].as_str().unwrap()], |row| {
        let id: Option<String> = row.get(0)?;
        let icon_uri: Option<String> = row.get(1)?;
        let banner_uri: Option<String> = row.get(2)?;
        let quantity: Option<i32> = row.get(3)?;
        let batch_size: Option<i32> = row.get(4)?;
        let edition: Option<String> = row.get(5)?;
        let receive_address: Option<String> = row.get(6)?;
        let royalty_address: Option<String> = row.get(7)?;
        let royalty_percentage: Option<i32> = row.get(8)?;
        let display_image_uris: Option<String> = row.get(9)?;
        let metadata_uris: Option<String> = row.get(10)?;
        let license_uris: Option<String> = row.get(11)?;
        let generate_offer_files: Option<String> = row.get(12)?;
        let sensitive_content: Option<String> = row.get(13)?;
        let sale_price: Option<f32> = row.get(14)?;
        let sale_asset: Option<String> = row.get(15)?;
        let minting_fee: Option<i32> = row.get(16)?;
        Ok(serde_json::json!({
            "id": id,
            "iconUri": icon_uri,
            "bannerUri": banner_uri,
            "quantity": quantity,
            "batchSize": batch_size,
            "edition": edition,
            "receiveAddress": receive_address,
            "royaltyAddress": royalty_address,
            "royaltyPercentage": royalty_percentage,
            "displayImageUris": display_image_uris,
            "metadataUris": metadata_uris,
            "licenseUris": license_uris,
            "generateOfferFiles": generate_offer_files,
            "sensitiveContent": sensitive_content,
            "salePrice": sale_price,
            "saleAsset": sale_asset,
            "mintingFee": minting_fee
        }))
    });

    Ok(match conf {
        Ok(config) => {
            println!("Minting Config loaded: {:?}", config);
            config
        }
        Err(e) => {
            println!("Error loading config: {}", e);
            serde_json::json!({"error": "Failed to load config"})
        }
    })
}

#[tauri::command]
fn set_minting_config(app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    match set_minting_config_impl(app, params) {
        Ok(params) => Ok(serde_json::json!({"result": params, "valid": true, "message": "Config saved"})),
        Err(e) => Err(format!("Error in set_config: {}", e)),
    }
}

#[tauri::command]
fn set_minting_config_impl(_app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    let conn = Connection::open("../resources/slime.db").map_err(|e| format!("Error opening database: {}", e))?;
    
    println!("set minting config params: {:?}", params);
    
    let _ = conn.execute(
        "INSERT OR REPLACE INTO mintingConfig (id, iconUri, bannerUri, quantity, batchSize, edition, receiveAddress, royaltyAddress, royaltyPercentage, displayImageUris, metadataUris, licenseUris, generateOfferFiles, sensitiveContent, salePrice, saleAsset, mintingFee)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        params![
            params["id"].as_str().expect("id is missing"),
            params["iconUri"].as_str().or(Some("")).unwrap(),
            params["bannerUri"].as_str().or(Some("")).unwrap(),
            params["quantity"].as_i64().or(Some(0)).unwrap(),
            params["batchSize"].as_i64().or(Some(0)).unwrap(),
            params["edition"].as_str().or(Some("")).unwrap(),
            params["receiveAddress"].as_str().or(Some("")).unwrap(),
            params["royaltyAddress"].as_str().or(Some("")).unwrap(),
            params["royaltyPercentage"].as_i64().or(Some(0)).unwrap(),
            params["displayImageUris"].as_str().or(Some("[]")).unwrap(),
            params["metadataUris"].as_str().or(Some("[]")).unwrap(),
            params["licenseUris"].as_str().or(Some("[]")).unwrap(),
            params["generateOfferFiles"].as_str().or(Some("true")).unwrap(),
            params["sensitiveContent"].as_str().or(Some("false")).unwrap(),
            params["salePrice"].as_f64().or(Some(0.0)).unwrap(),
            params["saleAsset"].as_str().or(Some("{\"displayName\": \"XCH\", \"assetId\": \"-1\", \"url\": \"https://chia.net\"}")).unwrap(),
            params["mintingFee"].as_i64().or(Some(0)).unwrap(),
        ],
    ).map_err(|e| format!("Error executing statement: {}", e))?;
    Ok(serde_json::json!({"status": "saved", "message": "Config saved"}))
}

#[tauri::command]
async fn get_marketplaces(_app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    match get_marketplaces_impl().await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in get_marketplaces: {}", e)),
    }
}

async fn get_marketplaces_impl() -> Result<serde_json::Value, String> {
    let conn = Connection::open("../resources/slime.db").map_err(|e| format!("Error opening database: {}", e))?;
    
    let mut stmt = conn.prepare("SELECT * FROM marketplaces").map_err(|e| format!("Error preparing statement: {}", e))?;

    let mut marketplaces = Vec::new();
    let rows = stmt.query_map([], |row| {
        let id: i32 = row.get(0)?;
        let display_name: String = row.get(1)?;
        let url: String = row.get(2)?;

        Ok(serde_json::json!({
            "id": id,
            "displayName": display_name,
            "url": url
        }))
    }).map_err(|e| format!("Error During Query: {}", e))?;
    for row in rows {
        match row {
            Ok(marketplace) => marketplaces.push(marketplace),
            Err(e) => return Err(format!("Error processing row: {}", e)),
        }
    }
    Ok(serde_json::json!(marketplaces))
}

#[tauri::command]
async fn add_marketplace(_app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    match add_marketplace_impl(params).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in add_marketplace: {}", e)),
    }
}

async fn add_marketplace_impl(params: serde_json::Value) -> Result<serde_json::Value, String> {
    let conn = Connection::open("../resources/slime.db").map_err(|e| format!("Error opening database: {}", e))?;
    println!("add marketplace params: {:?}", params);
    if params["id"].as_i64() >= Some(0) {
        let _ = conn.execute(
            "UPDATE marketplaces SET displayName = ?, url = ? WHERE id = ?",
            [
                params["displayName"].as_str().unwrap(),
                params["url"].as_str().unwrap(),
                params["id"].as_str().unwrap()
            ],
        ).map_err(|e| format!("Error executing statement: {}", e))?;
        return Ok(serde_json::json!({"status": "saved", "message": "Config saved"}));
    }
    let _ = conn.execute(
        "INSERT OR REPLACE INTO Marketplaces (
            displayName, url
        )
        VALUES (?, ?)",
        [
            params["displayName"].as_str().unwrap(),
            params["url"].as_str().unwrap()
        ],
    ).map_err(|e| format!("Error executing statement: {}", e))?;

    Ok(serde_json::json!({"status": "saved", "message": "Config saved"}))
}

#[tauri::command]
async fn remove_marketplace(_app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    match remove_marketplace_impl(params).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in remove_marketplace: {}", e)),
    }
}

async fn remove_marketplace_impl(params: serde_json::Value) -> Result<serde_json::Value, String> {
    let conn = Connection::open("../resources/slime.db").map_err(|e| format!("Error opening database: {}", e))?;
    let _ = conn.execute(
        "DELETE FROM marketplaces WHERE id = ?",
        [params["id"].as_i64().unwrap() as i32],
    ).map_err(|e| format!("Error executing statement: {}", e))?;

    Ok(serde_json::json!({"status": "saved", "message": "Config saved"}))
}


#[tauri::command]
async fn set_active_marketplace(_app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    match set_active_marketplace_impl(params).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in set_active_marketplace: {}", e)),
    }
}

async fn set_active_marketplace_impl(params: serde_json::Value) -> Result<serde_json::Value, String> {
    let conn = Connection::open("../resources/slime.db").map_err(|e| format!("Error opening database: {}", e))?;
    let _ = conn.execute(
        "UPDATE activeConfig SET marketplaceDisplayName = (SELECT displayName FROM marketplaces WHERE id = ?), marketplaceUrl = (SELECT url FROM marketplaces WHERE id = ?) WHERE id = 1",
        [
            params["marketplaceId"].as_i64().unwrap(),
            params["marketplaceId"].as_i64().unwrap()
        ],
    ).map_err(|e| format!("Error executing statement: {}", e))?;
    Ok(serde_json::json!({"status": "saved", "message": "Config saved"}))
}

#[tauri::command]
async fn get_nostr_relays(_app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    match get_nostr_relays_impl().await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in get_nostr_relays: {}", e)),
    }
}

async fn get_nostr_relays_impl() -> Result<serde_json::Value, String> {
    let conn = Connection::open("../resources/slime.db").map_err(|e| format!("Error opening database: {}", e))?;
    
    let mut stmt = conn.prepare("SELECT * FROM nostrRelays").map_err(|e| format!("Error preparing statement: {}", e))?;

    let mut relays = Vec::new();

    let rows = stmt.query_map([], |row| {
        let id: i32 = row.get(0)?;
        let display_name: String = row.get(1)?;
        let url: String = row.get(2)?;

        Ok(serde_json::json!({
            "id": id,
            "displayName": display_name,
            "url": url
        }))
    }).map_err(|e| format!("Error During Query: {}", e))?;
    for row in rows {
        match row {
            Ok(relay) => relays.push(relay),
            Err(e) => return Err(format!("Error processing row: {}", e)),
        }
    }
    Ok(serde_json::json!(relays))
}

#[tauri::command]
async fn add_nostr_relay(_app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    match add_nostr_relay_impl(params).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in add_nostr_relay: {}", e)),
    }
}

async fn add_nostr_relay_impl(params: serde_json::Value) -> Result<serde_json::Value, String> {
    let conn = Connection::open("../resources/slime.db").map_err(|e| format!("Error opening database: {}", e))?;
    let _ = conn.execute(
        "INSERT OR REPLACE INTO nostrRelays (
            displayName, url
        )
        VALUES (?, ?)",
        [
            params["displayName"].as_str().unwrap(),
            params["url"].as_str().unwrap()
        ],
    ).map_err(|e| format!("Error executing statement: {}", e))?;
    Ok(serde_json::json!({"status": "saved", "message": "Config saved"}))
}


#[tauri::command]
async fn remove_nostr_relay(_app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    match remove_nostr_relay_impl(params).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in remove_nostr_relay: {}", e)),
    }
}

async fn remove_nostr_relay_impl(params: serde_json::Value) -> Result<serde_json::Value, String> {
    let conn = Connection::open("../resources/slime.db").map_err(|e| format!("Error opening database: {}", e))?;
    let _ = conn.execute(
        "DELETE FROM nostrRelays WHERE id = ?",
        [params["id"].as_i64().unwrap() as i32],
    ).map_err(|e| format!("Error executing statement: {}", e))?;
    Ok(serde_json::json!({"status": "saved", "message": "Config saved"}))
}


#[tauri::command]
async fn get_identities(_app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    match get_identities_impl().await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in get_identities: {}", e)),
    }
}

async fn get_identities_impl() -> Result<serde_json::Value, String> {
    let conn = Connection::open("../resources/slime.db").map_err(|e| format!("Error opening database: {}", e))?;
    
    let mut stmt = conn.prepare("SELECT * FROM identities").map_err(|e| format!("Error preparing statement: {}", e))?;

    let mut identities = Vec::new();
    let rows = stmt.query_map([], |row| {
        let did: String = row.get(0)?;
        let active_proof: String = row.get(1)?;
        let display_name: String = row.get(2)?;
        let avatar: String = row.get(3)?;
        let bio: String = row.get(4)?;
        let location: String = row.get(5)?;
        let languages: String = row.get(6)?;
        let links: String = row.get(7)?;
        let proofs: String = row.get(8)?;

        Ok(serde_json::json!({
            "did": did,
            "activeProof": active_proof,
            "displayName": display_name,
            "avatar": avatar,
            "bio": bio,
            "location": location,
            "languages": languages,
            "links": links,
            "proofs": proofs
        }))
    }).map_err(|e| format!("Error During Query: {}", e))?;
    for row in rows {
        match row {
            Ok(identity) => identities.push(identity),
            Err(e) => return Err(format!("Error processing row: {}", e)),
        }
    }
    Ok(serde_json::json!(identities))
}

#[tauri::command]
async fn add_identity(_app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    match add_identity_impl(params).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in add_identity: {}", e)),
    }
}

async fn add_identity_impl(params: serde_json::Value) -> Result<serde_json::Value, String> {
    let conn = Connection::open("../resources/slime.db").map_err(|e| format!("Error opening database: {}", e))?;
    println!("add identity params: {:?}", params);
    let _ = conn.execute(
        "INSERT OR REPLACE INTO identities (
            did, activeProof, displayName, avatar, bio, location, languages, links, proofs
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        [
            params["did"].as_str().unwrap(),
            params["activeProof"].as_str().unwrap(),
            params["displayName"].as_str().unwrap(),
            params["avatar"].as_str().unwrap(),
            params["bio"].as_str().unwrap(),
            params["location"].as_str().unwrap(),
            params["languages"].as_str().unwrap(),
            params["links"].as_str().unwrap(),
            params["proofs"].as_str().unwrap()
        ],
    ).map_err(|e| format!("Error executing statement: {}", e))?;

    Ok(serde_json::json!({"status": "saved", "message": "Config saved"}))
}

#[tauri::command]
async fn remove_identity(_app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    match remove_identity_impl(params).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in remove_identity: {}", e)),
    }
}

async fn remove_identity_impl(params: serde_json::Value) -> Result<serde_json::Value, String> {
    let conn = Connection::open("../resources/slime.db").map_err(|e| format!("Error opening database: {}", e))?;
    let _ = conn.execute(
        "DELETE FROM identities WHERE did = ?",
        [params["did"].as_str().unwrap()],
    ).map_err(|e| format!("Error executing statement: {}", e))?;

    Ok(serde_json::json!({"status": "saved", "message": "Config saved"}))
}


#[tauri::command]
async fn set_active_identity(_app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    match set_active_identity_impl(params).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in set_active_identity: {}", e)),
    }
}

async fn set_active_identity_impl(params: serde_json::Value) -> Result<serde_json::Value, String> {
    let conn = Connection::open("../resources/slime.db").map_err(|e| format!("Error opening database: {}", e))?;
    let _ = conn.execute(
        "UPDATE activeConfig SET did = ?, activeProof = (SELECT activeProof FROM identities WHERE did = ?) WHERE id = 1",
        [
            params["did"].as_str().unwrap(),
            params["did"].as_str().unwrap()
        ],
    ).map_err(|e| format!("Error executing statement: {}", e))?;
    Ok(serde_json::json!({"status": "saved", "message": "Config saved"}))
}

#[tauri::command]
async fn get_install_paths(_app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    match get_install_paths_impl().await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in get_install_paths: {}", e)),
    }
}

async fn get_install_paths_impl() -> Result<serde_json::Value, String> {
    let conn = Connection::open("../resources/slime.db").map_err(|e| format!("Error opening database: {}", e))?;
    
    let mut stmt = conn.prepare("SELECT * FROM installPaths").map_err(|e| format!("Error preparing statement: {}", e))?;

    let mut paths = Vec::new();

    let rows = stmt.query_map([], |row| {
        let id: i32 = row.get(0)?;
        let display_name: String = row.get(1)?;
        let path: String = row.get(2)?;

        Ok(serde_json::json!({
            "id": id,
            "displayName": display_name,
            "path": path
        }))
    }).map_err(|e| format!("Error During Query: {}", e))?;

    for row in rows {
        match row {
            Ok(path) => paths.push(path),
            Err(e) => return Err(format!("Error processing row: {}", e)),
        }
    }

    Ok(serde_json::json!(paths))
}

#[tauri::command]
async fn add_install_path(_app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    match add_install_path_impl(params).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in add_install_path: {}", e)),
    }
}

async fn add_install_path_impl(params: serde_json::Value) -> Result<serde_json::Value, String> {
    let conn = Connection::open("../resources/slime.db").map_err(|e| format!("Error opening database: {}", e))?;
    println!("add install_path: {:?}", params);
    let _ = conn.execute(
        "INSERT OR REPLACE INTO installPaths (
            displayName, path
        )
        VALUES (?, ?)",
        [
            params["displayName"].as_str().unwrap(),
            params["path"].as_str().unwrap()
        ],
    ).map_err(|e| format!("Error executing statement: {}", e))?;
    Ok(serde_json::json!({"status": "saved", "message": "Config saved"}))
}

#[tauri::command]
async fn remove_install_path(_app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    match remove_install_path_impl(params).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in remove_install_path: {}", e)),
    }
}

async fn remove_install_path_impl(params: serde_json::Value) -> Result<serde_json::Value, String> {
    let conn = Connection::open("../resources/slime.db").map_err(|e| format!("Error opening database: {}", e))?;
    let _ = conn.execute(
        "DELETE FROM installPaths WHERE id = ?",
        [params["id"].as_i64().unwrap() as i32],
    ).map_err(|e| format!("Error executing statement: {}", e))?;
    Ok(serde_json::json!({"status": "saved", "message": "Config saved"}))
}

#[tauri::command]
async fn set_active_install_path(_app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    match set_active_install_path_impl(params).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in set_active_install_path: {}", e)),
    }
}

async fn set_active_install_path_impl(params: serde_json::Value) -> Result<serde_json::Value, String> {
    let conn = Connection::open("../resources/slime.db").map_err(|e| format!("Error opening database: {}", e))?;
    let _ = conn.execute(
        "UPDATE activeConfig SET installPath = (SELECT path FROM installPaths WHERE id = ?), installPathDisplayName = (SELECT displayName FROM installPaths WHERE id = ?) WHERE id = 1",
        [
            params["id"].as_i64().unwrap() as i32,
            params["id"].as_i64().unwrap() as i32
        ],
    ).map_err(|e| format!("Error executing statement: {}", e))?;
    Ok(serde_json::json!({"status": "saved", "message": "Config saved"}))
}

#[tauri::command]
async fn get_torrent_paths(_app: tauri::AppHandle) -> Result<serde_json::Value, String> {
    match get_torrent_paths_impl().await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in get_torrent_paths: {}", e)),
    }
}

async fn get_torrent_paths_impl() -> Result<serde_json::Value, String> {
    let conn = Connection::open("../resources/slime.db").map_err(|e| format!("Error opening database: {}", e))?;
    
    let mut stmt = conn.prepare("SELECT * FROM torrentPaths").map_err(|e| format!("Error preparing statement: {}", e))?;

    let mut paths = Vec::new();
    let rows = stmt.query_map([], |row| {
        let id: i32 = row.get(0)?;
        let display_name: String = row.get(1)?;
        let path: String = row.get(2)?;

        Ok(serde_json::json!({
            "id": id,
            "displayName": display_name,
            "path": path
        }))
    }).map_err(|e| format!("Error During Query: {}", e))?;
    for row in rows {
        match row {
            Ok(path) => paths.push(path),
            Err(e) => return Err(format!("Error processing row: {}", e)),
        }
    }
    Ok(serde_json::json!(paths))
}

#[tauri::command]
async fn add_torrent_path(_app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    match add_torrent_path_impl(params).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in add_torrent_path: {}", e)),
    }
}

async fn add_torrent_path_impl(params: serde_json::Value) -> Result<serde_json::Value, String> {
    let conn = Connection::open("../resources/slime.db").map_err(|e| format!("Error opening database: {}", e))?;
    
    let _ = conn.execute(
        "INSERT OR REPLACE INTO torrentPaths (
            displayName, path
        )
        VALUES (?, ?)",
        [
            params["displayName"].as_str().unwrap(),
            params["path"].as_str().unwrap()
        ],
    ).map_err(|e| format!("Error executing statement: {}", e))?;
    Ok(serde_json::json!({"status": "saved", "message": "Config saved"}))
}

#[tauri::command]
async fn remove_torrent_path(_app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    match remove_torrent_path_impl(params).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in remove_torrent_path: {}", e)),
    }
}

async fn remove_torrent_path_impl(params: serde_json::Value) -> Result<serde_json::Value, String> {
    let conn = Connection::open("../resources/slime.db").map_err(|e| format!("Error opening database: {}", e))?;
    let _ = conn.execute(
        "DELETE FROM torrentPaths WHERE id = ?",
        [params["id"].as_i64().unwrap() as i32],
    ).map_err(|e| format!("Error executing statement: {}", e))?;
    Ok(serde_json::json!({"status": "saved", "message": "Config saved"}))
}

#[tauri::command]
async fn set_active_torrent_path(_app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    match set_active_torrent_path_impl(params).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in set_active_torrent_path: {}", e)),
    }
}

async fn set_active_torrent_path_impl(params: serde_json::Value) -> Result<serde_json::Value, String> {
    let conn = Connection::open("../resources/slime.db").map_err(|e| format!("Error opening database: {}", e))?;
    println!("set_active_torrent_path: {:?}", params);
    let _ = conn.execute(
        "UPDATE activeConfig SET torrentPath = (SELECT path FROM torrentPaths WHERE id = ?), torrentPathDisplayName = (SELECT displayName FROM torrentPaths WHERE id = ?) WHERE id = 1",
        [
            params["id"].as_i64().unwrap() as i32,
            params["id"].as_i64().unwrap() as i32
        ],
    ).map_err(|e| format!("Error executing statement: {}", e))?;
    Ok(serde_json::json!({"status": "saved", "message": "Config saved"}))
}

#[tauri::command]
async fn set_minting_data_path(_app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    match set_minting_data_path_impl(params).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in set_minting_data_path: {}", e)),
    }
}

async fn set_minting_data_path_impl(params: serde_json::Value) -> Result<serde_json::Value, String> {
    let conn = Connection::open("../resources/slime.db").map_err(|e| format!("Error opening database: {}", e))?;
    let _ = conn.execute(
        "UPDATE activeConfig SET mintingDataPath = (SELECT path FROM mintingDataPaths WHERE id = ?) WHERE id = 1",
        [
            params["id"].as_i64().unwrap() as i32
        ],
    ).map_err(|e| format!("Error executing statement: {}", e))?;
    Ok(serde_json::json!({"status": "saved", "message": "Config saved"}))
}

async fn start() {
    let conn = Connection::open("../resources/slime.db").unwrap();
    let _ = conn.execute(
        "CREATE TABLE IF NOT EXISTS media (
                productId TEXT PRIMARY KEY,
                contentRatings JSON,
                descriptions JSON,
                credits JSON,
                childProducts JSON,
                lastUpdated INTEGER,
                lastUpdatedContent INTEGER,
                mediaType TEXT,
                nostrEventId TEXT,
                images JSON,
                videos JSON,
                donationAddress TEXT,
                parentProductId TEXT,
                publisherDid TEXT,
                releaseStatus TEXT,
                supportContact TEXT,
                tags JSON,
                titles JSON,
                files JSON
            )",
        [],
    );
    let _ = conn.execute(
            "CREATE TABLE IF NOT EXISTS identities (
                did TEXT PRIMARY KEY,
                activeProof JSON,
                displayName TEXT,
                avatar TEXT,
                bio TEXT,
                location TEXT,
                languages JSON,
                links JSON,
	            proofs JSON
            )",
            [],
    );
        let _ = conn.execute(
            "CREATE TABLE IF NOT EXISTS marketplaces (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                displayName TEXT,
                url TEXT
            )",
            [],
        );
        let _ = conn.execute(
            "CREATE TABLE IF NOT EXISTS installPaths (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                displayName TEXT,
                path TEXT
            )",
            [],
        );
        let _ = conn.execute(
            "CREATE TABLE IF NOT EXISTS torrentPaths (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                displayName TEXT,
                path TEXT
            )",
            [],
        );
        let _ = conn.execute(
            "CREATE TABLE IF NOT EXISTS nostrRelays (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                displayName TEXT,
                url TEXT
            )",
            [],
        );
        let _ = conn.execute(
            "CREATE TABLE IF NOT EXISTS nostrKeys (
                publicKey TEXT PRIMARY KEY,
                privateKey TEXT,
                proof TEXT,
                secured INTEGER
            )",
            [],
        );

    let _ = conn.execute(
        "CREATE TABLE IF NOT EXISTS activeConfig (
                id INTEGER PRIMARY KEY,
                did TEXT,
                activeProof JSON,
                marketplaceDisplayName TEXT,
                marketplaceUrl TEXT,
                torrentClientPort INTEGER,
                languages JSON,
                installPath TEXT,
                installPathDisplayName TEXT,
                torrentPath TEXT,
                torrentPathDisplayName TEXT,
                mintingDataPath TEXT
            )",
        [],
    );
    let _ = conn.execute(
        "INSERT OR IGNORE INTO activeConfig (id, installPath, torrentPath, mintingDataPath, torrentClientPort, languages, installPathDisplayName, torrentPathDisplayName) VALUES (1, './installs', './torrents', './minting', 5235, '[\"english\"]', 'Default', 'Default')",
        [],
    );

    let _ = conn.execute(
        "CREATE TABLE IF NOT EXISTS mintingConfig (
                id TEXT PRIMARY KEY,
                iconUri TEXT,
                bannerUri TEXT,
                quantity INTEGER,
                batchSize INTEGER,
                edition Text,
                receiveAddress TEXT,
                royaltyAddress TEXT,
                royaltyPercentage INTEGER,
                displayImageUris JSON,
                metadataUris JSON,
                licenseUris JSON,
                generateOfferFiles TEXT,
                sensitiveContent TEXT,
                salePrice REAL,
                saleAsset JSON,
                mintingFee INTEGER
            )",
        [],
    );

    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let show = CustomMenuItem::new("show".to_string(), "Open");
    let tray_menu = SystemTrayMenu::new()
        .add_item(show)
        .add_item(hide)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    // let (mut rx, mut _child) = Command::new_sidecar("torrentclient")
    //     .expect("failed to create `torrentclient` binary command")
    //     .args(&["../resources/slime.db"])
    //     .spawn()
    //     .expect("Failed to spawn sidecar");

    let (mut rx2, mut _child) = Command::new_sidecar("streamaudio")
        .expect("failed to create `streamaudio` binary command")
        .args(&["--no-vision", "--no-history", "--no-ai"])
        .spawn()
        .expect("Failed to spawn sidecar");

    // tauri::async_runtime::spawn(async move {
    //     // read events such as stdout
    //     while let Some(event) = rx.recv().await {
    //         println!("python event: {:?}", event);
    //         if let CommandEvent::Stdout(line) = event {
    //             println!("stdout: {}", line);
    //         } else if let CommandEvent::Stderr(line) = event {
    //             println!("stderr: {}", line);
    //         }
    //     }
    //     });

    tauri::async_runtime::spawn(async move {
        print!("Starting streamaudio sidecar...");
        // read events such as stdout
        while let Some(event) = rx2.recv().await {
            if let CommandEvent::Stdout(line) = event {
                println!("stdout: {}", line);
            } else if let CommandEvent::Stderr(line) = event {
                println!("stderr: {}", line);
            }
        }
    });

    tauri::Builder::default()
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
            get_minting_config,
            set_minting_config,
            get_operating_system,
            slime_nostr::add_nostr_keypair,
            slime_nostr::has_nostr_private_key,
            slime_nostr::sign_nostr_message,
            get_local_media_metadata,
            save_local_media_metadata,
            get_marketplaces,
            add_marketplace,
            remove_marketplace,
            set_active_marketplace,
            get_nostr_relays,
            add_nostr_relay,
            remove_nostr_relay,
            get_identities,
            add_identity,
            remove_identity,
            set_active_identity,
            get_install_paths,
            add_install_path,
            remove_install_path,
            set_active_install_path,
            get_torrent_paths,
            add_torrent_path,
            remove_torrent_path,
            set_active_torrent_path,
            set_minting_data_path,
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