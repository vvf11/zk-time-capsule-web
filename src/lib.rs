use sp1_sdk::{ProverClient, SP1Stdin};
use wasm_bindgen::prelude::*;
use js_sys::Uint8Array;
use web_sys::File;

fn is_locked(current_date: u64, unlock_date: u64) -> bool {
    current_date < unlock_date
}

#[wasm_bindgen]
pub fn generate_proof(unlock_date: u64, message: String, photo_data: Uint8Array) -> Result<Vec<u8>, JsValue> {
    let current_date = 20250309;

    if !is_locked(current_date, unlock_date) {
        return Err(JsValue::from_str("Дата открытия должна быть в будущем!"));
    }

    let mut stdin = SP1Stdin::new();
    stdin.write(&current_date);
    stdin.write(&unlock_date);
    stdin.write(&message);
    let photo_bytes = photo_data.to_vec();
    stdin.write(&photo_bytes);

    let locked = true;
    let mut stdout = Vec::new();
    stdout.extend_from_slice(&locked.to_le_bytes());

    let client = ProverClient::new();
    let (_, proof) = client
        .prove(&stdin, stdout)
        .map_err(|e| JsValue::from_str(&format!("Ошибка генерации: {:?}", e)))?;

    Ok(proof)
}
