use actix_cors::Cors;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde::Serialize;

async fn index(req: HttpRequest) -> impl Responder {
    let conn_info = req.connection_info();
    let mut response: String = "".to_string();
    if let Some(realip_remote_addr) = &conn_info.realip_remote_addr() {
        response = realip_remote_addr.to_string()
    }
    HttpResponse::Ok().body(response)
}

#[derive(Serialize)]
struct IP {
    a: String,
    b: String,
    c: String,
}

async fn all(req: HttpRequest) -> impl Responder {
    let conn_info = req.connection_info();
    let mut response: String = "".to_string();
    let ip = IP {
        a: conn_info.host().to_string(),
        b: conn_info.realip_remote_addr().unwrap().to_string(),
        c: conn_info.peer_addr().unwrap().to_strin()
    };
    HttpResponse::Ok().json(serde_json::to_string(&ip).unwrap())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .route("/all", web::get().to(all))
            .default_service(web::get().to(index))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
