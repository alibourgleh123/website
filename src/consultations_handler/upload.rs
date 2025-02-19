use actix_multipart::Multipart;
use actix_web::{post, web, HttpResponse};
use futures::StreamExt;
use std::path::Path;
use tokio::{self, io::AsyncWriteExt, fs};
use serde::Deserialize;

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
