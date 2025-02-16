use crate::accounts_managment::get_database_connection;

use actix_web::{post, web, HttpResponse, Responder};
use chrono::Local;
use rusqlite::{params, Connection};
use serde::{Serialize, Deserialize};

fn validate_length(field: &str, max_length: usize, neccesary: bool, error_message: &str) -> Option<HttpResponse> {
    if field.chars().count() > max_length || (neccesary && field.chars().count() == 0) {
        return Some(HttpResponse::BadRequest().json(JoinFormResponse {
            success: false,
            error: error_message.to_string(),
        }));
    }
    None
}

#[derive(Deserialize)]
struct JoinFormRequest {
    name: String,
    speciality: String,
    residence: String,
    phone_number: String,
    email: String,
    more: String,
}

#[derive(Serialize)]
struct JoinFormResponse {
    success: bool,
    error: String
}

#[post("/join_form")]
pub async fn join_form_endpoint(form: web::Json<JoinFormRequest>) -> actix_web::Result<impl Responder>{
    if let Some(response) = validate_length(&form.name, 32, true, "يجب ألا يكون اسمك فارغاً ويجب ألا تتجاوز حروف اسمك 32 حرفاً!") {
        return Ok(response);
    }
    if let Some(response) = validate_length(&form.speciality, 64, true, "يجب ألا يكون تخصصك فارغاً ويجب ألا تتجاوز حروف تخصصك 64 حرفاً!") {
        return Ok(response);
    }
    if let Some(response) = validate_length(&form.residence, 64, true, "يجب ألا يكون البلد الذي تقيم فيه فارغاً ويجب ألا تتجاوز حروف البلد الذي تقيم فيه 64 حرفاً!") {
        return Ok(response);
    }
    if let Some(response) = validate_length(&form.phone_number, 64, true, "يجب ألا يكون رقم جوالك فارغاً ويجب ألا تتجاوز الأرقام المكونة لرقم جوالك 64 حرفاً!") {
        return Ok(response);
    }
    if let Some(response) = validate_length(&form.email, 64, true, "يجب ألا يكون بريدك الإلكتروني فارغاً ويجب ألا تتجاوز حروف بريدك الإلكتروني 64 حرفاً!") {
        return Ok(response);
    }
    if let Some(response) = validate_length(&form.more, 1000, false, "يجب ألا تتجاوز حروف المعلومات الإضافية 1000 حرف!") {
        return Ok(response);
    }
    
    let connection = get_database_connection().expect("failed to get database connection");

    let local = Local::now();
    let time = local.format("%Y/%m/%d %I:%M:%S %p").to_string();

    match connection.execute(
        "INSERT INTO form (name, speciality, residence, phone_number, email, more, time) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        (&form.name, &form.speciality, &form.residence, &form.phone_number, &form.email, &form.more, time), 
    ) {
        Ok(_) => { return Ok(HttpResponse::Ok().json(JoinFormResponse {
            success: true,
            error: "".to_string()
        })); },
        Err(_) => { return Ok(HttpResponse::InternalServerError().json(JoinFormResponse {
            success: false,
            error: "حدث خطأ بالخادم".to_string()
        })); },
    };
}

#[derive(Serialize)]
pub struct FormsResponse {
    forms: Option<Vec<Form>>
}

#[derive(Serialize)]
pub struct Form {
    name: String,
    speciality: String,
    residence: String,
    phone_number: String,
    email: String,
    more: String,
    time: String
}

#[derive(Deserialize)]
pub struct GetFormsRequest {
    token: String
}

#[post("/get_forms")]
pub async fn get_forms_endpoint(req: web::Json<GetFormsRequest>) -> actix_web::Result<impl Responder> {
    let connection: Connection = match get_database_connection() {
        Ok(connection) => connection,
        Err(e) => { 
            eprintln!("We are the get_forms_endpoint function, We have failed to get connection to the database! here is the error:\n{}", e);
            return Ok(HttpResponse::InternalServerError().json(FormsResponse {
                forms: None
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
            eprintln!("We are the get_forms_endpoint function, We have failed to prepare the validation statement! here is the error:\n{}", e);
            return Ok(HttpResponse::InternalServerError().json(FormsResponse {
                forms: None
            })); 
        }
    };
    
    // Check if the account exists
    let grant_access: bool = match validation_statement.query_row(
        params![req.token, "Admin"],
        |row| row.get(0),
    ) {
        Ok(access) => access,
        Err(e) => {
            eprintln!("We are the get_forms_endpoint function, We have failed to check if the account with the token {} exists! here is the error:\n{}", req.token, e);
            return Ok(HttpResponse::InternalServerError().json(FormsResponse {
                forms: None
            }));
        }
    };

    if !grant_access {
        return Ok(HttpResponse::Unauthorized().json(FormsResponse {
            forms: None
        }));
    }

    let mut forms_statement = match connection.prepare("SELECT name, speciality, residence, phone_number, email, more, time FROM form") {
        Ok(statement) => statement,
        Err(e) => {
            eprintln!("We are the get_forms_endpoint function, We have failed to select stuff from the form table in the database! here is the error:\n{}", e);
            return Ok(HttpResponse::InternalServerError().json(FormsResponse {
                forms: None
            }));
        }
    };
    let forms_iter = match forms_statement.query_map([], |row| {
        Ok(Form {
            name: row.get(0)?,
            speciality: row.get(1)?,
            residence: row.get(2)?,
            phone_number: row.get(3)?,
            email: row.get(4)?,
            more: row.get(5)?,
            time: row.get(6)?
        })
    }) {
        Ok(iter) => iter,
        Err(e) => {
            eprintln!("We are the get_forms_endpoint function, We have failed to create forms_iter! here is the error:\n{}", e);
            return Ok(HttpResponse::InternalServerError().json(FormsResponse {
                forms: None
            }));
        }
    };

    let mut forms: Vec<Form> = Vec::new();

    for form in forms_iter {
        forms.push(match form {
            Ok(form) => form,
            Err(e) => {
                eprintln!("We are the get_forms_endpoint function, We have failed to push a form into forms vector! here is the error:\n{}", e);
                return Ok(HttpResponse::InternalServerError().json(FormsResponse {
                    forms: None
                }));
            }
        });
    }

    Ok(HttpResponse::Ok().json(FormsResponse {
        forms: Some(forms)
    }))
}