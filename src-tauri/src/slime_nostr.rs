use std::fs::File;
use std::io::Read;
use std::fs::write;

use hex::ToHex;
use secp256k1::{Secp256k1, SecretKey, Keypair, Message};
use serde_json::json;


#[tauri::command]
pub async fn add_nostr_keypair(app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    match add_nostr_keypair_impl(app, params).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in add_nostr_keypair: {}", e)),
    }
}

async fn add_nostr_keypair_impl(app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    println!("adding keys params: {:?}", params);

    let mut file = File::open(app.path_resolver().resolve_resource("../resources/nostr-keys.json").expect("failed to resolve resource")).map_err(|e| format!("Error opening file: {}", e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| format!("Error reading file: {}", e))?;
    
    let mut keys: serde_json::Value = serde_json::from_str(&contents).map_err(|e| format!("Error parsing JSON: {}", e))?;


    println!("keys 1: {:?}", keys["keys"].as_array_mut());

    keys["keys"].as_array_mut().unwrap().push(params);

    println!("keys 2: {:?}", keys);

    let updated_contents = serde_json::to_string(&keys).map_err(|e| format!("Error serializing JSON: {}", e))?;
    write(app.path_resolver().resolve_resource("../resources/nostr-keys.json").expect("failed to resolve resource"), updated_contents).map_err(|e| format!("Error writing file: {}", e))?;
    
    Ok(serde_json::json!({"message": "Keys saved"}))
}


#[tauri::command]
pub async fn has_nostr_private_key(app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    match has_nostr_private_key_impl(app, params).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in has_nostr_private_key: {}", e)),
    }
}

async fn has_nostr_private_key_impl(app: tauri::AppHandle, params: serde_json::Value) -> Result<serde_json::Value, String> {
    let mut file = File::open(app.path_resolver().resolve_resource("../resources/nostr-keys.json").expect("failed to resolve resource")).map_err(|e| format!("Error opening file: {}", e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| format!("Error reading file: {}", e))?;
    
    let keys: serde_json::Value = serde_json::from_str(&contents).map_err(|e| format!("Error parsing JSON: {}", e))?;
    let keys = &keys["keys"];

    for key in keys.as_array().unwrap() {
        if key["publicKey"].as_str().unwrap() == params["publicKey"].as_str().unwrap() {
            println!("private key found for: {:?}", params["publicKey"].as_str().unwrap());
            return Ok(json!({"hasPrivateKey": true}));
        }
    }
    println!("no private key found for: {:?}", params["publicKey"].as_str().unwrap());

    Ok(json!({"hasPrivateKey": false}))
}


#[tauri::command]
pub async fn sign_nostr_message(app: tauri::AppHandle, message: serde_json::Value) -> Result<serde_json::Value, String> {
    match sign_nostr_message_impl(app, message).await {
        Ok(response) => Ok(response),
        Err(e) => Err(format!("Error in sign_nostr_message: {}", e)),
    }
}

async fn sign_nostr_message_impl(app: tauri::AppHandle, message: serde_json::Value) -> Result<serde_json::Value, String> {
    
    println!("signing message: {:?}", message);
    
    let mut file = File::open(app.path_resolver().resolve_resource("../resources/nostr-keys.json").expect("failed to resolve resource")).map_err(|e| format!("Error opening file: {}", e))?;
    let mut contents_keys = String::new();
    file.read_to_string(&mut contents_keys).map_err(|e| format!("Error reading file: {}", e))?;
    

    let mut file = File::open(app.path_resolver().resolve_resource("../resources/slime-config.json").expect("failed to resolve resource")).map_err(|e| format!("Error opening file: {}", e))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).map_err(|e| format!("Error reading file: {}", e))?;
    
    // Parse the contents as JSON
    let config: serde_json::Value = serde_json::from_str(&contents).map_err(|e| format!("Error parsing JSON: {}", e))?;
    let keychain: serde_json::Value = serde_json::from_str(&contents_keys).map_err(|e| format!("Error parsing JSON: {}", e))?;
    let keys = &keychain["keys"];

    let mut sig = serde_json::Value::Null;

    println!("config: {:?}", config["activeIdentity"]);

    println!("looking for key: {:?}", config["activeIdentity"]["currentNostrPublicKey"]);
    println!("keys: {:?}", keys);
    for key in keys.as_array().unwrap() {
        println!("11111: {:?}", key["publicKey"].as_str().unwrap());
        println!("22222: {:?}", config["activeIdentity"]["currentNostrPublicKey"].as_str().unwrap());
        if key["publicKey"].as_str().unwrap() == config["activeIdentity"]["currentNostrPublicKey"].as_str().unwrap() {
            println!("found key: {:?}", key);
            let secp = Secp256k1::new();
            let secret_key = SecretKey::from_slice(&hex::decode(key["privateKey"].as_str().unwrap()).unwrap()).unwrap();
            let keypair = Keypair::from_secret_key(&secp, &secret_key);
            let message_to_sign = Message::from_digest_slice(hex::decode(message.as_str().unwrap()).unwrap().as_slice()).unwrap();
            println!("message_to_sign: {:?}", message_to_sign);
            let real_sig = secp.sign_schnorr_no_aux_rand(&message_to_sign, &keypair);
            // let real_sig = secp.sign_ecdsa(&message_to_sign, &secret_key);
            sig = serde_json::Value::String(real_sig.serialize().encode_hex::<String>());
        }
    }

    println!("sig final: {:?}", sig);
    Ok(json!({"signature": sig, "message": message, "status": "signed"}))
}