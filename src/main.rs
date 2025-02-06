mod accounts_managment;
mod config;

use accounts_managment::register::register_endpoint;
use actix_files::Files;
use actix_web::{web, App, HttpServer, Responder};
use actix_web::middleware::Logger;
use env_logger::Env;

use accounts_managment::{misc_endpoints::main_handler, init_database, login::login_endpoint};

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
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .service(web::redirect("/", "/ar/login.html"))
            .service(main_handler)
            .service(login_endpoint)
            .service(register_endpoint)
            .service(Files::new("/", "./site").index_file("/ar/login.html")).default_service(web::get().to(invalid_path_handler))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
