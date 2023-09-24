/// API model type definitions.
use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Response<T> {
    pub errors: Option<Vec<String>>,
    pub valid_auth: bool,
    pub count: Option<i64>,
    pub limit: Option<i64>,
    pub total: Option<i64>,
    pub last_page: Option<i64>,
    pub result: Vec<T>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Country {
    pub name: String,
    pub code: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Company {
    pub id: Option<i64>,
    pub name: String,
    pub inactive: bool,
    pub country: Country,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Launch {
    pub id: Option<i64>,
    pub cospar_id: Option<String>,
    pub sort_date: String,
    pub name: String,
    pub provider: Provider,
    pub vehicle: Vehicle,
    pub pad: Pad,
    pub missions: Vec<Mission>,
    pub mission_description: Option<String>,
    pub launch_description: String,
    pub win_open: Value,
    pub t0: Option<String>,
    pub win_close: Value,
    pub est_date: EstDate,
    pub date_str: String,
    pub tags: Vec<Tag>,
    pub slug: String,
    pub weather_summary: Value,
    pub weather_temp: Value,
    pub weather_condition: Value,
    pub weather_wind_mph: Value,
    pub weather_icon: Value,
    pub weather_updated: Value,
    pub quicktext: String,
    pub media: Vec<Medum>,
    pub result: Option<i64>,
    pub suborbital: bool,
    pub modified: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Provider {
    pub id: Option<i64>,
    pub name: String,
    pub slug: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Vehicle {
    pub id: Option<i64>,
    pub name: String,
    pub company_id: Option<i64>,
    pub slug: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Pad {
    pub id: Option<i64>,
    pub name: String,
    pub location: Location,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub id: Option<i64>,
    pub name: String,
    pub state: Option<String>,
    pub statename: Option<String>,
    pub country: String,
    pub slug: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mission {
    pub id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EstDate {
    pub month: Option<i64>,
    pub day: Option<i64>,
    pub year: Option<i64>,
    pub quarter: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tag {
    pub id: Option<i64>,
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Medum {
    pub id: Option<i64>,
    pub media_url: Option<String>,
    pub youtube_vidid: String,
    pub featured: bool,
    pub ldfeatured: bool,
    pub approved: bool,
}
