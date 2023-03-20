use chrono::{NaiveDate, NaiveDateTime};
use serde::{de, Deserialize, Serialize};

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
    location: Location,
    current: Current,
    forecast: Forecast,
}

#[derive(Serialize, Deserialize, Debug)]
struct Location {
    name: String,
    region: String,
    country: String,
    lat: f64,
    lon: f64,
    tz_id: String,
    localtime_epoch: usize,
    #[serde(with = "weather_datetime_format")]
    localtime: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
struct Current {
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
struct Condition {
    text: String,
    icon: String,
    code: usize,
}

#[derive(Serialize, Deserialize, Debug)]
struct AirQuality {
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
struct Forecast {
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

pub async fn get_weather_from_api(
    lat: f64,
    lng: f64,
) -> Result<WeatherResponse, Box<dyn std::error::Error>> {
    let mock_json: WeatherResponse = serde_json::from_str(
        r#"{
            "location": {
                "name": "Edina",
                "region": "Missouri",
                "country": "United States of America",
                "lat": 40.13,
                "lon": -92.14,
                "tz_id": "America/Chicago",
                "localtime_epoch": 1679286189,
                "localtime": "2023-03-19 23:23"
            },
            "current": {
                "temp_c": -2.2,
                "temp_f": 28.0,
                "condition": {
                    "text": "Clear",
                    "icon": "//cdn.weatherapi.com/weather/64x64/night/113.png",
                    "code": 1000
                },
                "wind_mph": 9.4,
                "wind_kph": 15.1,
                "wind_degree": 190,
                "precip_mm": 0.0,
                "precip_in": 0.0,
                "humidity": 55,
                "cloud": 0,
                "feelslike_c": -8.0,
                "feelslike_f": 17.5,
                "uv": 1.0,
                "air_quality": {
                    "co": 247.0,
                    "no2": 2.5,
                    "o3": 101.5999984741211,
                    "so2": 1.7999999523162842,
                    "pm2_5": 5.599999904632568,
                    "pm10": 5.800000190734863,
                    "us-epa-index": 1,
                    "gb-defra-index": 1
                }
            },
            "forecast": {
                "forecastday": [
                    {
                        "date": "2023-03-19",
                        "day": {
                            "maxtemp_c": 6.0,
                            "maxtemp_f": 42.8,
                            "mintemp_c": -7.7,
                            "mintemp_f": 18.1,
                            "avgtemp_c": -1.8,
                            "avgtemp_f": 28.7,
                            "maxwind_mph": 13.0,
                            "maxwind_kph": 20.9,
                            "totalprecip_mm": 0.0,
                            "totalprecip_in": 0.0,
                            "totalsnow_cm": 0.0,
                            "avghumidity": 47.0,
                            "daily_chance_of_rain": 0,
                            "condition": {
                                "text": "Sunny",
                                "icon": "//cdn.weatherapi.com/weather/64x64/day/113.png",
                                "code": 1000
                            },
                            "air_quality": {
                                "co": 243.8079992675781,
                                "no2": 1.2560000026226044,
                                "o3": 94.38800140380859,
                                "so2": 0.31600000739097595,
                                "pm2_5": 1.7439999961853028,
                                "pm10": 1.923999993801117,
                                "us-epa-index": 1,
                                "gb-defra-index": 1
                            }
                        },
                        "astro": {
                            "is_moon_up": 0,
                            "is_sun_up": 1
                        },
                        "hour": [
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 247.0,
                                    "no2": 1.399999976158142,
                                    "o3": 97.30000305175781,
                                    "so2": 0.30000001192092896,
                                    "pm2_5": 1.399999976158142,
                                    "pm10": 1.5,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 247.0,
                                    "no2": 1.5,
                                    "o3": 97.30000305175781,
                                    "so2": 0.30000001192092896,
                                    "pm2_5": 1.600000023841858,
                                    "pm10": 1.7999999523162842,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 247.0,
                                    "no2": 1.5,
                                    "o3": 95.80000305175781,
                                    "so2": 0.4000000059604645,
                                    "pm2_5": 1.899999976158142,
                                    "pm10": 2.0999999046325684,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 247.0,
                                    "no2": 1.5,
                                    "o3": 95.80000305175781,
                                    "so2": 0.4000000059604645,
                                    "pm2_5": 2.200000047683716,
                                    "pm10": 2.5,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 247.0,
                                    "no2": 1.600000023841858,
                                    "o3": 94.4000015258789,
                                    "so2": 0.4000000059604645,
                                    "pm2_5": 2.5,
                                    "pm10": 2.799999952316284,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 250.3000030517578,
                                    "no2": 1.600000023841858,
                                    "o3": 93.0,
                                    "so2": 0.4000000059604645,
                                    "pm2_5": 2.700000047683716,
                                    "pm10": 3.0,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 250.3000030517578,
                                    "no2": 1.7000000476837158,
                                    "o3": 93.0,
                                    "so2": 0.4000000059604645,
                                    "pm2_5": 2.700000047683716,
                                    "pm10": 3.0,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 250.3000030517578,
                                    "no2": 1.7000000476837158,
                                    "o3": 91.5999984741211,
                                    "so2": 0.4000000059604645,
                                    "pm2_5": 2.799999952316284,
                                    "pm10": 3.0999999046325684,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 253.6999969482422,
                                    "no2": 1.7999999523162842,
                                    "o3": 90.80000305175781,
                                    "so2": 0.4000000059604645,
                                    "pm2_5": 2.9000000953674316,
                                    "pm10": 3.200000047683716,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 253.6999969482422,
                                    "no2": 2.200000047683716,
                                    "o3": 91.5999984741211,
                                    "so2": 0.4000000059604645,
                                    "pm2_5": 2.799999952316284,
                                    "pm10": 3.0999999046325684,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 253.6999969482422,
                                    "no2": 2.0,
                                    "o3": 93.0,
                                    "so2": 0.30000001192092896,
                                    "pm2_5": 2.5999999046325684,
                                    "pm10": 2.9000000953674316,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 247.0,
                                    "no2": 1.5,
                                    "o3": 95.80000305175781,
                                    "so2": 0.30000001192092896,
                                    "pm2_5": 2.299999952316284,
                                    "pm10": 2.5,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 243.6999969482422,
                                    "no2": 1.100000023841858,
                                    "o3": 97.30000305175781,
                                    "so2": 0.30000001192092896,
                                    "pm2_5": 2.0999999046325684,
                                    "pm10": 2.299999952316284,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 243.6999969482422,
                                    "no2": 0.8999999761581421,
                                    "o3": 97.30000305175781,
                                    "so2": 0.30000001192092896,
                                    "pm2_5": 2.0,
                                    "pm10": 2.200000047683716,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 233.6999969482422,
                                    "no2": 0.6000000238418579,
                                    "o3": 104.4000015258789,
                                    "so2": 0.30000001192092896,
                                    "pm2_5": 1.7000000476837158,
                                    "pm10": 1.899999976158142,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 227.0,
                                    "no2": 0.5,
                                    "o3": 107.30000305175781,
                                    "so2": 0.30000001192092896,
                                    "pm2_5": 1.399999976158142,
                                    "pm10": 1.5,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 227.0,
                                    "no2": 0.5,
                                    "o3": 107.30000305175781,
                                    "so2": 0.4000000059604645,
                                    "pm2_5": 1.2999999523162842,
                                    "pm10": 1.399999976158142,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 223.60000610351562,
                                    "no2": 0.6000000238418579,
                                    "o3": 107.30000305175781,
                                    "so2": 0.5,
                                    "pm2_5": 1.399999976158142,
                                    "pm10": 1.5,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 227.0,
                                    "no2": 1.0,
                                    "o3": 105.9000015258789,
                                    "so2": 0.699999988079071,
                                    "pm2_5": 1.7000000476837158,
                                    "pm10": 1.7999999523162842,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 230.3000030517578,
                                    "no2": 1.5,
                                    "o3": 103.0,
                                    "so2": 0.8999999761581421,
                                    "pm2_5": 2.0,
                                    "pm10": 2.0999999046325684,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 230.3000030517578,
                                    "no2": 1.7999999523162842,
                                    "o3": 101.5999984741211,
                                    "so2": 1.0,
                                    "pm2_5": 2.4000000953674316,
                                    "pm10": 2.5999999046325684,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 233.6999969482422,
                                    "no2": 2.0,
                                    "o3": 100.0999984741211,
                                    "so2": 1.100000023841858,
                                    "pm2_5": 3.0999999046325684,
                                    "pm10": 3.4000000953674316,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 240.3000030517578,
                                    "no2": 2.5999999046325684,
                                    "o3": 100.0999984741211,
                                    "so2": 2.0,
                                    "pm2_5": 4.699999809265137,
                                    "pm10": 5.0,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 247.0,
                                    "no2": 2.5,
                                    "o3": 101.5999984741211,
                                    "so2": 1.7999999523162842,
                                    "pm2_5": 5.599999904632568,
                                    "pm10": 5.800000190734863,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            }
                        ]
                    },
                    {
                        "date": "2023-03-20",
                        "day": {
                            "maxtemp_c": 14.4,
                            "maxtemp_f": 57.9,
                            "mintemp_c": -2.8,
                            "mintemp_f": 27.0,
                            "avgtemp_c": 4.6,
                            "avgtemp_f": 40.2,
                            "maxwind_mph": 22.6,
                            "maxwind_kph": 36.4,
                            "totalprecip_mm": 0.0,
                            "totalprecip_in": 0.0,
                            "totalsnow_cm": 0.0,
                            "avghumidity": 54.0,
                            "daily_chance_of_rain": 0,
                            "condition": {
                                "text": "Sunny",
                                "icon": "//cdn.weatherapi.com/weather/64x64/day/113.png",
                                "code": 1000
                            },
                            "air_quality": {
                                "co": 234.19600036621094,
                                "no2": 1.5319999837875367,
                                "o3": 101.74,
                                "so2": 1.2679999959468842,
                                "pm2_5": 3.3560000133514403,
                                "pm10": 3.671999998092651,
                                "us-epa-index": 1,
                                "gb-defra-index": 1
                            }
                        },
                        "astro": {
                            "is_moon_up": 0,
                            "is_sun_up": 1
                        },
                        "hour": [
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 243.6999969482422,
                                    "no2": 2.0999999046325684,
                                    "o3": 101.5999984741211,
                                    "so2": 1.399999976158142,
                                    "pm2_5": 4.800000190734863,
                                    "pm10": 5.0,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 240.3000030517578,
                                    "no2": 1.7999999523162842,
                                    "o3": 100.0999984741211,
                                    "so2": 1.899999976158142,
                                    "pm2_5": 3.9000000953674316,
                                    "pm10": 4.099999904632568,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 237.0,
                                    "no2": 1.600000023841858,
                                    "o3": 100.0999984741211,
                                    "so2": 2.0,
                                    "pm2_5": 3.4000000953674316,
                                    "pm10": 3.700000047683716,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 237.0,
                                    "no2": 1.5,
                                    "o3": 100.0999984741211,
                                    "so2": 1.7999999523162842,
                                    "pm2_5": 3.4000000953674316,
                                    "pm10": 3.5999999046325684,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 237.0,
                                    "no2": 1.399999976158142,
                                    "o3": 98.69999694824219,
                                    "so2": 1.5,
                                    "pm2_5": 3.0999999046325684,
                                    "pm10": 3.4000000953674316,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 237.0,
                                    "no2": 1.2999999523162842,
                                    "o3": 98.69999694824219,
                                    "so2": 1.2999999523162842,
                                    "pm2_5": 2.9000000953674316,
                                    "pm10": 3.0999999046325684,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 237.0,
                                    "no2": 1.2000000476837158,
                                    "o3": 97.30000305175781,
                                    "so2": 1.100000023841858,
                                    "pm2_5": 2.700000047683716,
                                    "pm10": 2.9000000953674316,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 237.0,
                                    "no2": 1.2000000476837158,
                                    "o3": 97.30000305175781,
                                    "so2": 1.2000000476837158,
                                    "pm2_5": 2.9000000953674316,
                                    "pm10": 3.0999999046325684,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 237.0,
                                    "no2": 1.5,
                                    "o3": 97.30000305175781,
                                    "so2": 1.2999999523162842,
                                    "pm2_5": 3.299999952316284,
                                    "pm10": 3.5999999046325684,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 237.0,
                                    "no2": 2.299999952316284,
                                    "o3": 97.30000305175781,
                                    "so2": 1.2999999523162842,
                                    "pm2_5": 3.700000047683716,
                                    "pm10": 4.0,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 237.0,
                                    "no2": 2.5,
                                    "o3": 100.0999984741211,
                                    "so2": 1.399999976158142,
                                    "pm2_5": 3.9000000953674316,
                                    "pm10": 4.400000095367432,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 233.6999969482422,
                                    "no2": 2.0999999046325684,
                                    "o3": 100.0999984741211,
                                    "so2": 1.600000023841858,
                                    "pm2_5": 4.099999904632568,
                                    "pm10": 4.900000095367432,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 230.3000030517578,
                                    "no2": 1.7999999523162842,
                                    "o3": 101.5999984741211,
                                    "so2": 1.7000000476837158,
                                    "pm2_5": 4.599999904632568,
                                    "pm10": 5.5,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 227.0,
                                    "no2": 1.399999976158142,
                                    "o3": 104.4000015258789,
                                    "so2": 1.7000000476837158,
                                    "pm2_5": 5.5,
                                    "pm10": 6.300000190734863,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 227.0,
                                    "no2": 1.0,
                                    "o3": 110.19999694824219,
                                    "so2": 1.5,
                                    "pm2_5": 6.400000095367432,
                                    "pm10": 7.199999809265137,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 223.60000610351562,
                                    "no2": 0.699999988079071,
                                    "o3": 113.0,
                                    "so2": 1.2999999523162842,
                                    "pm2_5": 6.599999904632568,
                                    "pm10": 7.5,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 220.3000030517578,
                                    "no2": 0.6000000238418579,
                                    "o3": 114.4000015258789,
                                    "so2": 1.100000023841858,
                                    "pm2_5": 6.300000190734863,
                                    "pm10": 7.199999809265137,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 220.3000030517578,
                                    "no2": 0.699999988079071,
                                    "o3": 113.0,
                                    "so2": 1.100000023841858,
                                    "pm2_5": 5.800000190734863,
                                    "pm10": 6.699999809265137,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 220.3000030517578,
                                    "no2": 0.8999999761581421,
                                    "o3": 113.0,
                                    "so2": 1.0,
                                    "pm2_5": 5.5,
                                    "pm10": 6.300000190734863,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 220.3000030517578,
                                    "no2": 1.2000000476837158,
                                    "o3": 111.5999984741211,
                                    "so2": 1.100000023841858,
                                    "pm2_5": 5.099999904632568,
                                    "pm10": 6.0,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 223.60000610351562,
                                    "no2": 1.5,
                                    "o3": 110.19999694824219,
                                    "so2": 1.2000000476837158,
                                    "pm2_5": 5.0,
                                    "pm10": 5.800000190734863,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 227.0,
                                    "no2": 1.600000023841858,
                                    "o3": 107.30000305175781,
                                    "so2": 1.2000000476837158,
                                    "pm2_5": 5.0,
                                    "pm10": 5.900000095367432,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 227.0,
                                    "no2": 1.600000023841858,
                                    "o3": 105.9000015258789,
                                    "so2": 1.100000023841858,
                                    "pm2_5": 5.099999904632568,
                                    "pm10": 6.0,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            },
                            {
                                "condition": {},
                                "air_quality": {
                                    "co": 227.0,
                                    "no2": 1.5,
                                    "o3": 103.0,
                                    "so2": 1.0,
                                    "pm2_5": 5.0,
                                    "pm10": 6.0,
                                    "us-epa-index": 1,
                                    "gb-defra-index": 1
                                }
                            }
                        ]
                    }
                ]
            }
        }"#,
    )?;

    return Ok(mock_json);
}

pub async fn add_weather_to_db(
    response_json: WeatherResponse,
) -> Result<(), Box<dyn std::error::Error>> {
    // Add the weather data from a response string to the database

    println!("{:?}", response_json);
    Ok(())
}
