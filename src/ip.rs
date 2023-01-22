use rocket::serde::{Deserialize, Serialize};
use std::{error::Error, fmt};

#[cfg(test)]
use mockito;

#[derive(Serialize, Debug, PartialEq, PartialOrd)]
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

impl FetchResponse {
    fn is_success(&self) -> bool {
        self.status == "success"
    }

    fn is_failure(&self) -> bool {
        !self.is_success()
    }
}

#[derive(Debug)]
pub struct FetchError {
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

pub async fn fetch_geo_ip(mut ip: &str) -> Result<GeoLocation, Box<dyn Error>> {
    if ip == "127.0.0.1" {
        // When running locally, use ISP IP via the default behaviour of the API.
        ip = "";
    }

    let response = get_from_api(format!("/json/{}", ip))
        .await?
        .json::<FetchResponse>()
        .await?;

    if response.is_failure() {
        return Err(Box::new(FetchError {
            status: response.status,
            message: response.message,
            query: response.query,
        }));
    }

    return Ok(GeoLocation {
        ip: response.query,
        country_code: response.country_code,
        country_name: response.country,
        region_name: response.region_name,
        city: response.city,
        zip_code: response.zip,
        time_zone: response.timezone,
        latitude: response.lat,
        longitude: response.lon,
    });
}

async fn get_from_api(path: String) -> reqwest::Result<reqwest::Response> {
    reqwest::get(format!("{0}{1}", base_api_url(), path)).await
}

fn base_api_url() -> String {
    #[cfg(not(test))]
    let url = "http://ip-api.com";

    #[cfg(test)]
    let url = &mockito::server_url();

    url.into()
}

#[cfg(test)]
mod test {
    use super::{fetch_geo_ip, FetchError, GeoLocation};
    use mockito::mock;

    const SAMPLE_SUCCESS_RESPONSE: &str = r#"{"status":"success","country":"Norway","countryCode":"NO","region":"50","regionName":"Trøndelag","city":"Halsanaustan","zip":"6680","lat":63.0913,"lon":8.2362,"timezone":"Europe/Oslo","isp":"GLOBALCONNECT","org":"Svorka FTTH","as":"AS2116 GLOBALCONNECT AS","query":"143.110.98.165"}"#;
    const SAMPLE_FAILURE_RESPONSE: &str =
        r#"{"status":"fail","message":"reserved range","query":"127.0.0.1"}"#;

    #[tokio::test]
    async fn test_fetch_geo_ip() {
        let expected = test_geo_location();

        let _m = mock("GET", format!("/json/{}", expected.ip).as_str())
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(SAMPLE_SUCCESS_RESPONSE)
            .create();

        match fetch_geo_ip(expected.ip.as_str()).await {
            Err(error) => {
                panic!("fetch_geo_ip failed: {}", error)
            }
            Ok(result) => {
                assert_eq!(result, expected);
            }
        }
    }

    #[tokio::test]
    async fn test_fetch_geo_ip_error() {
        let input = "123.234.111.123";
        let _m = mock("GET", format!("/json/{}", input).as_str())
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(SAMPLE_FAILURE_RESPONSE)
            .create();

        match fetch_geo_ip(input).await {
            Err(error) => {
                let expected_error = FetchError {
                    status: String::from("fail"),
                    message: String::from("reserved range"),
                    query: String::from("127.0.0.1"),
                };
                assert_eq!(error.to_string(), expected_error.to_string())
            }
            Ok(result) => {
                panic!(
                    "fetch_geo_ip was successful when expecting to fail: {:?}",
                    result
                )
            }
        }
    }

    fn test_geo_location() -> GeoLocation {
        GeoLocation {
            ip: String::from("143.110.98.165"),
            country_code: String::from("NO"),
            country_name: String::from("Norway"),
            region_name: String::from("Trøndelag"),
            city: String::from("Halsanaustan"),
            zip_code: String::from("6680"),
            time_zone: String::from("Europe/Oslo"),
            latitude: 63.0913,
            longitude: 8.2362,
        }
    }
}
