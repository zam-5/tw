use std::env;
use std::fs::read_to_string;
use std::process;
mod weather_fetcher;
mod weather_report;
use weather_fetcher::WeatherFetcher;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("No location entered");
        process::exit(0);
    }

    let key = match read_to_string("./key") {
        Ok(k) => k,
        Err(_) => match env::var("TW_KEY") {
            Ok(key) => key,
            Err(_) => {
                eprintln!("Key not found");
                process::exit(1);
            }
        },
    };

    let mut wf = WeatherFetcher::new();
    wf.set_location_vec(args);
    wf.set_key(&key);

    let weather_forcast = wf.get_weather_forecast().unwrap();
    let weather_current = wf.get_weather_current().unwrap();

    println!(
        "\tIn {} it is {}°F and {}.",
        wf.get_report_location(),
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
