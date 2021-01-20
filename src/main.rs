#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

use actix_web::middleware;
use actix_web::web::{self, Query};
use actix_web::{App, HttpResponse, HttpServer, Responder};
use env_logger::builder as log_builder;
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty as to_json_pretty;
use std::env::{set_var, var};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Result as IOResult;

const RUST_LOG: &str = "RUST_LOG";

#[derive(Debug, Deserialize, Serialize)]
struct TheRequest {
    #[serde(rename(deserialize = "NAME"))]
    name: String,
    #[serde(rename(deserialize = "AGE"))]
    age: u8,
}

impl Display for TheRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", to_json_pretty(self).unwrap())
    }
}

fn init_logger() {
    if var(RUST_LOG).is_err() {
        #[cfg(debug_assertions)]
        set_var(RUST_LOG, "debug,actix_server=debug,actix_web=debug");
        #[cfg(not(debug_assertions))]
        set_var(RUST_LOG, "info,actix_server=info,actix_web=info");
    }

    log_builder()
        .default_format()
        .format_timestamp_nanos()
        .format_indent(Some(2))
        .init();
}

async fn return_request(the_request: Query<TheRequest>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(format!("{}", the_request.into_inner()))
        .await
}

#[actix_web::main]
async fn main() -> IOResult<()> {
    init_logger();
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::get().to(return_request)))
    })
    .bind("0.0.0.0:8787")?
    .run()
    .await
}
