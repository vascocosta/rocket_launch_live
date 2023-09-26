//! # Rocket Launch Live
//!
//! A type safe and asynchronous wrapper around the RocketLaunch.Live API.
//!
//! `rocket_launch_live` allows you to easily integrate your code asynchronously with the
//! [RocketLaunch.Live API]. Instead of dealing with low level details, the user can instanciate a
//! client with a valid API key and use a high level interface to this service.
//!
//! # Design
//!
//! [`RocketLaunchLive`] is the main struct, containing methods for each endpoint. The JSON data is
//! deserialised into meaningful model types defined in the [`api_models`] module. Each call to an
//! endpoint method returns a [`Response<T>`] which is generic over T, allowing tailored responses.
//! Depending on which method you call, the response contains a result field of type `Vec<T>` where
//! T can be of the type [`api_models::Company`], [`api_models::Launch`], [`api_models::Location`],
//! [`api_models::Mission`], [`api_models::Pad`], [`api_models::Tag`] or [`api_models::Vehicle`].
//!
//! This REST API provides access to a growing database of curated rocket launch data through the
//! following endpoints:
//!
//! * Companies
//! * Launches
//! * Locations
//! * Missions
//! * Pads
//! * Tags
//! * Vehicles
//!
//! # Examples
//!
//! ```
//! use rocket_launch_live::api_models::{Launch, Response};
//! use rocket_launch_live::{Direction, LaunchParamsBuilder, NaiveDate, RocketLaunchLive};
//! use std::{env, error::Error};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     // Read the API key from an environment variable.
//!     let api_key = env::var("RLL_API_KEY")?;
//!
//!     // Create an instance of RocketLaunchLive to access the API.
//!     let client = RocketLaunchLive::new(&api_key);
//!
//!     // Create an instance of LaunchParamsBuilder.
//!     // Set some parameters to filter out the launches we're interested in.
//!     let params = LaunchParamsBuilder::new()
//!         .country_code("US")
//!         .after_date(NaiveDate::from_ymd_opt(2023, 9, 1))?
//!         .search("ISS")
//!         .direction(Direction::Descending)
//!         .limit(10)
//!         .build();
//!
//!     // Call the launches endpoint method with the parameters set above.
//!     // This returns a Response from the API server asynchronously.
//!     // Generic type annotations since each endpoint has a specific response.
//!     let resp: Response<Launch> = client.launches(Some(params)).await?;
//!
//!     // Iterate over the the result field of the Response.
//!     for launch in resp.result {
//!         println!(
//!             "{} | {} | {}",
//!             launch.date_str, launch.vehicle.name, launch.name
//!         );
//!     }
//!
//!     Ok(())
//! }
//! ```
//! [RocketLaunch.Live API]: https://www.rocketlaunch.live/api

use api_models::Response;
pub use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::de::DeserializeOwned;
use std::error::Error;

pub mod api_models;
mod macros;

/// Represents the sorting order of results (ascending or descending).
pub enum Direction {
    Ascending,
    Descending,
}

/// Low level text representation of the API parameters sent to the server.
#[derive(Debug, Default)]
pub struct Params(Vec<String>);

/// Parameters used by multiple builders by composition.
#[derive(Default)]
pub struct CommonParams<'a> {
    id: Option<i64>,
    name: Option<&'a str>,
    state_abbr: Option<&'a str>,
    country_code: Option<&'a str>,
    slug: Option<&'a str>,
    page: Option<i64>,
}

/// Builder to generate the API parameters to filter calls to the companies endpoint.
#[derive(Default)]
pub struct CompanyParamsBuilder<'a> {
    common_params: CommonParams<'a>,
    inactive: Option<bool>,
}

impl<'a> CompanyParamsBuilder<'a> {
    /// Create a new builder for the company paramaters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the company id parameter.
    pub fn id(&mut self, id: i64) -> &mut Self {
        self.common_params.id = Some(id);

        self
    }

    /// Set the company name paramter.
    pub fn name(&mut self, name: &'a str) -> &mut Self {
        self.common_params.name = Some(name);

        self
    }

    /// Set the company country_code parameter.
    pub fn country_code(&mut self, country_code: &'a str) -> &mut Self {
        self.common_params.country_code = Some(country_code);

        self
    }

    /// Set the company slug paramter.
    pub fn slug(&mut self, slug: &'a str) -> &mut Self {
        self.common_params.slug = Some(slug);

        self
    }

    /// Set the company inactive parameter.
    pub fn inactive(&mut self, inactive: bool) -> &mut Self {
        self.inactive = Some(inactive);

        self
    }

    /// Set the company page parameter.
    pub fn page(&mut self, page: i64) -> &mut Self {
        self.common_params.page = Some(page);

        self
    }

    /// Build the low level company parameters from all the set parameters.
    pub fn build(&self) -> Params {
        let mut params: Vec<String> = Vec::new();

        add_param!(params, self.common_params.id, "id");
        add_param!(params, self.common_params.name, "name");
        add_param!(params, self.common_params.country_code, "country_code");
        add_param!(params, self.common_params.slug, "slug");
        add_param!(params, self.inactive, "inactive");
        add_param!(params, self.common_params.page, "page");

        Params(params)
    }
}

/// Builder to generate the API parameters to filter calls to the launches endpoint.
#[derive(Default)]
pub struct LaunchParamsBuilder<'a> {
    common_params: CommonParams<'a>,
    cospar_id: Option<&'a str>,
    after_date: Option<NaiveDate>,
    before_date: Option<NaiveDate>,
    modified_since: Option<NaiveDateTime>,
    location_id: Option<i64>,
    pad_id: Option<i64>,
    provider_id: Option<i64>,
    tag_id: Option<i64>,
    vehicle_id: Option<i64>,
    search: Option<&'a str>,
    limit: Option<i64>,
    direction: Option<Direction>,
}

impl<'a> LaunchParamsBuilder<'a> {
    /// Create a new builder for the launch paramaters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the launch id parameter.
    pub fn id(&mut self, id: i64) -> &mut Self {
        self.common_params.id = Some(id);

        self
    }

    /// Set the launch cospar_id parameter.
    pub fn cospar_id(&mut self, cospar_id: &'a str) -> &mut Self {
        self.cospar_id = Some(cospar_id);

        self
    }

    /// Set the launch after_date parameter.
    pub fn after_date(&mut self, after_date: Option<NaiveDate>) -> Result<&mut Self, &'static str> {
        match after_date {
            Some(date) => {
                self.after_date = Some(date);

                Ok(self)
            }
            None => Err("Could not parse date."),
        }
    }

    /// Set the launch before_date parameter.
    pub fn before_date(
        &mut self,
        before_date: Option<NaiveDate>,
    ) -> Result<&mut Self, &'static str> {
        match before_date {
            Some(date) => {
                self.before_date = Some(date);

                Ok(self)
            }
            None => Err("Could not parse date."),
        }
    }

    /// Set the launch modified_since parameter.
    pub fn modified_since(
        &mut self,
        date: Option<NaiveDate>,
        time: Option<NaiveTime>,
    ) -> Result<&mut Self, &'static str> {
        match date {
            Some(date) => match time {
                Some(time) => {
                    self.modified_since = Some(NaiveDateTime::new(date, time));

                    Ok(self)
                }
                None => Err("Could not parse time."),
            },
            None => Err("Could not parse date."),
        }
    }

    /// Set the launch location_id parameter.
    pub fn location_id(&mut self, location_id: i64) -> &mut Self {
        self.location_id = Some(location_id);

        self
    }

    /// Set the launch pad_id parameter.
    pub fn pad_id(&mut self, pad_id: i64) -> &mut Self {
        self.pad_id = Some(pad_id);

        self
    }

    /// Set the launch provider_id parameter.
    pub fn provider_id(&mut self, provider_id: i64) -> &mut Self {
        self.provider_id = Some(provider_id);

        self
    }

    /// Set the launch tag_id parameter.
    pub fn tag_id(&mut self, tag_id: i64) -> &mut Self {
        self.tag_id = Some(tag_id);

        self
    }

    /// Set the launch vehicle_id parameter.
    pub fn vehicle_id(&mut self, vehicle_id: i64) -> &mut Self {
        self.vehicle_id = Some(vehicle_id);

        self
    }

    /// Set the launch state_abbr parameter.
    pub fn state_abbr(&mut self, sate_abbr: &'a str) -> &mut Self {
        self.common_params.state_abbr = Some(sate_abbr);

        self
    }

    /// Set the launch country_code parameter.
    pub fn country_code(&mut self, country_code: &'a str) -> &mut Self {
        self.common_params.country_code = Some(country_code);

        self
    }

    /// Set the launch search parameter.
    pub fn search(&mut self, search: &'a str) -> &mut Self {
        self.search = Some(search);

        self
    }

    /// Set the launch slug parameter.
    pub fn slug(&mut self, slug: &'a str) -> &mut Self {
        self.common_params.slug = Some(slug);

        self
    }

    /// Set the launch limit parameter.
    pub fn limit(&mut self, limit: i64) -> &mut Self {
        self.limit = Some(limit);

        self
    }

    /// Set the launch direction parameter.
    pub fn direction(&mut self, direction: Direction) -> &mut Self {
        self.direction = Some(direction);

        self
    }

    /// Set the launch page parameter.
    pub fn page(&mut self, page: i64) -> &mut Self {
        self.common_params.page = Some(page);

        self
    }

    /// Build the low level launch parameters from all the set parameters.
    pub fn build(&self) -> Params {
        let mut params: Vec<String> = Vec::new();

        add_param!(params, self.common_params.id, "id");
        add_param!(params, self.cospar_id, "cospar_id");
        add_param!(params, self.after_date, "after_date");
        add_param!(params, self.before_date, "before_date");
        add_param!(params, self.location_id, "location_id");
        add_param!(params, self.pad_id, "pad_id");
        add_param!(params, self.provider_id, "provider_id");
        add_param!(params, self.tag_id, "tag_id");
        add_param!(params, self.vehicle_id, "vehicle_id");
        add_param!(params, self.common_params.state_abbr, "state_abbr");
        add_param!(params, self.common_params.country_code, "country_code");
        add_param!(params, self.search, "search");
        add_param!(params, self.common_params.slug, "slug");
        add_param!(params, self.limit, "limit");
        add_param!(params, self.common_params.page, "page");

        if let Some(modified_since) = self.modified_since {
            params.push(format!(
                "modified_since={}T{}Z",
                modified_since.date(),
                modified_since.time()
            ));
        }

        if let Some(direction) = &self.direction {
            match direction {
                Direction::Ascending => params.push(String::from("direction=asc")),
                Direction::Descending => params.push(String::from("direction=desc")),
            }
        }

        Params(params)
    }
}

/// Builder to generate the API parameters to filter calls to the locations endpoint.
#[derive(Default)]
pub struct LocationParamsBuilder<'a> {
    common_params: CommonParams<'a>,
}

impl<'a> LocationParamsBuilder<'a> {
    /// Create a new builder for the location parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the location id parameter.
    pub fn id(&mut self, id: i64) -> &mut Self {
        self.common_params.id = Some(id);

        self
    }

    /// Set the location name parameter.
    pub fn name(&mut self, name: &'a str) -> &mut Self {
        self.common_params.name = Some(name);

        self
    }

    /// Set the location state_abbr parameter.
    pub fn state_abbr(&mut self, state_abbr: &'a str) -> &mut Self {
        self.common_params.state_abbr = Some(state_abbr);

        self
    }

    /// Set the location country_code parameter.
    pub fn country_code(&mut self, country_code: &'a str) -> &mut Self {
        self.common_params.country_code = Some(country_code);

        self
    }

    /// Set the location page parameter.
    pub fn page(&mut self, page: i64) -> &mut Self {
        self.common_params.page = Some(page);

        self
    }

    /// Build the low level location parameters from all the set parameters.
    pub fn build(&self) -> Params {
        let mut params = Vec::new();

        add_param!(params, self.common_params.id, "id");
        add_param!(params, self.common_params.name, "name");
        add_param!(params, self.common_params.state_abbr, "state_abbr");
        add_param!(params, self.common_params.country_code, "country_code");
        add_param!(params, self.common_params.page, "page");

        Params(params)
    }
}

/// Builder to generate the API parameters to filter calls to the missions endpoint.
#[derive(Default)]
pub struct MissionParamsBuilder<'a> {
    common_params: CommonParams<'a>,
}

impl<'a> MissionParamsBuilder<'a> {
    /// Create a new builder for the mission parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the mission id parameter.
    pub fn id(&mut self, id: i64) -> &mut Self {
        self.common_params.id = Some(id);

        self
    }

    /// Set the mission name parameter.
    pub fn name(&mut self, name: &'a str) -> &mut Self {
        self.common_params.name = Some(name);

        self
    }

    /// Set the mission page parameter.
    pub fn page(&mut self, page: i64) -> &mut Self {
        self.common_params.page = Some(page);

        self
    }

    /// Build the low level mission parameters from all the set parameters.
    pub fn build(&self) -> Params {
        let mut params = Vec::new();

        add_param!(params, self.common_params.id, "id");
        add_param!(params, self.common_params.name, "name");
        add_param!(params, self.common_params.page, "page");

        Params(params)
    }
}

/// Builder to generate the API parameters to filter calls to the pads endpoint.
#[derive(Default)]
pub struct PadParamsBuilder<'a> {
    common_params: CommonParams<'a>,
}

impl<'a> PadParamsBuilder<'a> {
    /// Create a new builder for the pad parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the pad id parameter.
    pub fn id(&mut self, id: i64) -> &mut Self {
        self.common_params.id = Some(id);

        self
    }

    /// Set the pad name parameter.
    pub fn name(&mut self, name: &'a str) -> &mut Self {
        self.common_params.name = Some(name);

        self
    }

    /// Set the pad state_abbr parameter.
    pub fn state_abbr(&mut self, state_abbr: &'a str) -> &mut Self {
        self.common_params.state_abbr = Some(state_abbr);

        self
    }

    /// Set the pad country_code parameter.
    pub fn country_code(&mut self, country_code: &'a str) -> &mut Self {
        self.common_params.country_code = Some(country_code);

        self
    }

    /// Set the pad page parameter.
    pub fn page(&mut self, page: i64) -> &mut Self {
        self.common_params.page = Some(page);

        self
    }

    /// Build the low level pad parameters from all the set parameters.
    pub fn build(&self) -> Params {
        let mut params = Vec::new();

        add_param!(params, self.common_params.id, "id");
        add_param!(params, self.common_params.name, "name");
        add_param!(params, self.common_params.state_abbr, "state_abbr");
        add_param!(params, self.common_params.country_code, "country_code");
        add_param!(params, self.common_params.page, "page");

        Params(params)
    }
}

/// Builder to generate the API parameters to filter calls to the tags endpoint.
#[derive(Default)]
pub struct TagParamsBuilder<'a> {
    common_params: CommonParams<'a>,
    text: Option<&'a str>,
}

impl<'a> TagParamsBuilder<'a> {
    /// Create a new builder for the tag parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the tag id parameter.
    pub fn id(&mut self, id: i64) -> &mut Self {
        self.common_params.id = Some(id);

        self
    }

    /// Set the tag text parameter.
    pub fn text(&mut self, text: &'a str) -> &mut Self {
        self.text = Some(text);

        self
    }

    /// Set the tag page parameter.
    pub fn page(&mut self, page: i64) -> &mut Self {
        self.common_params.page = Some(page);

        self
    }

    /// Build the low level tag parameters from all the set parameters.
    pub fn build(&self) -> Params {
        let mut params = Vec::new();

        add_param!(params, self.common_params.id, "id");
        add_param!(params, self.text, "text");
        add_param!(params, self.common_params.page, "page");

        Params(params)
    }
}

/// Builder to generate the API parameters to filter calls to the vehicles endpoint.
#[derive(Default)]
pub struct VehicleParamsBuilder<'a> {
    common_params: CommonParams<'a>,
}

impl<'a> VehicleParamsBuilder<'a> {
    /// Create a new builder for the vehicle parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the vehicle id parameter.
    pub fn id(&mut self, id: i64) -> &mut Self {
        self.common_params.id = Some(id);

        self
    }

    /// Set the vehicle name parameter.
    pub fn name(&mut self, name: &'a str) -> &mut Self {
        self.common_params.name = Some(name);

        self
    }

    /// Set the vehicle page parameter.
    pub fn page(&mut self, page: i64) -> &mut Self {
        self.common_params.page = Some(page);

        self
    }

    /// Build the low level vehicle parameters from all the set parameters.
    pub fn build(&self) -> Params {
        let mut params = Vec::new();

        add_param!(params, self.common_params.id, "id");
        add_param!(params, self.common_params.name, "name");
        add_param!(params, self.common_params.page, "page");

        Params(params)
    }
}

/// API client containing all the public endpoint methods.
pub struct RocketLaunchLive<'a> {
    key: &'a str,
    url: &'a str,
}

impl<'a> RocketLaunchLive<'a> {
    /// Create a new API client with an API key.
    pub fn new(key: &'a str) -> Self {
        Self {
            key,
            url: "https://fdo.rocketlaunch.live",
        }
    }

    async fn request<T: DeserializeOwned>(
        &self,
        endpoint: &'a str,
        params: Option<Params>,
    ) -> Result<Response<T>, Box<dyn Error>> {
        let client = reqwest::Client::new();
        let resp: Response<T> = client
            .get(format!(
                "{}/json/{}?{}",
                self.url,
                endpoint,
                params.unwrap_or_default().0.join("&")
            ))
            .header("Authorization", format!("Bearer {}", self.key))
            .send()
            .await?
            .json()
            .await?;

        Ok(resp)
    }

    /// Retrieve all companies in the database (optionally filtered by params) or an error.
    pub async fn companies<T: DeserializeOwned>(
        &self,
        params: Option<Params>,
    ) -> Result<Response<T>, Box<dyn Error>> {
        self.request("companies", params).await
    }

    /// Retrieve all launches in the database (optionally filtered by params) or an error.
    pub async fn launches<T: DeserializeOwned>(
        &self,
        params: Option<Params>,
    ) -> Result<Response<T>, Box<dyn Error>> {
        self.request("launches", params).await
    }

    /// Retrieve all locations in the database (optionally filtered by params) or an error.
    pub async fn locations<T: DeserializeOwned>(
        &self,
        params: Option<Params>,
    ) -> Result<Response<T>, Box<dyn Error>> {
        self.request("locations", params).await
    }

    /// Retrieve all missions in the database (optionally filtered by params) or an error.
    pub async fn missions<T: DeserializeOwned>(
        &self,
        params: Option<Params>,
    ) -> Result<Response<T>, Box<dyn Error>> {
        self.request("missions", params).await
    }

    /// Retrieve all pads in the database (optionally filtered by params) or an error.
    pub async fn pads<T: DeserializeOwned>(
        &self,
        params: Option<Params>,
    ) -> Result<Response<T>, Box<dyn Error>> {
        self.request("pads", params).await
    }

    /// Retrieve all tags in the database (optionally filtered by params) or an error.
    pub async fn tags<T: DeserializeOwned>(
        &self,
        params: Option<Params>,
    ) -> Result<Response<T>, Box<dyn Error>> {
        self.request("tags", params).await
    }

    /// Retrieve all vehicles in the database (optionally filtered by params) or an error.
    pub async fn vehicles<T: DeserializeOwned>(
        &self,
        params: Option<Params>,
    ) -> Result<Response<T>, Box<dyn Error>> {
        self.request("vehicles", params).await
    }
}
