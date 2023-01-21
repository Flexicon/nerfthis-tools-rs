use rocket::{
    http::Status,
    serde::{json::Json, Deserialize, Serialize},
};
use std::{error::Error, fmt, net::IpAddr};

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "NerfThis Tools 🛠️"
}

#[derive(Serialize, Deserialize, Debug)]
struct ApiError {
    status: String,
    title: String,
    detail: String,
}

fn api_error(status: Status, title: String, detail: String) -> (Status, Json<ApiError>) {
    (
        status,
        Json(ApiError {
            status: status.to_string(),
            title,
            detail,
        }),
    )
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
async fn ip(ip_addr: IpAddr) -> Result<Json<GeoIp>, (Status, Json<ApiError>)> {
    fetch_geo_ip(ip_addr.to_string())
        .await
        .map(|data| Json(data))
        .map_err(|error| {
            println!("{}", error);
            api_error(
                Status::InternalServerError,
                String::from("IP Geolocation lookup failed"),
                error.to_string(),
            )
        })
}

#[derive(Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
struct GeoIpFetchResponse {
    status: String,
    message: String,
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

#[derive(Debug)]
struct FetchGeoIpError {
    status: String,
    message: String,
    query: String,
}

impl fmt::Display for FetchGeoIpError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "fetch geo ip: status='{0}', message='{1}' query='{2}'",
            self.status, self.message, self.query
        )
    }
}

impl Error for FetchGeoIpError {}

async fn fetch_geo_ip(mut ip: String) -> Result<GeoIp, Box<dyn Error>> {
    if ip == "127.0.0.1" {
        // When running locally, use ISP IP via the default behaviour of the API.
        ip = String::new();
    }

    let data = reqwest::get(format!("http://ip-api.com/json/{}", ip))
        .await?
        .json::<GeoIpFetchResponse>()
        .await?;

    if data.status != "success" {
        return Err(Box::new(FetchGeoIpError {
            status: data.status,
            message: data.message,
            query: data.query,
        }));
    }

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
