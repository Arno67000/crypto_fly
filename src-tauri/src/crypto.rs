use age::secrecy::Secret;
use std::io::{Read, Write};
use std::{env, fs};

#[tauri::command]
pub fn handle_file_encryption(pass: &str) -> u8 {
    let key = pass;

    let current_dir = match env::current_dir() {
        Ok(p) => p,
        _ => return 5,
    };

    let filename = String::from("enc.zip");
    let codename = String::from("SecuredVault");

    let file_path = current_dir.join(&filename);
    let enc_file_path = current_dir.join(&codename);

    let enc_file_path_clone = enc_file_path.clone();

    match fs::read(file_path.clone()) {
        Ok(file) => {
            let content = match encrypt(key, file) {
                Ok(content) => content,
                Err(_) => return 7,
            };
            match fs::write(file_path.clone(), content) {
                Ok(()) => (),
                Err(_) => return 6,
            }
            match fs::rename(file_path, enc_file_path) {
                Ok(_) => 0,
                Err(_) => return 6,
            }
        }
        Err(_err) => match fs::read(enc_file_path) {
            Ok(file) => {
                let content = match decrypt(key, file) {
                    Ok(content) => content,
                    Err(_) => return 8,
                };
                match fs::write(enc_file_path_clone.clone(), content) {
                    Ok(()) => (),
                    Err(_) => return 6,
                }
                match fs::rename(enc_file_path_clone, file_path) {
                    Ok(_) => return 0,
                    Err(_) => return 6,
                }
            }
            Err(_err) => 4,
        },
    }
}

fn decrypt(key: &str, file: Vec<u8>) -> Result<Vec<u8>, std::io::Error> {
    let decryptor = match age::Decryptor::new(&file[..]) {
        Ok(decryptor) => decryptor,
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Decryptor build failed",
            ))
        }
    };
    let pass_decryptor = match decryptor {
        age::Decryptor::Passphrase(d) => d,
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Decryptor failed",
            ))
        }
    };

    let mut decrypted = vec![];
    let mut reader = match pass_decryptor.decrypt(&Secret::new(key.to_owned()), None) {
        Ok(reader) => reader,
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Decrypt failed",
            ))
        }
    };
    reader.read_to_end(&mut decrypted)?;

    Ok(decrypted)
}

fn encrypt(key: &str, file: Vec<u8>) -> Result<Vec<u8>, std::io::Error> {
    let encryptor = age::Encryptor::with_user_passphrase(Secret::new(key.to_owned()));
    let mut encrypted = vec![];

    let mut writer = match encryptor.wrap_output(&mut encrypted) {
        Ok(writer) => writer,
        _ => {
            return Err(std::io::Error::new(
                std::io::ErrorKind::WouldBlock,
                "Writer failed to stop",
            ))
        }
    };
    writer.write_all(&file)?;
    writer.finish()?;

    Ok(encrypted)
}
