use rocket::{http::Status, serde::json::Json};
use rocket_dyn_templates::{context, Template};
use std::net::IpAddr;

mod api;
mod ip;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}

#[get("/ip")]
async fn ip_handler(ip_addr: IpAddr) -> Result<Template, (Status, String)> {
    ip::fetch_geo_ip(&ip_addr.to_string())
        .await
        .map_err(|error| {
            warn!("{}", error);
            (Status::InternalServerError, error.to_string())
        })
        .map(|data| {
            Template::render(
                "ip",
                context! {
                    title: "IP",
                    geo_location: data,
                },
            )
        })
}

#[get("/ip.json")]
async fn ip_json_handler(
    ip_addr: IpAddr,
) -> Result<Json<ip::GeoLocation>, (Status, Json<api::ErrorResponse>)> {
    ip::fetch_geo_ip(&ip_addr.to_string())
        .await
        .map_err(|error| {
            warn!("{}", error);
            api::error_response(
                Status::InternalServerError,
                "IP Geolocation lookup failed",
                &error.to_string(),
            )
        })
        .map(|data| Json(data))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, ip_handler, ip_json_handler])
        .attach(Template::fairing())
}
