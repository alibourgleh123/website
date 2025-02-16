use crate::accounts_managment::get_database_connection;

use actix_web::{post, web, HttpResponse, Responder};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ConsultationRequest {
    targeted_doctor: String,
    name: String,
    sur_name: String,
    email: String,
    phone_number: String,
    issue: String,
}

#[derive(Serialize)]
pub struct ConsultationResponse {
    error: Option<String>
}

#[post("/send_consultation")]
pub async fn consultations_sending_handler_endpoint(info: web::Json<ConsultationRequest>) -> actix_web::Result<impl Responder> {
    let connection: Connection = match get_database_connection() {
        Ok(connection) => connection,
        Err(e) => { 
            eprintln!("We are the consultations_sending_handler_endpoint function, We have failed to get connection to the database! here is the error:\n{}", e);
            return Ok(HttpResponse::InternalServerError().json(ConsultationResponse { error: Some("حدث خطأ بالخادم".to_string()) }));
        }
    };

    if info.targeted_doctor.is_empty() || info.targeted_doctor.chars().count() > 64 {
        // targeted_doctor is sent by the website automatically
        return Ok(HttpResponse::BadRequest().json(ConsultationResponse { error: Some("Bad Request".to_string()) }));
    }

    if info.name.is_empty() || info.name.chars().count() > 16 {
        return Ok(HttpResponse::BadRequest().json(ConsultationResponse { error: Some("يجب ألا يكون اسمك فارغاً ويجب ألا يتجاوز 16 حرفاً!".to_string()) }));
    }

    if info.sur_name.is_empty() || info.sur_name.chars().count() > 16 {
        return Ok(HttpResponse::BadRequest().json(ConsultationResponse { error: Some("يجب ألا يكون اسم العائلة فارغاً ويجب ألا يتجاوز 16 حرفاً!".to_string()) }));
    }

    if info.email.is_empty() || info.email.chars().count() > 64 {
        return Ok(HttpResponse::BadRequest().json(ConsultationResponse { error: Some("البريد الإلكتروني غير صالح، يجب أن يكون بريداً فعلياً وألا تكون خانة البريد فارغة ويجب ألا يتجاوز طول بريدك 64 حرفاً".to_string()) }));
    }

    if info.phone_number.is_empty() || info.phone_number.chars().count() > 32 {
        return Ok(HttpResponse::BadRequest().json(ConsultationResponse { error: Some("يجب ألا يكون رقم الهاتف فارغاً ويجب ألا يتجاوز 32 حرفاً!".to_string()) }));
    }

    if info.issue.is_empty() || info.issue.chars().count() > 1024 {
        return Ok(HttpResponse::BadRequest().json(ConsultationResponse { error: Some("يجب ألا يكون وصف حالتك فارغاً ويجب ألا يتجاوز 1024 حرفاً!".to_string()) }));
    }

    if connection.execute(
        "INSERT INTO consultations (targeted_doctor, name, sur_name, email, phone_number, issue, uuid) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        &[&info.targeted_doctor, &info.name, &info.sur_name, &info.email, &info.phone_number, &info.issue, &uuid::Uuid::new_v4().to_string()]
    ).is_err() {
        eprintln!("We are the consultations_sending_handler_endpoint function, We have failed to insert the consultation into the database!");
        return Ok(HttpResponse::InternalServerError().json(ConsultationResponse { error: Some("حدث خطأ بالخادم".to_string()) }));
    }

    Ok(HttpResponse::Ok().into())
}