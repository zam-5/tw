use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct WeatherReport {
    location: Location,
    pub current: CurrentWeather,
    pub forecast: ForecastArr,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Location {
    name: String,
    region: String,
    country: String,
    lat: f64,
    lon: f64,
    tz_id: String,
    localtime_epoch: i64,
    localtime: String,
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CurrentWeather {
    pub last_updated_epoch: i64,
    pub last_updated: String,
    pub temp_c: f64,
    pub temp_f: f64,
    pub is_day: i32,
    pub condition: Condition,
    pub wind_mph: f64,
    pub wind_dir: String,
    pub cloud: f64,
    pub feelslike_f: f64,
    pub feelslike_c: f64,
    pub uv: f64,
    pub gust_mph: f64,
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Condition {
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ForecastArr {
    pub forecastday: [DayWeather; 3],
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DayWeather {
    pub date: String,
    pub day: DailyValues,
    pub astro: Astro,
    pub hour: [HourlyValues; 24],
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct DailyValues {
    pub maxtemp_c: f64,
    pub maxtemp_f: f64,
    pub mintemp_c: f64,
    pub mintemp_f: f64,
    pub totalprecip_mm: f64,
    pub totalprecip_in: f64,
    pub avgvis_miles: f64,
    pub daily_will_it_rain: f64,
    pub daily_chance_of_rain: f64,
    pub daily_will_it_snow: f64,
    pub daily_chance_of_snow: f64,
    pub condition: Condition,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Astro {
    pub sunrise: String,
    pub sunset: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct HourlyValues {
    pub time: String,
    pub temp_c: f64,
    pub temp_f: f64,
    pub is_day: f64,
    pub condition: Condition,
    pub wind_mph: f64,
    pub wind_dir: String,
    pub cloud: f64,
    pub feelslike_c: f64,
    pub feelslike_f: f64,
}
