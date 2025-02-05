use crate::accounts_managment::login::*;
use crate::accounts_managment::get_database_connection;

use actix_web::{get, web::get, App, HttpRequest, HttpServer, Responder, HttpResponse};
use std::{fs, path::Path};

fn get_authorization<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    req.headers().get("Authorization")?.to_str().ok()
}

#[get("/ar/main.html")]
pub async fn main_handler(req: HttpRequest) -> impl Responder {
    let unauthed_access_page = r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <meta http-equiv="refresh" content="1;url=/ar/login.html">
    <title>نعيد توجيهك...</title>
</head>
<body>
    <h1>وصولك غير مصرح به</h1>
    <p>إذا لم يعاد توجيهك تلقائيا لصفحة تسجيل الدخول اضغط <a href="/ar/login.html">هنا</a></p>
</body>
</html>
"#.to_string();

    let connection = get_database_connection().unwrap();
    if let Some(auth_header) = get_authorization(&req) {
        match login_with_token(&connection, auth_header) {
            Ok(result) => {
                if result {
                    let path = "site/ar/main.html";
                    if Path::new(path).exists() {
                        return HttpResponse::Ok().body(fs::read_to_string(path).expect("NANI! main.html doesnt exist even tho it does"));
                    } else {
                        println!("someone is trying to access main.html but it fails due to the file not existing!");
                        return HttpResponse::Ok().body("حدث خطأ بالخادم");
                    }
                } else {
                    return HttpResponse::Ok().body(unauthed_access_page);
                }
            },
            Err(_) => {
                return HttpResponse::Unauthorized().body(unauthed_access_page);
            }
        };
    } else {
        return HttpResponse::Unauthorized().body(unauthed_access_page);
    }
}