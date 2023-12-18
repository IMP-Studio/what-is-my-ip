use actix_cors::Cors;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn index(req: HttpRequest) -> impl Responder {
    let conn_info = req.connection_info();
    let mut response: String = "".to_string();
    if let Some(realip_remote_addr) = &conn_info.realip_remote_addr() {
        response = realip_remote_addr.to_string()
    }
    HttpResponse::Ok().body(response)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .default_service(web::get().to(index))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
