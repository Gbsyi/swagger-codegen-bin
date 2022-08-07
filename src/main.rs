extern crate tokio;
extern crate reqwest;
extern crate serde_json;
extern crate core;
extern crate zip;
extern crate bytes;

use std::{fs, io};
use std::io::ErrorKind;
use std::path::Path;
use serde_json::json;
use serde_json::value::RawValue;
use bytes::Bytes;

struct CodegenConfig {
    api_url: String,
    lang: String,
    gen_type: String,
    folder: String
}

#[tokio::main]
async fn main() {
    let result = generate_code().await;
    if let Err(msg) = result {
        println!("\x1b[0;31mError:\x1b[0m {}", msg);
        return;
    }
}

async fn generate_code<'a>() -> Result<(), String> {
    let config = read_config()?;

    println!("Successfully got config data");
    println!("\tApi Url: {}", &config.api_url);
    println!("\tLanguage: {}", &config.lang);
    println!("\tType: {}", &config.gen_type);
    println!("Trying to get api info from server");
    let api_info = download_api_info(&config).await;
    println!("Successfully got api info");
    println!("Trying to download generated code");
    let zip_bytes = download_generated_archive(&config, &api_info).await?;
    println!("Successfully downloaded archive");
    println!("Trying to extract archive to folder");
    unzip_to_folder(zip_bytes, &config.folder)?;
    println!("\x1b[0;32mSuccess\x1b[0m");
    Ok(())
}

fn read_config<'a>() -> Result<CodegenConfig, String> {
    let content_result = fs::read_to_string("./src/codegen.config");
    if let Err(err) = content_result {
        return match err.kind() {
            ErrorKind::NotFound => Err(String::from("Can't find \"codegen.config\" file")),
            ErrorKind::PermissionDenied => Err(String::from("Can't read config file")),
            _ => {
                let error: String = format!("Unknown error ({})", err.to_string());
                return Err(error);
            }
        }
    }

    let lines = content_result.unwrap();
    let lines = lines.lines();
    let mut api_url = "";
    let mut lang= "";
    let mut gen_type = "";
    let mut folder = "";

    for line in lines {
        let values = line.split("=").collect::<Vec<&str>>();
        match values[0] {
            "api_url" => api_url = values[1],
            "lang" => lang = values[1],
            "gen_type" => gen_type = values[1],
            "folder" => folder = values[1],
            _ => return Err(String::from("Found unknown value in config file"))
        }
    }

    Ok(CodegenConfig{
        api_url: String::from(api_url),
        lang: String::from(lang),
        gen_type: String::from(gen_type),
        folder: String::from(folder)
    })
}

async fn download_api_info(config: &CodegenConfig) -> String {
    let api_info = reqwest::get(config.api_url.as_str())
        .await.unwrap()
        .text()
        .await.unwrap();
    api_info
}

async fn download_generated_archive<'a>(config: &CodegenConfig, api_info: &String) -> Result<Bytes, String> {
    let val = RawValue::from_string(api_info.clone()).unwrap();
    let body= json!({
        "lang": config.lang.clone(),
        "type": config.gen_type.clone(),
        "spec": val
    });
    let client = reqwest::Client::new();
    let response_result = client.post("https://generator3.swagger.io/api/generate")
        .json(&body)
        .send().await;
    let response = match response_result {
        Ok(response) => Ok(response),
        Err(error) =>  Err(format!("Can't download archive. ({})", error.to_string()))
    };

    match response?.bytes().await {
        Ok(bytes) => Ok(bytes),
        Err(error) => Err(format!("Can't receive archive. ({})", error.to_string()))
    }
}

fn delete_folder_if_exist(folder: &String) -> Result<(), String> {
    let path = Path::new(folder);
    if path.is_file() || path.is_symlink() {
        return Err(String::from("Output is not a folder"));
    }

    if path.is_dir() {
        return match fs::remove_dir_all(folder) {
            Ok(_) => Ok(()),
            Err(error) => Err(format!("Can't remove folder. ({})", error.to_string()))
        }
    }

    Ok(())
}

fn unzip_to_folder(zip_bytes: Bytes, folder: &String) -> Result<(), String> {
    delete_folder_if_exist(&folder)?;
    let path = Path::new(folder);
    let file = fs::create_dir(path);
    if file.is_err() {
        return Err(String::from("Can't create folder"));
    }
    let reader = io::Cursor::new(zip_bytes);
    let mut archive = match zip::ZipArchive::new(reader) {
        Ok(archive) => Ok(archive),
        Err(err) => Err(format!("Can't read downloaded file. ({})", err.to_string()))
    }?;
    match archive.extract(path) {
        Ok(()) => Ok(()),
        Err(error) => Err(format!("Can't extract downloaded file ({})", error.to_string()))
    }
}