use crate::weather_report::WeatherReport;
use std::collections::HashMap;
use std::error::Error;

pub struct WeatherFetcher {
    city: String,
    state: String,
    country: String,
    key: String,
    url: String,
    report: Option<WeatherReport>,
    //report_valid: bool,
}

impl WeatherFetcher {
    pub fn new() -> Self {
        Self {
            city: String::new(),
            state: String::new(),
            country: String::new(),
            key: String::new(),
            url: String::new(),
            report: None,
            //report_valid: false,
        }
    }

    pub fn set_location(&mut self, city: &str, state: &str, country: &str) {
        self.city.push_str(city);
        self.state.push_str(state);
        self.country.push_str(country);
    }

    pub fn set_key(&mut self, key: &str) {
        self.key.push_str(key);
    }

    fn _generate_link(&mut self) -> Result<String, &str> {
        if !self.city.is_empty() && !self.state.is_empty() && !self.key.is_empty() {
            let url = format!(
                "http://api.weatherapi.com/v1/current.json?key={}&q={},{},{}&aqi=no",
                self.key, self.city, self.state, self.country
            );
            self.url.push_str(&url);
            Ok(url)
        } else {
            Err("Unanble to create url.")
        }
    }

    fn generate_link_forecast(&mut self) -> Result<String, &str> {
        if !self.city.is_empty() && !self.state.is_empty() && !self.key.is_empty() {
            let url = format!("http://api.weatherapi.com/v1/forecast.json?key={}&q={},{},{}&days=3&aqi=no&alerts=no
            ", self.key, self.city, self.state, self.country );
            self.url.push_str(&url);
            Ok(url)
        } else {
            Err("Unanble to create url.")
        }
    }

    fn generate_report(&mut self) {
        match self.generate_link_forecast() {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_gen_pass() {
        let result = String::from("http://api.weatherapi.com/v1/current.json?key=408f3f7ccf3144af97d150358212910&q=Ames,IA,USA&aqi=no");
        let city = String::from("Ames");
        let state = String::from("IA");
        let country = String::from("USA");
        let key = String::from("408f3f7ccf3144af97d150358212910");

        let mut wf = WeatherFetcher::new();
        wf.set_location(&city, &state, &country);
        wf.set_key(&key);

        assert_eq!(result, wf._generate_link().unwrap());
    }
    #[test]
    #[should_panic]
    fn test_no_key_set() {
        let city = String::from("Ames");
        let state = String::from("IA");
        let country = String::from("USA");
        let mut wf = WeatherFetcher::new();
        wf.set_location(&city, &state, &country);

        match wf._generate_link() {
            Ok(i) => drop(i),
            Err(e) => panic!("Error: {}", e),
        };
    }

    #[test]
    #[should_panic]
    fn test_get_weather_method() {
        let mut wf = WeatherFetcher::new();
        match wf.get_weather_current() {
            Ok(i) => drop(i),
            Err(e) => panic!("Error:{}", e),
        }
    }
}
