use crate::accounts_managment::login::*;
use crate::accounts_managment::get_database_connection;

use actix_web::{get, HttpRequest, Responder, HttpResponse};

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

    // Extract the token from the cookie
    let auth_cookie = req.cookie("auth_token");

    if let Some(auth_cookie) = auth_cookie {
        match login_with_token(&connection, auth_cookie.value()) {
            Ok(result) => {
                if result {
                    let path = "site/ar/main.html";
                    if std::path::Path::new(path).exists() {
                        return HttpResponse::Ok()
                            .body(std::fs::read_to_string(path).expect("main.html not found"));
                    } else {
                        println!("main.html not found!");
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