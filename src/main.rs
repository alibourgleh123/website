mod accounts_managment;
mod config;
mod join_form_handler;

use accounts_managment::misc_endpoints::get_user_info_endpoint;
use accounts_managment::register::register_endpoint;
use actix_files::Files;
use actix_web::{middleware, web, App, HttpServer, Responder};
use actix_web::middleware::Logger;
use chrono::Local;
use env_logger::Env;
use colorize::*;

use accounts_managment::{misc_endpoints::main_handler, init_database, login::login_endpoint};
use join_form_handler::form::{get_forms_endpoint, join_form_endpoint};

async fn invalid_path_handler() -> impl Responder {
    "invalid link
    الرابط غير صالح"
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    init_database();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new(&format!("{} %{}a %r \n{} %D {} %{}i", Local::now().format("%I:%M:%S %p"), r"{r}", "serve time taken:".bold().cyan(), "device:".bold().cyan(), r"{User-Agent}")))
            .wrap(middleware::Compress::default())
            .service(web::redirect("/", "/ar/index.html"))
            .service(main_handler)
            .service(login_endpoint)
            .service(register_endpoint)
            .service(get_user_info_endpoint)
            .service(join_form_endpoint)
            .service(get_forms_endpoint)
            .service(Files::new("/", "./site").index_file("/ar/index.html")).default_service(web::get().to(invalid_path_handler))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
