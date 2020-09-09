use serde::{Serialize, Deserialize};

use super::name;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Fossil {
  pub file_name: String,
  pub name: name::Name,
  pub price: isize,
  pub museum_phrase: String,
  #[serde(alias = "image_uri")]
  pub image_uri: String,
  pub part_of: String,
}