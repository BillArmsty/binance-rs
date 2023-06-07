use serde::{ Deserialize, Serialize };
use serde::de;
use serde::Deserializer;

//Struct that represents single bid or ask which is a string array of length 2
#[derive(Deserialize, Serialize, Debug)]
pub struct OfferData {
    #[serde(deserialize_with = "de_float_from_str")]
    pub price: f32,
    #[serde(deserialize_with = "de_float_from_str")]
    pub size: f32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DepthStreamData {
    pub last_update_id: Option<i32>,
    pub bids: Vec<OfferData>,
    pub asks: Vec<OfferData>,
}

pub fn de_float_from_str<'a, D>(deserializer: D) -> Result<f32, D::Error> where D: Deserializer<'a> {
    let str_val = String::deserialize(deserializer)?;
    str_val.parse::<f32>().map_err(de::Error::custom)
}

#[derive(Deserialize, Debug)]
pub struct DepthStreamDataWrapper {
    pub stream: String,
    pub data: DepthStreamData,
}
