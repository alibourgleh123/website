pub mod upload;

use crate::database::get_database_connection;

use actix_web::{post, web, HttpResponse, Responder};
use chrono::Local;
use rusqlite::{params, Connection};
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
    error: Option<String>,
    uuid: Option<String>
}

#[post("/send_consultation")]
pub async fn consultations_sending_handler_endpoint(info: web::Json<ConsultationRequest>) -> actix_web::Result<impl Responder> {
    let connection: Connection = match get_database_connection() {
        Ok(connection) => connection,
        Err(e) => { 
            eprintln!("We are the consultations_sending_handler_endpoint function, We have failed to get connection to the database! here is the error:\n{}", e);
            return Ok(HttpResponse::InternalServerError().json(ConsultationResponse { error: Some("حدث خطأ بالخادم".to_string()), uuid: None }));
        }
    };

    if info.targeted_doctor.is_empty() || info.targeted_doctor.chars().count() > 64 {
        // targeted_doctor is sent by the website automatically
        return Ok(HttpResponse::BadRequest().json(ConsultationResponse { error: Some("Bad Request".to_string()), uuid: None }));
    }

    if info.name.is_empty() || info.name.chars().count() > 16 {
        return Ok(HttpResponse::BadRequest().json(ConsultationResponse { error: Some("يجب ألا يكون اسمك فارغاً ويجب ألا يتجاوز 16 حرفاً!".to_string()), uuid: None }));
    }

    if info.sur_name.is_empty() || info.sur_name.chars().count() > 16 {
        return Ok(HttpResponse::BadRequest().json(ConsultationResponse { error: Some("يجب ألا يكون اسم العائلة فارغاً ويجب ألا يتجاوز 16 حرفاً!".to_string()), uuid: None }));
    }

    if info.email.is_empty() || info.email.chars().count() > 64 {
        return Ok(HttpResponse::BadRequest().json(ConsultationResponse { error: Some("البريد الإلكتروني غير صالح، يجب أن يكون بريداً فعلياً وألا تكون خانة البريد فارغة ويجب ألا يتجاوز طول بريدك 64 حرفاً".to_string()), uuid: None }));
    }

    if info.phone_number.is_empty() || info.phone_number.chars().count() > 32 {
        return Ok(HttpResponse::BadRequest().json(ConsultationResponse { error: Some("يجب ألا يكون رقم الهاتف فارغاً ويجب ألا يتجاوز 32 حرفاً!".to_string()), uuid: None }));
    }

    if info.issue.is_empty() || info.issue.chars().count() > 1024 {
        return Ok(HttpResponse::BadRequest().json(ConsultationResponse { error: Some("يجب ألا يكون وصف حالتك فارغاً ويجب ألا يتجاوز 1024 حرفاً!".to_string()), uuid: None }));
    }

    // Get current time 
    let now = Local::now();
    let formatted_time = now.format("%Y/%m/%d %I:%M:%S %p").to_string();

    let uuid = uuid::Uuid::new_v4().to_string();

    if let Err(e) = connection.execute(
        "INSERT INTO consultations (targeted_doctor, name, sur_name, email, phone_number, issue, time, uuid) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        &[&info.targeted_doctor, &info.name, &info.sur_name, &info.email, &info.phone_number, &info.issue, &formatted_time, &uuid ]
    ) {
        eprintln!("We are the consultations_sending_handler_endpoint function, We have failed to insert the consultation into the database!, here is the error:\n{}", e);
        return Ok(HttpResponse::InternalServerError().json(ConsultationResponse { error: Some("حدث خطأ بالخادم".to_string()), uuid: None }));
    }

    Ok(HttpResponse::Ok().json(ConsultationResponse { error: None, uuid: Some(uuid) }))
}


#[derive(Serialize)]
pub struct Consultation {
    targeted_doctor: String,
    name: String,
    sur_name: String,
    email: String,
    phone_number: String,
    issue: String,
    time: String,
    uuid: String
}

#[derive(Serialize)]
pub struct GetConsultationsResponse {
    consultations: Option<Vec<Consultation>>
}

#[derive(Deserialize)]
pub struct GetConsultationsRequest {
    token: String
}

#[post("/get_consultations")]
pub async fn get_consultations_endpoint(req: web::Json<GetConsultationsRequest>) -> actix_web::Result<impl Responder> {
    let connection: Connection = match get_database_connection() {
        Ok(connection) => connection,
        Err(e) => { 
            eprintln!("We are the get_consultations_endpoint function, We have failed to get connection to the database! here is the error:\n{}", e);
            return Ok(HttpResponse::InternalServerError().json(GetConsultationsResponse {
                consultations: None
            })); 
        }
    };

    let mut validation_statement = match connection.prepare(
        "SELECT EXISTS (
            SELECT 1 
            FROM accounts 
            WHERE token = ?1 AND role = ?2
        )",
    ) {
        Ok(statement) => statement,
        Err(e) => {
            eprintln!("We are the get_consultations_endpoint function, We have failed to prepare the validation statement! here is the error:\n{}", e);
            return Ok(HttpResponse::InternalServerError().json(GetConsultationsResponse {
                consultations: None
            })); 
        }
    };
    
    // Check if the account exists and he is admin
    let grant_access: bool = match validation_statement.query_row(
        params![req.token, "Admin"],
        |row| row.get(0),
    ) {
        Ok(access) => access,
        Err(e) => {
            eprintln!("We are the get_consultations_endpoint function, We have failed to check if the account with the token {} exists! here is the error:\n{}", req.token, e);
            return Ok(HttpResponse::InternalServerError().json(GetConsultationsResponse {
                consultations: None
            }));
        }
    };

    if !grant_access {
        return Ok(HttpResponse::Unauthorized().json(GetConsultationsResponse {
            consultations: None
        }));
    }

    let mut consultations_statement = match connection.prepare("SELECT targeted_doctor, name, sur_name, email, phone_number, issue, time, uuid FROM consultations") {
        Ok(statement) => statement,
        Err(e) => {
            eprintln!("We are the get_consultations_endpoint function, We have failed to select stuff from the consultations table in the database! here is the error:\n{}", e);
            return Ok(HttpResponse::InternalServerError().json(GetConsultationsResponse {
                consultations: None
            }));
        }
    };
    let consultations_iter = match consultations_statement.query_map([], |row| {
        Ok(Consultation {
            targeted_doctor: row.get(0)?,
            name: row.get(1)?,
            sur_name: row.get(2)?,
            email: row.get(3)?,
            phone_number: row.get(4)?,
            issue: row.get(5)?,
            time: row.get(6)?,
            uuid: row.get(7)?
        })
    }) {
        Ok(iter) => iter,
        Err(e) => {
            eprintln!("We are the get_consultations_endpoint function, We have failed to create consultations_iter! here is the error:\n{}", e);
            return Ok(HttpResponse::InternalServerError().json(GetConsultationsResponse {
                consultations: None
            }));
        }
    };

    let mut consultations: Vec<Consultation> = Vec::new();

    for consultation in consultations_iter {
        consultations.push(match consultation {
            Ok(consultation) => consultation,
            Err(e) => {
                eprintln!("We are the get_consultations_endpoint function, We have failed to push a consultations into consultations vector! here is the error:\n{}", e);
                return Ok(HttpResponse::InternalServerError().json(GetConsultationsResponse {
                    consultations: None
                }));
            }
        });
    }

    Ok(HttpResponse::Ok().json(GetConsultationsResponse {
        consultations: Some(consultations)
    }))
}