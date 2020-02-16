use actix_web::{HttpRequest, HttpResponse, web};
use serde::{Serialize};

#[derive(Serialize)]
struct Example {
    name: String,
}

impl Example {
    pub fn new(name: String) -> Self {
        Self {
            name: name,
        }
    }
}

fn hello_handler(_request: HttpRequest) -> HttpResponse {
    let response = Example::new("Hello".to_string());
    return HttpResponse::Ok().json(response);
}

fn root_handler(_request: HttpRequest) -> HttpResponse {
    let response = Example::new("Alejandro".to_string());
    return HttpResponse::Ok().json(response);
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(root_handler));
    cfg.route("/hello", web::get().to(hello_handler));
}
