use hex::ToHex;
use secp256k1::{Secp256k1, SecretKey, Keypair, Message};
use serde_json::json;
use rusqlite::{Connection, Result};

#[tauri::command]
pub async fn add_nostr_keypair(app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    match add_nostr_keypair_impl(app, params).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in add_nostr_keypair: {}", e)),
    }
}

async fn add_nostr_keypair_impl(_app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    println!("adding keys params: {:?}", params);

    let conn = Connection::open("../resources/slime.db").map_err(|e| format!("Error opening database: {}", e))?;
    let _ = conn.execute("INSERT INTO nostrKeys (publicKey, privateKey, proof) VALUES (?, ?, ?)", &[params["publicKey"].as_str().unwrap(), params["privateKey"].as_str().unwrap(), params["proof"].as_str().unwrap()]).map_err(|e| format!("Error inserting into database: {}", e))?;
    Ok(serde_json::json!({"message": "Keys saved", "status": "success" }))
}


#[tauri::command]
pub async fn has_nostr_private_key(app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    match has_nostr_private_key_impl(app, params).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in has_nostr_private_key: {}", e)),
    }
}

async fn has_nostr_private_key_impl(_app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    let conn = Connection::open("../resources/slime.db").map_err(|e| format!("Error opening database: {}", e))?;
    let mut stmt = conn.prepare("SELECT * FROM nostrKeys WHERE publicKey = ?").map_err(|e| format!("Error preparing statement: {}", e))?;
    
    let keys = stmt.query_row([params["publicKey"].as_str().unwrap()], |row| {
        let public_key: Option<String> = row.get(0)?;
        let private_key: Option<String> = row.get(1)?;
        let proof: Option<String> = row.get(2)?;
        Ok(serde_json::json!({
            "publicKey": public_key,
            "privateKey": private_key,
            "proof": proof
        }))
    }).map_err(|e| format!("Error querying database: {}", e))?;

    if keys["privateKey"].is_null() {
        return Ok(serde_json::json!({"message": "No private key found", "status": "success", "hasPrivateKey": false }));
    } else {
        return Ok(serde_json::json!({"message": "Private key found", "status": "success", "hasPrivateKey": true }));
    }
}


#[tauri::command]
pub async fn sign_nostr_message(app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    match sign_nostr_message_impl(app, params).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in sign_nostr_message: {}", e)),
    }
}

async fn sign_nostr_message_impl(_app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    
    println!("signing message: {:?}", params);
    
    let conn = Connection::open("../resources/slime.db").map_err(|e| format!("Error opening database: {}", e))?;

    let mut stmt = conn.prepare("SELECT * FROM nostrKeys WHERE publicKey = ?").map_err(|e| format!("Error preparing statement: {}", e))?;
    let keys = stmt.query_row([params["publicKey"].as_str().unwrap()], |row| {
        let public_key: Option<String> = row.get(0)?;
        let private_key: Option<String> = row.get(1)?;
        let proof: Option<String> = row.get(2)?;
        Ok(serde_json::json!({
            "publicKey": public_key,
            "privateKey": private_key,
            "proof": proof
        }))
    }).unwrap();

    println!("keys: {:?}", keys);

    let secp = Secp256k1::new();
    let secret_key = SecretKey::from_slice(&hex::decode(keys["privateKey"].as_str().unwrap()).unwrap()).unwrap();
    let keypair = Keypair::from_secret_key(&secp, &secret_key);
    let message_to_sign = Message::from_digest_slice(hex::decode(params["message"].as_str().unwrap()).unwrap().as_slice()).unwrap();
    println!("message_to_sign: {:?}", message_to_sign);
    let real_sig = secp.sign_schnorr_no_aux_rand(&message_to_sign, &keypair);
    // let real_sig = secp.sign_ecdsa(&message_to_sign, &secret_key);
    let sig = serde_json::Value::String(real_sig.serialize().encode_hex::<String>());

    println!("sig final: {:?}", sig);
    Ok(json!({"signature": sig, "message": params["message"].as_str().unwrap(), "status": "signed"}))
}