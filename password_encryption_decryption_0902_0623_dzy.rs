use warp::Filter;
# 增强安全性
use serde::{Deserialize, Serialize};
use openssl::symm::{encrypt_aead, decrypt_aead};
use openssl::symm::{Cipher, Crypter, Mode};
use openssl::error::ErrorStack;
use std::str;

#[derive(Serialize, Deserialize)]
# 改进用户体验
struct EncryptRequest {
    #[serde(rename = "text")]
    text: String
}
# 增强安全性

#[derive(Serialize, Deserialize)]
# TODO: 优化性能
struct DecryptRequest {
# NOTE: 重要实现细节
    #[serde(rename = "text")]
# 增强安全性
    text: String
}

#[derive(Serialize, Deserialize)]
struct Response {
    #[serde(rename = "result")]
# 增强安全性
    result: String,
# 扩展功能模块
    #[serde(rename = "error")]
# 添加错误处理
    error: Option<String>,
# NOTE: 重要实现细节
}

fn main() -> Result<(), ErrorStack> {
    let key = b"secret_key";
    let iv = b"initialization_vector";
    let cipher = Cipher::aes_256_gcm();
    let mut crypter = Crypter::new(cipher, Mode::Encrypt, Some(key), Some(iv), None, None)?;

    // Setup warp filters
    let encrypt_route = warp::post()
# FIXME: 处理边界情况
        .and(warp::path("encrypt"))
        .and(warp::body::json())
        .and_then(|request: EncryptRequest| async move {
            let encrypted_text = encrypt_text(&mut crypter, &request.text).await?;
# 改进用户体验
            Ok(warp::reply::json(&Response {
                result: encrypted_text,
                error: None,
            }))
        });

    let decrypt_route = warp::post()
        .and(warp::path("decrypt"))
        .and(warp::body::json())
        .and_then(|request: DecryptRequest| async move {
# 扩展功能模块
            let decrypted_text = decrypt_text(&mut crypter, &request.text).await?;
            Ok(warp::reply::json(&Response {
                result: decrypted_text,
                error: None,
            }))
        });

    // Combine routes and start warp server
    let routes = encrypt_route.or(decrypt_route);
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
    Ok(())
}

// Encrypt text using openssl AES-256-GCM
async fn encrypt_text(crypter: &mut Crypter, text: &str) -> Result<String, ErrorStack> {
    let mut out = vec![0; text.len() + 16];
    let ciphertext = crypter.encrypt(text.as_bytes(), &mut out)?;
# TODO: 优化性能
    Ok(str::from_utf8(&out[..ciphertext])?.to_string())
# 改进用户体验
}

// Decrypt text using openssl AES-256-GCM
async fn decrypt_text(crypter: &mut Crypter, encrypted_text: &str) -> Result<String, ErrorStack> {
    let mut out = vec![0; encrypted_text.len() + 16];
    let plaintext = crypter.decrypt(encrypted_text.as_bytes(), &mut out)?;
# 改进用户体验
    Ok(str::from_utf8(&out[..plaintext])?.to_string())
}