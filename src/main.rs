use actix_cors::Cors;
use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index(req: HttpRequest) -> impl Responder {
    let conn_info = req.connection_info();
    if let Some(realip_remote_addr) = &conn_info.realip_remote_addr() {
        HttpResponse::Ok().body(realip_remote_addr.to_string())
    } else {
        HttpResponse::Ok().body("")
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().wrap(Cors::permissive()).service(index))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
