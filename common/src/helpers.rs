use base64::{engine::general_purpose, Engine as _};
use openssl::error::ErrorStack;
use openssl::symm::{Cipher, Crypter, Mode};

pub fn get_media_url(encrypted_media_url: &str, high_quality: bool) -> Result<String, ErrorStack> {
    let media_url = decrypt_url(encrypted_media_url)?;
    if high_quality {
        Ok(media_url.replace("_96.mp4", "_320.mp4"))
    } else {
        Ok(media_url)
    }
}

fn decrypt_url(url: &str) -> Result<String, ErrorStack> {
    let key = b"38346591";
    let encrypted_data_b64 = url.trim().as_bytes();

    let encrypted_data = general_purpose::STANDARD
        .decode(&encrypted_data_b64)
        .unwrap();

    let cipher = Cipher::des_ecb();
    let mut decrypter = Crypter::new(cipher, Mode::Decrypt, key, None)?;

    let mut decrypted_data = vec![0; encrypted_data.len() + cipher.block_size()];
    let len = decrypter.update(&encrypted_data, &mut decrypted_data)?;
    let len = len + decrypter.finalize(&mut decrypted_data[len..])?;
    decrypted_data.truncate(len);

    let decrypted_url = std::str::from_utf8(&decrypted_data).unwrap();
    Ok(decrypted_url.to_string())
}
