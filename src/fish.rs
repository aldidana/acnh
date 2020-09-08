use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[allow(non_snake_case)]
pub struct Name {
  pub name_USen: String,
  pub name_EUen: String,
  pub name_EUde: String,
  pub name_EUes: String,
  pub name_USes: String,
  pub name_EUfr: String,
  pub name_USfr: String,
  pub name_EUit: String,
  pub name_EUnl: String,
  pub name_CNzh: String,
  pub name_TWzh: String,
  pub name_JPja: String,
  pub name_KRko: String,
  pub name_EUru: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Availability {
  pub month_northern: String,
  pub month_southern: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub time: Option<String>,
  #[serde(alias = "isAllDay")]
  pub is_all_day: bool,
  #[serde(alias = "isAllYear")]
  pub is_all_year: bool,
  pub location: String,
  pub rarity: String,
  pub month_array_northern: Vec<i8>,
  pub month_array_southern: Vec<i8>,
  pub time_array: Vec<i8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Fish {
  pub id: i8,
  // #[serde(alias = "file-name")]
  pub file_name: String,
  pub name: Name,
  pub availability: Availability,
  pub shadow: String,
  pub price: isize,
  pub price_cj: isize,
  pub catch_phrase: String,
  pub museum_phrase: String,
  #[serde(alias = "image_uri")]
  pub image_uri: String,
  #[serde(alias = "icon_uri")]
  pub icon_uri: String,
}