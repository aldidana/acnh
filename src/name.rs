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