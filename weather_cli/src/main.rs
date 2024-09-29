use std::io;

use colored::*;
use serde::Deserialize;

// Struct to deserialize the JSON response from openWeatherMap API
#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
}

// Struct to represent weather description
#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

// Strut to represent the main weather parameter
#[derive(Deserialize, Debug)]
struct Main {
    temp: f64,
    humidity: f64,
    pressure: f64,
}

// Strut to represent wind information
#[derive(Deserialize, Debug)]
struct Wind {
    speed: f64,
}

// Function to get the weather information from OpenWeatherMap API
fn get_weather_info(city: &str, country_code: &str, api_key: &str) -> Result<WeatherResponse, reqwest::Error> {
    let url: String = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={},{}&units=metric&appid={}",
        city, country_code, api_key
    );
    // non-blocking i/o - synchronous
    let response = reqwest::blocking::get(&url)?;
    let response_json: WeatherResponse = response.json::<WeatherResponse>()?;
    Ok(response_json)
}

// Function to display the weather information
fn display_weather_info(response: &WeatherResponse) {
//     Extract the weather information from the response
    let description: &String = &response.weather[0].description;
    let temperature: f64 = response.main.temp;
    let humidity: f64 = response.main.humidity;
    let pressure: f64 = response.main.pressure;
    let wind_speed: f64 = response.wind.speed;

    let weather_text: String = format!(
        "Weather in {}: {} {}
        > Temperature: {:.1}℃,
        > Humidity: {:.1}%,
        > Pressure: {:.1} hPa,
        > wind_speed: {:.1} m/s",
        response.name,
        description,
        // this is a function to get tem emoji
        get_temp_emoji(temperature),
        temperature,
        humidity,
        pressure,
        wind_speed,
    );

    // Coloring the weather text based on weather conditions
    let weather_text_colored = match description.as_str() {
        "clear sky" => weather_text.bright_yellow(),
        "few clouds" | "scattered clouds" | "broken clouds" => weather_text.bright_blue(),
        "overcast clouds" | "mist" | "haze" | "smoke" | "sand" | "dust" | "fog" | "squalls" => weather_text.dimmed(),
        "shower rain" | "rain" | "thunderstorm" | "snow" => weather_text.bright_cyan(),
        _ => weather_text.normal(),
    };

    // Printing the colored weather information
    println!("{}", weather_text_colored);

}

//     Function to get emoji based on temperature
fn get_temp_emoji(temperature: f64) -> &'static str {
    if temperature < 0.0 {
        "❄️"
    } else if temperature >= 0.0 && temperature < 10.0 {
        "☁️"
    } else if temperature >= 10.0 && temperature < 20.0 {
        "⛅"
    } else if temperature >= 20.0 && temperature < 30.0 {
        "🌤️"
    } else {
        "🔥"
    }
}

fn main() {

    println!("{}", "Welcome to Weather Station!".bright_yellow()); // Displaying welcome message

    loop {
        println!("{}", "Please enter the name of the city:".bright_green()); // Prompting user to enter city name

        let mut city = String::new();
        io::stdin().read_line(&mut city).expect("Failed to read input"); // Reading user input for city name
        let city = city.trim();

        println!("{}", "Please enter the country code (e.g., US for United States):".bright_green()); // Prompting user to enter country code

        let mut country_code = String::new();
        io::stdin().read_line(&mut country_code).expect("Failed to read input"); // Reading user input for country code
        let country_code = country_code.trim();

        // Get your API key from OpenWeatherMap
        let api_key = "b210e465006f18c6a031a9db1bc81c3e";

        // Calling the function to fetch weather information
        match get_weather_info(&city, &country_code, api_key) {
            Ok(response) => {
                display_weather_info(&response); // Displaying weather information
            }
            Err(err) => {
                eprintln!("Error: {}", err); // Printing error message in case of failure
            }
        }

        println!("{}", "Do you want to search for weather in another city? (yes/no):".bright_green()); // Prompting user to continue or exit
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input"); // Reading user input for continuation
        let input = input.trim().to_lowercase();

        if input != "yes" {
            println!("Thank you for using our software!");
            break; // Exiting the loop if user doesn't want to continue
        }
    }

}
