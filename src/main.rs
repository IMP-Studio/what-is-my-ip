use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index(req: HttpRequest) -> impl Responder {
    let Some(client_ip) = req.peer_addr().map(|addr| addr.ip()) else { todo!() };
    HttpResponse::Ok().body(client_ip.to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}
