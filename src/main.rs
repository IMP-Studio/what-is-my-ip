use actix_cors::Cors;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use serde_json::Serializer;

async fn index(req: HttpRequest) -> impl Responder {
    let conn_info = req.connection_info();
    let mut response: String = "".to_string();
    if let Some(realip_remote_addr) = &conn_info.realip_remote_addr() {
        response = realip_remote_addr.to_string()
    }
    HttpResponse::Ok().body(response)
}

#[derive(Serializer)]
struct IP {
    a: string,
    b: string,
    c: string,
}

async fn all(req: HttpRequest) -> impl Responder {
    let conn_info = req.connection_info();
    let mut response: String = "".to_string();
    let ip = IP {
        a: conn_info.host(),
        b: conn_info.realip_remote_addr(),
        c: conn_info.peer_addr()
    };
    HttpResponse::Ok().json(ip)
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
