use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherResponse {
    location: String,
}

pub async fn get_weather_from_api(
    lat: f64,
    lng: f64,
) -> Result<WeatherResponse, Box<dyn std::error::Error>> {
    println!("{},{}", lat, lng);
    let mock_json: WeatherResponse = serde_json::from_str(
        r#"{
        "location": {
            "name": "Edina",
            "region": "Missouri",
            "country": "United States of America",
            "lat": 40.13,
            "lon": -92.14,
            "tz_id": "America/Chicago",
            "localtime_epoch": 1679269271,
            "localtime": "2023-03-19 18:41"
        },
        "current": {
            "temp_c": 3.9,
            "temp_f": 39.0,
            "condition": {
                "text": "Sunny",
                "icon": "//cdn.weatherapi.com/weather/64x64/day/113.png",
                "code": 1000
            },
            "wind_mph": 9.4,
            "wind_kph": 15.1,
            "wind_degree": 220,
            "precip_mm": 0.0,
            "precip_in": 0.0,
            "humidity": 31,
            "cloud": 0,
            "feelslike_c": -0.1,
            "feelslike_f": 31.9,
            "uv": 3.0,
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
        "forecast": {
            "forecastday": [
                {
                    "date": "2023-03-19",
                    "date_epoch": 1679184000,
                    "day": {
                        "maxtemp_c": 6.3,
                        "maxtemp_f": 43.3,
                        "mintemp_c": -7.7,
                        "mintemp_f": 18.1,
                        "avgtemp_c": -1.7,
                        "avgtemp_f": 28.9,
                        "maxwind_mph": 12.8,
                        "maxwind_kph": 20.5,
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
                    "date_epoch": 1679270400,
                    "day": {
                        "maxtemp_c": 13.5,
                        "maxtemp_f": 56.3,
                        "mintemp_c": -2.7,
                        "mintemp_f": 27.1,
                        "avgtemp_c": 4.5,
                        "avgtemp_f": 40.2,
                        "maxwind_mph": 22.4,
                        "maxwind_kph": 36.0,
                        "totalprecip_mm": 0.0,
                        "totalprecip_in": 0.0,
                        "totalsnow_cm": 0.0,
                        "avghumidity": 53.0,
                        "daily_chance_of_rain": 0,
                        "condition": {
                            "text": "Partly cloudy",
                            "icon": "//cdn.weatherapi.com/weather/64x64/day/116.png",
                            "code": 1003
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
