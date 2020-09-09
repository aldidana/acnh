use serde::{Serialize, Deserialize};

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
  #[serde(skip_serializing_if = "Option::is_none")]
  pub location: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub rarity: Option<String>,
  pub month_array_northern: Vec<i8>,
  pub month_array_southern: Vec<i8>,
  pub time_array: Vec<i8>,
}