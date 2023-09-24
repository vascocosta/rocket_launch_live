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
