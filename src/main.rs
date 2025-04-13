
mod api;
mod model;
mod repository;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use api::task::{complete_task, fail_task, get_task, pause_task, start_task, submit_task};
use repository::ddb::DDBRepository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1"); // fixed typo here too
    env_logger::init();

    let config = aws_config::load_from_env().await;
    let ddb_repo: DDBRepository = DDBRepository::init(String::from("task"), config);
    let ddb_data = Data::new(ddb_repo);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(ddb_data.clone())
            .service(get_task)
            .service(submit_task)
            .service(start_task)
            .service(complete_task)
            .service(pause_task)
            .service(fail_task)
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await?;

    Ok(())
}

