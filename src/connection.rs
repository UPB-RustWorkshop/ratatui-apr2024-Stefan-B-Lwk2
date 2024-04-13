use chrono::{DateTime, Local};
use reqwest::Error;
use serde::Deserialize;
use serde::Serialize;
//use serde_json::Result;
//use reqwest::blocking;


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CityInfo {
    // TODO: define elements in the structure
    city: String,
    weather: Weather,
  pub  conditions: Conditions,
    current_time: String,
    sunrise: String,
    sunset: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Weather {
    main_weather: String,
    description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Conditions {
  pub  temp: f32,
    feels_like: f32,
    temp_min: f32,
    temp_max: f32,
    pressure: f32,
    humidity: f32,
    clouds_pct: f32,
    wind: Wind ,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Wind{
    speed: f32,
    deg: f32,
    gust: Option<f64>,
}


#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Orase{
    cities: Vec<String>,
}

/// Method that is handling the request to the openweather api,
/// parsing the response
///
/// IP: 34.116.205.113
/// Port: 3000
///
/// Returns weather details about a certain city
/// 

pub async fn get_temperature(city: String) -> Result<CityInfo,Error> {
    let client = reqwest::Client::new();
    let my_body = format!("{{\"city\": \"{}\"}}",city);

    let res = client.post("http://34.116.205.113:3000/cities/current_weather")
        .header("Content-Type", "application/json")
        .body(my_body)
        .send().await;

    match res {
        Ok(response) => {
            // Check status code
            // Parse response
        let read = response.json::<CityInfo>();
        let p: CityInfo = read.await.unwrap();
        return Ok(p);
        //println!("Oras Temp {}",p.conditions.temp);

        },
        Err(_error) => {
            // Handle error
            return Err(_error);
        }
    
    }
    
}

pub async fn get_cities() -> Vec<String> {
    let client = reqwest::Client::new();
    let res = client.get("http://34.116.205.113:3000/cities")
                                             .send().await;

    let mut cities: Orase = Orase {
        cities: vec![],
    };

    match res {
        Ok(response) => {
            // Check status code
            // Parse response
            let read = response.json::<Orase>();
            cities = read.await.unwrap();
        },
        Err(_error) => {
                // Handle error
        }
    }

    cities.cities
}