# Rocket Launch Live

A type safe and asynchronous wrapper around the RocketLaunch.Live API.

`rocket_launch_live` allows you to easily integrate your code asynchronously with the
[RocketLaunch.Live API]. Instead of dealing with low level details, the user can instanciate a
client with a valid API key and use a high level interface to this service.

# Design

[`RocketLaunchLive`] is the main struct, containing methods for each endpoint. The JSON data is
deserialised into meaningful model types defined in the [`api_models`] module. Each call to an
endpoint method returns a [`Response<T>`] which is generic over T, allowing tailored responses.
Depending on which method you call, the response contains a result field of type `Vec<T>` where
T can be of the type [`api_models::Company`], [`api_models::Launch`], [`api_models::Location`],
[`api_models::Mission`], [`api_models::Pad`], [`api_models::Tag`] or [`api_models::Vehicle`].

This REST API provides access to a growing database of curated rocket launch data through the
following endpoints:

* Companies
* Launches
* Locations
* Missions
* Pads
* Tags
* Vehicles

# Examples

```rust
use rocket_launch_live::api_models::{Launch, Response};
use rocket_launch_live::{Direction, LaunchParamsBuilder, NaiveDate, RocketLaunchLive};
use std::{env, error::Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Read the API key from an environment variable.
    let api_key = env::var("RLL_API_KEY")?;

    // Create an instance of RocketLaunchLive to access the API.
    let client = RocketLaunchLive::new(&api_key);

    // Create an instance of LaunchParamsBuilder.
    // Set some parameters to filter out the launches we're interested in.
    let params = LaunchParamsBuilder::new()
        .country_code("US")
        .after_date(NaiveDate::from_ymd_opt(2023, 9, 1))?
        .search("ISS")
        .direction(Direction::Descending)
        .limit(10)
        .build();

    // Call the launches endpoint method with the parameters set above.
    // This returns a Response from the API server asynchronously.
    // Generic type annotations since each endpoint has a specific response.
    let resp: Response<Launch> = client.launches(Some(params)).await?;

    // Iterate over the the result field of the Response.
    for launch in resp.result {
        println!(
            "{} | {} | {}",
            launch.date_str, launch.vehicle.name, launch.name
        );
    }

    Ok(())
}
```
[RocketLaunch.Live API]: https://www.rocketlaunch.live/api