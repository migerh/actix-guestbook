use std::sync::Mutex;
use actix_web::web::{Data, scope};
use actix_web::{HttpServer, App};

mod controllers;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = Data::new(Mutex::new(state::AppState { entries: vec![] }));

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(scope("/").configure(controllers::routes))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
