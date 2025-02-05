mod accounts_managment;
mod config;

use actix_files::Files;
use actix_web::{web, App, HttpServer, Responder};

use accounts_managment::{endpoints::main_handler, init_database, login::login_endpoint};

async fn invalid_path_handler() -> impl Responder {
    "invalid link
    الرابط غير صالح"
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_database();
    
    HttpServer::new(|| {
        App::new()
            .service(web::redirect("/", "/ar/login.html"))
            .service(main_handler)
            .service(login_endpoint)
            .service(Files::new("/", "./site").index_file("/ar/login.html")).default_service(web::get().to(invalid_path_handler))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
