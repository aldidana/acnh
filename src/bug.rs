use serde::{Serialize, Deserialize};

use super::name;
use super::availability;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Bug {
  pub id: i8,
  pub file_name: String,
  pub name: name::Name,
  pub availability: availability::Availability,
  pub price: isize,
  pub price_flick: isize,
  pub catch_phrase: String,
  pub museum_phrase: String,
  #[serde(alias = "image_uri")]
  pub image_uri: String,
  #[serde(alias = "icon_uri")]
  pub icon_uri: String,
}