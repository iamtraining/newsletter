use actix_web::dev::Server;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
	let name = req.match_info().get("name").unwrap_or("World");
	format!("Hello {}!", &name)
}

async fn health_check() -> HttpResponse {
	HttpResponse::Ok().finish()
}

pub fn run() -> Result<Server, std::io::Error> {
	let srv = HttpServer::new(|| {
		App::new()
			// .route("/", web::get().to(greet))
			// .route("/{name}", web::get().to(greet))
			.route("/health", web::get().to(health_check))
	})
	.bind("localhost:8000")?
	.run();
	Ok(srv)
}
