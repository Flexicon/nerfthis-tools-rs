use rocket::{
    http::Status,
    serde::{json::Json, Deserialize, Serialize},
};
use std::{error::Error, fmt, net::IpAddr};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello Rocket! Blast off 🚀"
}

#[derive(Serialize, Deserialize, Debug)]
struct GeoIp {
    ip: String,
    country_code: String,
    country_name: String,
    region_name: String,
    city: String,
    zip_code: String,
    time_zone: String,
    latitude: f32,
    longitude: f32,
}

#[get("/ip")]
async fn ip(ip_addr: IpAddr) -> Result<Json<GeoIp>, (Status, String)> {
    match fetch_geo_ip(ip_addr.to_string()).await {
        Err(why) => {
            println!("failed to get_geo_ip: {}", why);
            Err((
                Status::InternalServerError,
                String::from("failed to lookup ip geolocation"),
            ))
        }
        Ok(geo_ip) => Ok(Json(geo_ip)),
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct GeoIpFetchResponse {
    query: String,
    country: String,
    country_code: String,
    region_name: String,
    city: String,
    zip: String,
    timezone: String,
    lat: f32,
    lon: f32,
}

#[derive(Debug, Clone)]
struct BadHttpResponseError;

impl fmt::Display for BadHttpResponseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "bad http response")
    }
}

impl Error for BadHttpResponseError {}

async fn fetch_geo_ip(ip: String) -> Result<GeoIp, Box<dyn Error>> {
    let response = reqwest::get(format!("http://ip-api.com/json/{}", ip)).await?;
    if !response.status().is_success() {
        return Err(Box::new(BadHttpResponseError));
    }

    let data = response.json::<GeoIpFetchResponse>().await?;

    return Ok(GeoIp {
        ip: data.query,
        country_code: data.country_code,
        country_name: data.country,
        region_name: data.region_name,
        city: data.city,
        zip_code: data.zip,
        time_zone: data.timezone,
        latitude: data.lat,
        longitude: data.lon,
    });
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, ip])
}
