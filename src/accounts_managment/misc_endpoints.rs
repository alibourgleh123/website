use crate::accounts_managment::get_user_info;
use crate::accounts_managment::login::*;
use crate::database::get_database_connection;

use actix_web::{web, post, get, HttpRequest, Responder, HttpResponse};
use rusqlite::Connection;
use serde::Deserialize;

#[get("/ar/main")]
pub async fn main_handler(req: HttpRequest) -> impl Responder {
    let unauthed_access_page = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <meta charset="UTF-8">
        <meta http-equiv="refresh" content="0;url=/ar/login.html">
        <title>نعيد توجيهك...</title>
    </head>
    <body>
        <h1>وصولك غير مصرح به</h1>
        <p>إذا لم يعاد توجيهك تلقائيا لصفحة تسجيل الدخول اضغط <a href="/ar/login.html">هنا</a></p>
    </body>
    </html>
    "#.to_string();

    let connection: Connection = match get_database_connection() {
        Ok(connection) => connection,
        Err(e) => { 
            eprintln!("We are the main_handler function, We have failed to get connection to the database! here is the error:\n{}", e);
            return HttpResponse::InternalServerError().body("حدث خطأ بالخادم");
        }
    };

    // Extract the token from the cookie
    let auth_cookie = req.cookie("auth_token");

    if let Some(auth_cookie) = auth_cookie {
        match login_with_token(&connection, auth_cookie.value()) {
            Ok(result) => {
                if check_if_admin(&connection, auth_cookie.value()).unwrap() {
                    let path = "privileged_site/ar/admin.html";
                    if std::path::Path::new(path).exists() {
                        return HttpResponse::Ok()
                            .body(std::fs::read_to_string(path).expect("admin.html not found"));
                    } else {
                        eprintln!("admin.html not found!");
                        return HttpResponse::InternalServerError().body("حدث خطأ بالخادم");
                    }
                }

                if result {
                    let path = "privileged_site/ar/main.html";
                    if std::path::Path::new(path).exists() {
                        return HttpResponse::Ok()
                            .body(std::fs::read_to_string(path).expect("main.html not found"));
                    } else {
                        eprintln!("main.html not found!");
                        return HttpResponse::InternalServerError().body("حدث خطأ بالخادم");
                    }
                } else {
                    return HttpResponse::Unauthorized().body(unauthed_access_page);
                }
            },
            Err(_) => {
                return HttpResponse::Unauthorized().body(unauthed_access_page);
            }
        }
    } else {
        return HttpResponse::Unauthorized().body(unauthed_access_page);
    }
}


#[derive(Deserialize)]
struct RegisterRequest {
    token: String
}

#[post("/getuserinfo")]
pub async fn get_user_info_endpoint(token: web::Json<RegisterRequest>) -> actix_web::Result<impl Responder>{
    let connection = get_database_connection().expect("failed to get database connection");

    Ok(HttpResponse::Ok().json(get_user_info(&connection, token.token.clone())))
}