use crate::weather_report::WeatherReport;
use std::collections::HashMap;
use std::error::Error;
use std::process;

pub struct WeatherFetcher {
    _city: String,
    _state: String,
    _country: String,
    locstr: String,
    key: String,
    url: String,
    report: Option<WeatherReport>,
    //report_valid: bool,
}

impl WeatherFetcher {
    pub fn new() -> Self {
        Self {
            _city: String::new(),
            _state: String::new(),
            _country: String::new(),
            locstr: String::new(),
            key: String::new(),
            url: String::new(),
            report: None,
            //report_valid: false,
        }
    }

    pub fn _set_location(&mut self, city: &str, state: &str, country: &str) {
        self._city.push_str(city);
        self._state.push_str(state);
        self._country.push_str(country);
    }

    pub fn set_location_vec(&mut self, loc: Vec<String>) {
        for i in 1..loc.len() {
            self.locstr.push_str(&loc[i]);
            if i != loc.len() - 1 {
                self.locstr.push(',');
            }
        }
    }

    pub fn set_key(&mut self, key: &str) {
        self.key.push_str(key);
    }

    fn _generate_link(&mut self) -> Result<String, &str> {
        if !self._city.is_empty() && !self._state.is_empty() && !self.key.is_empty() {
            let url = format!(
                "http://api.weatherapi.com/v1/current.json?key={}&q={},{},{}&aqi=no",
                self.key, self._city, self._state, self._country
            );
            self.url.push_str(&url);
            Ok(url)
        } else {
            Err("Unanble to create url.")
        }
    }

    fn _generate_link_forecast(&mut self) -> Result<String, &str> {
        if !self._city.is_empty() && !self._state.is_empty() && !self.key.is_empty() {
            let url = format!("http://api.weatherapi.com/v1/forecast.json?key={}&q={},{},{}&days=3&aqi=no&alerts=no
            ", self.key, self._city, self._state, self._country );
            self.url.push_str(&url);
            Ok(url)
        } else {
            Err("Unanble to create url.")
        }
    }

    fn generate_link_from_arr(&mut self) -> Result<String, &str> {
        if !self.locstr.is_empty() {
            let url = format!(
                "http://api.weatherapi.com/v1/forecast.json?key={}&q={}&days=3&aqi=no&alerts=no",
                self.key, self.locstr
            );
            self.url.push_str(&url);
            Ok(url)
        } else {
            Err("Unanble to create url.")
        }
    }

    fn generate_report(&mut self) {
        match self.generate_link_from_arr() {
            Ok(url) => {
                let json: WeatherReport = reqwest::blocking::get(url)
                    .expect("error fetching data")
                    .json()
                    .expect("error parsing");
                self.report = Some(json);
            }
            Err(e) => panic!("Error: {}", e),
        }
    }

    pub fn _print(&self) {
        println!("{:?}", self.report);
    }

    pub fn get_weather_current(&mut self) -> Result<HashMap<String, String>, Box<dyn Error>> {
        if self.report.is_none() {
            self.generate_report();
        }

        let rep = &self.report.as_ref().unwrap().current;

        let map = HashMap::from([
            ("temp_c".to_string(), rep.temp_c.to_string()),
            ("temp_f".to_string(), rep.temp_f.to_string()),
            ("condition".to_string(), rep.condition.text.clone()),
            ("wind_mph".to_string(), rep.wind_mph.to_string()),
            ("wind_dir".to_string(), rep.wind_dir.to_string()),
            ("cloud".to_string(), rep.cloud.to_string()),
            ("feelslike_c".to_string(), rep.feelslike_c.to_string()),
            ("feelslike_f".to_string(), rep.feelslike_f.to_string()),
            ("gust_mph".to_string(), rep.gust_mph.to_string()),
        ]);
        Ok(map)
    }

    pub fn get_weather_forecast(&mut self) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {
        if self.report.is_none() {
            self.generate_report();
        }

        let forecast_arr = match &self.report {
            Some(rep) => &rep.forecast.forecastday,
            None => return Err("no report".into()),
        };

        let mut return_vec = Vec::with_capacity(3);

        for day_report in forecast_arr.iter() {
            let map = HashMap::from([
                ("date".to_string(), day_report.date.clone()),
                (
                    "maxtemp_c".to_string(),
                    day_report.day.maxtemp_c.to_string(),
                ),
                (
                    "maxtemp_f".to_string(),
                    day_report.day.maxtemp_f.to_string(),
                ),
                (
                    "mintemp_c".to_string(),
                    day_report.day.mintemp_c.to_string(),
                ),
                (
                    "mintemp_f".to_string(),
                    day_report.day.mintemp_f.to_string(),
                ),
                (
                    "totalpreicp_mm".to_string(),
                    day_report.day.totalprecip_mm.to_string(),
                ),
                (
                    "totalpreicp_in".to_string(),
                    day_report.day.totalprecip_in.to_string(),
                ),
                (
                    "daily_chance_of_rain".to_string(),
                    day_report.day.daily_chance_of_rain.to_string(),
                ),
                (
                    "daily_chance_of_snow".to_string(),
                    day_report.day.daily_chance_of_snow.to_string(),
                ),
                (
                    "condition".to_string(),
                    day_report.day.condition.text.clone(),
                ),
            ]);

            return_vec.push(map)
        }
        Ok(return_vec)
    }

    pub fn get_report_location(&self) -> String {
        let loc = match &self.report {
            Some(rep) => rep.location.name.clone(),
            None => {
                eprintln!("Report not found");
                process::exit(1);
            }
        };
        loc
    }
}
