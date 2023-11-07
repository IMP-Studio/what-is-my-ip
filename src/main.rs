use actix_cors::Cors;
use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index(req: HttpRequest) -> impl Responder {
    let conn_info = req.connection_info();
    HttpResponse::Ok().body(conn_info.realip_remote_addr().unwrap().to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().wrap(Cors::permissive()).service(index))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
