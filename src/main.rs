mod api;
mod repository;
use:api::task::{
    get_task
}

use std::env;

use actix_web::{ HttpServer, App, web::Data, middleware::Logger };
use repository::ddb::DDBRepository;

use log::{debug, error, log_enabled, info, Level};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //logging
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST BACKTRACE", "1");
    env_logger::init();

    let config = aws_config::load_from_env().await;
    let ddb_repo: DDBRepository = DDBRepository::init(String::from("task"), config.clone());

    let ddb_data = Data::new(ddb_repo);
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(ddb_data)
            .service(get_task)
    }).bind(("127.0.0.1", "80"))?
            .run()
            .await?
}
