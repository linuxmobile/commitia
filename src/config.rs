use block_padding::{Pkcs7, UnpadError};
use cbc::{cipher::KeyIvInit, Decryptor, Encryptor};
use dirs;
use hex_literal::hex;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::path::PathBuf;

use aes::cipher::{BlockDecryptMut, BlockEncryptMut};

type Aes256Cbc = Encryptor<aes::Aes256>;
type Aes256CbcDecryptor = Decryptor<aes::Aes256>;

const KEY: [u8; 32] = hex!("000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f");
const IV: [u8; 16] = hex!("1a1b1c1d1e1f20212223242526272829");

#[derive(Serialize, Deserialize)]
pub struct Config {
  pub token: String,
}

pub fn config_exists() -> bool {
  let config_path = match get_config_path() {
    Ok(path) => path,
    Err(_) => return false,
  };
  Path::new(&config_path).exists()
}

fn encrypt(data: &[u8]) -> Vec<u8> {
  let cipher = Aes256Cbc::new(&KEY.into(), &IV.into());
  let block_size = 16;
  let mut buffer = data.to_vec();
  let padding_len = block_size - (buffer.len() % block_size);
  buffer.extend(std::iter::repeat(padding_len as u8).take(padding_len));
  cipher
    .encrypt_padded_mut::<Pkcs7>(&mut buffer, data.len())
    .unwrap();
  buffer
}

fn decrypt(data: &[u8]) -> Result<Vec<u8>, UnpadError> {
  let cipher = Aes256CbcDecryptor::new(&KEY.into(), &IV.into());
  let mut buffer = data.to_vec();
  let pt = cipher.decrypt_padded_mut::<Pkcs7>(&mut buffer).unwrap();
  Ok(pt.to_vec())
}

fn get_config_path() -> io::Result<PathBuf> {
  let mut config_dir = dirs::home_dir()
    .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Home directory not found"))?;
  config_dir.push(".commitia");
  if !config_dir.exists() {
    fs::create_dir(&config_dir)?;
  }
  config_dir.push("config.json");
  Ok(config_dir)
}

pub fn save_token(token: &str) -> io::Result<()> {
  let encrypted_token = encrypt(token.as_bytes());
  let config = Config {
    token: hex::encode(encrypted_token),
  };
  let config_json = serde_json::to_string(&config)?;
  let config_path = get_config_path()?;
  let mut file = fs::File::create(config_path)?;
  file.write_all(config_json.as_bytes())?;
  Ok(())
}

pub fn load_token() -> io::Result<String> {
  let config_path = get_config_path()?;
  let config_json = fs::read_to_string(config_path)?;
  let config: Config = serde_json::from_str(&config_json)?;
  let encrypted_token =
    hex::decode(config.token).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
  let decrypted_token = decrypt(&encrypted_token)
    .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;
  String::from_utf8(decrypted_token).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}
