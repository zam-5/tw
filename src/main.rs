use std::env;
use std::fs::read_to_string;
use std::process;
mod weather_fetcher;
mod weather_report;
use weather_fetcher::WeatherFetcher;

fn main() {
    let args: Vec<String> = env::args().collect();
    let key = match read_to_string("./key") {
        Ok(k) => k,
        Err(_) => {
            eprintln!("Key not found");
            process::exit(1);
        }
    };

    let loc = match Location::new(args) {
        Ok(loc) => loc,
        Err(str) => panic!("Error: {}", str),
    };

    let mut wf = WeatherFetcher::new();
    wf.set_location(&loc.city, &loc.state, &loc.country);
    wf.set_key(&key);

    let weather_forcast = wf.get_weather_forecast().unwrap();
    let weather_current = wf.get_weather_current().unwrap();

    println!(
        "\tIn {}, {} it is {} degrees and {}.",
        &loc.city,
        &loc.state.to_uppercase(),
        weather_current
            .get("temp_f")
            .expect("Temperature not found"),
        weather_current
            .get("condition")
            .expect("Condition not found")
            .to_lowercase()
    );

    println!(
        "\tWind is {} mph out of the {}, with gusts up to {} mph.\n\tFeels like: {} degrees.\n",
        weather_current
            .get("wind_mph")
            .expect("Wind speed not found"),
        weather_current
            .get("wind_dir")
            .expect("Wind direction not found"),
        weather_current
            .get("gust_mph")
            .expect("Gust speed not found"),
        weather_current
            .get("feelslike_f")
            .expect("Feels like not found"),
    );

    for item in weather_forcast.iter() {
        println!(
            "\t{}:\n\tHigh: {}, Low: {}\n\tChance of Rain/Snow: {}%/{}%\n",
            item.get("date").expect("date not found"),
            item.get("maxtemp_f").expect("high not found"),
            item.get("mintemp_f").expect("low not found"),
            item.get("daily_chance_of_rain").expect("rain not found"),
            item.get("daily_chance_of_snow").expect("snow not found")
        );
    }
}
struct Location {
    pub city: String,
    pub state: String,
    pub country: String,
}

impl Location {
    pub fn new(args: Vec<String>) -> Result<Location, String> {
        let country = match env::var("COUNTRY") {
            Ok(c) => c,
            Err(_e) => String::from("USA"),
        };

        match args.len() {
            1 => Err("Default location not set".to_string()),
            2 => Err("Please provide a city and a state".to_string()),
            3 => Ok(Location {
                city: args[1].to_string(),
                state: args[2].to_string(),
                country,
            }),
            _ => Err("Too many arguments".to_string()),
        }
    }
}
