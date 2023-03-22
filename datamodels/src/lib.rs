use chrono::{NaiveDate, NaiveDateTime};
use serde::{de, Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Area {
    pub area_name: String,
    pub metadata: Metadata,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Metadata {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Data {
    pub areas: Vec<Area>,
}
mod weather_datetime_format {
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M";

    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

mod weather_date_format {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d";

    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherResponse {
    pub location: Location,
    pub current: Current,
    pub forecast: Forecast,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Location {
    pub name: String,
    region: String,
    country: String,
    pub lat: f64,
    pub lon: f64,
    tz_id: String,
    localtime_epoch: usize,
    #[serde(with = "weather_datetime_format")]
    localtime: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Current {
    temp_c: f64,
    temp_f: f64,
    condition: Condition,
    wind_mph: f64,
    wind_kph: f64,
    wind_degree: f64,
    precip_mm: f64,
    precip_in: f64,
    humidity: f64,
    cloud: f64,
    feelslike_c: f64,
    feelslike_f: f64,
    uv: f64,
    air_quality: AirQuality,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Condition {
    text: String,
    icon: String,
    code: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AirQuality {
    co: f64,
    no2: f64,
    o3: f64,
    so2: f64,
    pm2_5: f64,
    pm10: f64,
    #[serde(alias = "us-epa-index")]
    us_epa_index: usize,
    #[serde(alias = "gb-defra-index")]
    gb_defra_index: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Forecast {
    forecastday: Vec<ForecastDay>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ForecastDay {
    #[serde(with = "weather_date_format")]
    date: NaiveDate,
    day: Day,
    astro: Astro,
    hour: Vec<Hour>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Day {
    maxtemp_c: f64,
    maxtemp_f: f64,
    mintemp_c: f64,
    mintemp_f: f64,
    avgtemp_c: f64,
    avgtemp_f: f64,
    maxwind_mph: f64,
    maxwind_kph: f64,
    totalprecip_mm: f64,
    totalprecip_in: f64,
    totalsnow_cm: f64,
    daily_chance_of_rain: f64,
    condition: Condition,
    air_quality: AirQuality,
}
#[derive(Serialize, Deserialize, Debug)]
struct Astro {
    #[serde(deserialize_with = "deserialize_int_to_bool")]
    is_moon_up: bool,
    #[serde(deserialize_with = "deserialize_int_to_bool")]
    is_sun_up: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct Hour {
    condition: NoCondition,
    air_quality: AirQuality,
}

#[derive(Serialize, Deserialize, Debug)]
struct NoCondition {}

fn deserialize_int_to_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    let val: i8 = de::Deserialize::deserialize(deserializer)?;

    match val {
        0 => Ok(true),
        1 => Ok(false),
        _ => Err(de::Error::unknown_variant(
            &val.to_string()[..],
            &["0", "1"],
        )),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AreaWeather {
    pub area_name: String,
}

pub struct ResponseAndArea {
    pub response: WeatherResponse,
    pub area: Area,
}

// TODO trait to map between weather data models
impl From<ResponseAndArea> for AreaWeather {
    fn from(ra: ResponseAndArea) -> Self {
        Self {
            area_name: ra.area.area_name,
        }
    }
}
