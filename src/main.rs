mod api;
mod models;
mod repo;
use actix_web::{web::Data, App, HttpServer , middleware::Logger};
// use actix_cors::Cors;
use api::user_api::{create_user, delete_user, get_all_users, get_user, update_user ,index};
use repo::mongodb_repo::MongoRepo;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = MongoRepo::init().await;
    let db_data = Data::new(db);

    println!("start server on port 8080");
    HttpServer::new(move || {
        // let cors = Cors::default()
        //     .allowed_origin("http://localhost:8080");
        
        
        App::new()
            // .wrap(cors)
            .wrap(Logger::default())
            // .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(db_data.clone())
            .service(create_user)
            .service(get_user)
            .service(update_user)
            .service(delete_user)
            .service(get_all_users)
            .service(index)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}