mod accounts_managment;
mod config;
mod join_form_handler;
mod consultations_handler;
mod database;

use actix_files::Files;
use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web::middleware::Logger;
use env_logger::Env;
use colorize::*;
use log::warn;

use database::init_database;
use consultations_handler::{consultations_sending_handler_endpoint, get_consultations_endpoint, 
                            attachments::{handle_consultations_upload, get_attachments_endpoint, download_attachment}};
use accounts_managment::{misc_endpoints::{main_handler, get_user_info_endpoint},
                         login::login_endpoint,
                         register::register_endpoint
                        };
use join_form_handler::{get_forms_endpoint, join_form_endpoint};
use urlencoding::decode;

async fn invalid_path_handler(req: HttpRequest) -> impl Responder {
    let decoded_path = decode(&req.path()).unwrap_or_else(|_| "Invalid encoding".into());
    warn!("Invalid link was requested: {}", decoded_path);
    HttpResponse::NotFound()
        .content_type("text/html")
        .body(r#"
                <style>
                    :root {
                        color-scheme: light dark;
                    }
            
                    body {
                        display: flex;
                        justify-content: center;
                        align-items: center;
                        text-align: center;
                        height: 100vh;
                        margin: 0;
                        flex-direction: column;
                    }
            
                    a {
                        font-size: 1.2rem;
                        margin-top: 1rem;
                    }
                </style>
                <meta charset="UTF-8">
                <h1>Invalid Link</h1>
                <h1>الرابط غير صالح</h1>
                <a href="/ar/index.html">العودة للصفحة الرئيسية</a>"#)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    init_database();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new(&format!("%{}a %r \n{} %D {} %{}i", r"{r}", "serve time taken:".bold().cyan(), "device:".bold().cyan(), r"{User-Agent}")))
            .wrap(middleware::Compress::default())
            .service(web::redirect("/", "/ar/index.html"))
            .service(main_handler)
            .service(login_endpoint)
            .service(register_endpoint)
            .service(get_user_info_endpoint)
            .service(join_form_endpoint)
            .service(get_forms_endpoint)
            .service(consultations_sending_handler_endpoint)
            .service(handle_consultations_upload)
            .service(get_consultations_endpoint)
            .service(get_attachments_endpoint)
            .route("/download_attachment", web::get().to(download_attachment))
            .service(Files::new("/", "./site").index_file("/ar/index.html")).default_service(web::get().to(invalid_path_handler))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
