use actix_files::NamedFile;
use actix_multipart::Multipart;
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use futures::StreamExt;
use rusqlite::Connection;
use std::path::{Path, PathBuf};
use tokio::{self, io::AsyncWriteExt, fs};
use serde::{Deserialize, Serialize};

use crate::{accounts_managment::login::check_if_admin, database::get_database_connection};

#[derive(Deserialize)]
struct ConsultationUploadRequest {
    filename: String,
    uuid: String
}

#[post("/uploadconsultationfile")]
pub async fn handle_consultations_upload(payload: Multipart, request: web::Query<ConsultationUploadRequest>) -> Result<HttpResponse, actix_web::error::Error> {
    if request.uuid.contains("..") || request.filename.contains("..") || request.uuid.chars().count() > 256 || request.filename.chars().count() > 256 {
        return Ok(HttpResponse::BadRequest().body("العب بعيد يلا"));
    }

    if !Path::new("consultations_files").exists() {
        if let Err(_) = fs::create_dir("consultations_files").await {
            return Ok(HttpResponse::InternalServerError().body("failed to create consultations_files directory"));
        }
    }

    let path = &format!("consultations_files/{}", request.uuid);

    if !Path::new(path).exists() {
        if let Err(_) = fs::create_dir(path).await {
            return Ok(HttpResponse::InternalServerError().body("failed to create uuid specific path"));
        }
    }

    save_file(payload, format!("{}/{}", path, request.filename).into()).await
}

pub async fn save_file(mut payload: Multipart, file_path: std::path::PathBuf) -> Result<HttpResponse, actix_web::error::Error> {
    let mut file = tokio::fs::File::create(file_path).await?;

    while let Some(field) = payload.next().await {
        let mut field = match field {
            Ok(field) => field,
            Err(e) => return Err(actix_web::error::ErrorBadRequest(e.to_string())),
        };

        if field.name() == "file" {
            // Write the file content to the file
            while let Some(chunk) = field.next().await {
                let chunk = match chunk {
                    Ok(chunk) => chunk,
                    Err(e) => return Err(actix_web::error::ErrorBadRequest(e.to_string()))
                };

                let _ = file.write_all(&chunk).await?;
            }
        }
    }

    Ok(HttpResponse::Ok().body("تم حفظ الملف بنجاح"))
}

#[derive(Deserialize)]
pub struct GetAttachmentsRequest {
    token: String,
    uuid: String
}

#[derive(Serialize)]
pub struct GetAttachmentsResponse {
    attachments: Option<Vec<String>>
}

#[post("/get_attachments")]
pub async fn get_attachments_endpoint(req: web::Json<GetAttachmentsRequest>) -> actix_web::Result<impl Responder> {
    let connection: Connection = match get_database_connection() {
        Ok(connection) => connection,
        Err(e) => { 
            eprintln!("We are the get_attachments_endpoint function, We have failed to get connection to the database! here is the error:\n{}", e);
            return Ok(HttpResponse::InternalServerError().json(GetAttachmentsResponse {
                attachments: None
            })); 
        }
    };
    
    let grant_access: bool = match check_if_admin(&connection, &req.token) {
        Ok(access) => access,
        Err(e) => {
            eprintln!("We are the get_attachments_endpoint function, We have failed to check if the account with the token {} is an admin! here is the error:\n{}", req.token, e);
            return Ok(HttpResponse::InternalServerError().json(GetAttachmentsResponse {
                attachments: None
            }));
        }
    };

    if !grant_access {
        return Ok(HttpResponse::Unauthorized().json(GetAttachmentsResponse {
            attachments: None
        }));
    }

    if req.uuid.contains("..") {
        return Ok(HttpResponse::BadRequest().json(GetAttachmentsResponse {
            attachments: None
        }));
    }

    let dir = &format!("consultations_files/{}", req.uuid);

    let path = Path::new(dir);
    let mut files = Vec::new();

    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Some(file_name) = entry.file_name().to_str() {
                files.push(file_name.to_string());
            }
        }
    }

    if files.is_empty() {
        return Ok(HttpResponse::Ok().json(GetAttachmentsResponse {
            attachments: None
        }));
    } else {
        return Ok(HttpResponse::Ok().json(GetAttachmentsResponse {
            attachments: Some(files)
        }));
    }
}

#[derive(Deserialize)]
pub struct GetFileRequest {
    uuid: String,
    file: String
}

pub async fn download_attachment(req: HttpRequest, query: web::Query<GetFileRequest>) -> actix_web::Result<impl Responder> {
    let token = match req.headers().get("Authorization") {
        Some(t) => t.to_str().unwrap_or(""),
        None => return Err(actix_web::error::ErrorBadRequest("No authorization header was provided")),
    };

    let connection: Connection = match get_database_connection() {
        Ok(connection) => connection,
        Err(e) => { 
            eprintln!("We are the download_attachment function, We have failed to get connection to the database! here is the error:\n{}", e);
            return Err(actix_web::error::ErrorInternalServerError("Failed to get connection to the database")); 
        }
    };
    
    // Check if the account exists and he is admin
    let grant_access: bool = match check_if_admin(&connection, token) {
        Ok(access) => access,
        Err(e) => {
            eprintln!("We are the download_attachment function, We have failed to check if the account with the token {} is an admin! here is the error:\n{}", token, e);
            return Err(actix_web::error::ErrorInternalServerError("Failed to check if the token represents an admin account!")); 
        }
    };

    if !grant_access {
        return Err(actix_web::error::ErrorUnauthorized("Access denied!")); 
    }

    if query.uuid.contains("..") || query.file.contains("..") {
        return Err(actix_web::error::ErrorBadRequest("You are not allowed to put .. in your request"))
    }

    let file_path: PathBuf = format!("consultations_files/{}/{}", query.uuid, query.file).into(); 

    NamedFile::open(file_path).map_err(|_| actix_web::error::ErrorNotFound("File not found"))
}