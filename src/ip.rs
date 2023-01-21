use rocket::serde::{Deserialize, Serialize};
use std::{error::Error, fmt};

#[derive(Serialize, Debug)]
pub struct GeoLocation {
    pub ip: String,
    pub country_code: String,
    pub country_name: String,
    pub region_name: String,
    pub city: String,
    pub zip_code: String,
    pub time_zone: String,
    pub latitude: f32,
    pub longitude: f32,
}

#[derive(Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
struct FetchResponse {
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

pub async fn fetch_geo_ip(mut ip: String) -> Result<GeoLocation, Box<dyn Error>> {
    if ip == "127.0.0.1" {
        // When running locally, use ISP IP via the default behaviour of the API.
        ip = String::new();
    }

    let data = reqwest::get(format!("http://ip-api.com/json/{}", ip))
        .await?
        .json::<FetchResponse>()
        .await?;

    if data.status != "success" {
        return Err(Box::new(FetchError {
            status: data.status,
            message: data.message,
            query: data.query,
        }));
    }

    return Ok(GeoLocation {
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

#[derive(Debug)]
struct FetchError {
    status: String,
    message: String,
    query: String,
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "fetch geo ip: status='{0}', message='{1}' query='{2}'",
            self.status, self.message, self.query
        )
    }
}

impl Error for FetchError {}
