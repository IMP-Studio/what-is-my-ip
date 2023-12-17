use actix_cors::Cors;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn index(req: HttpRequest) -> impl Responder {
    let conn_info = req.connection_info();
    if let Some(realip_remote_addr) = &conn_info.realip_remote_addr() {
        HttpResponse::Ok().body(realip_remote_addr.to_string())
    } else {
        HttpResponse::Ok().body("")
    }
}

fn services() -> actix_web::Scope {
    web::scope("/").service(web::resource("").route(web::get().to(index)))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().wrap(Cors::permissive()).service(services()))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
