//=== web ===
use actix_files::{Files, NamedFile};
use actix_web::{get, App, HttpServer, Responder, Result};

#[get("/")]
async fn root() -> Result<impl Responder>{
	Ok(NamedFile::open("static/html/login.html")?)
}

#[get("/account")]
async fn account() -> Result<impl Responder>{
	Ok(NamedFile::open("static/html/account.html")?)
}

#[get("/feed")]
async fn feed() -> Result<impl Responder>{
	Ok(NamedFile::open("static/html/feed.html")?)
}

#[get("/messages")]
async fn messages() -> Result<impl Responder>{
	Ok(NamedFile::open("static/html/messages.html")?)
}

#[get("/notifications")]
async fn notifications() -> Result<impl Responder>{
	Ok(NamedFile::open("static/html/notifications.html")?)
}

#[get("/premium")]
async fn premium() -> Result<impl Responder>{
	Ok(NamedFile::open("static/html/premium.html")?)
}

#[get("/settings")]
async fn settings() -> Result<impl Responder>{
	Ok(NamedFile::open("static/html/settings.html")?)
}

#[get("/stories")]
async fn stories() -> Result<impl Responder>{
	Ok(NamedFile::open("static/html/stories.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()>{
	HttpServer::new(|| {
		App::new().service(Files::new("/static", "static"))
		.service(root)
		.service(account)
		.service(feed)
		.service(messages)
		.service(notifications)
		.service(premium)
		.service(settings)
		.service(stories)
	})
	.bind(("127.0.0.1", 8080))?.run().await
}
