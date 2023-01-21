use rocket::{http::Status, serde::json::Json};
use std::net::IpAddr;

mod api;
mod ip;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "NerfThis Tools 🛠️"
}

#[get("/ip")]
async fn ip_handler(
    ip_addr: IpAddr,
) -> Result<Json<ip::GeoLocation>, (Status, Json<api::ErrorResponse>)> {
    ip::fetch_geo_ip(ip_addr.to_string())
        .await
        .map(|data| Json(data))
        .map_err(|error| {
            warn!("{}", error);
            api::error_response(
                Status::InternalServerError,
                String::from("IP Geolocation lookup failed"),
                error.to_string(),
            )
        })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, ip_handler])
}
