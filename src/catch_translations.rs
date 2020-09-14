use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
#[allow(non_snake_case)]
pub struct CatchTranslations {
  pub catch_USen: String,
  pub catch_EUen: String,
  pub catch_EUde: String,
  pub catch_EUes: String,
  pub catch_USes: String,
  pub catch_EUfr: String,
  pub catch_USfr: String,
  pub catch_EUit: String,
  pub catch_EUnl: String,
  pub catch_CNzh: String,
  pub catch_TWzh: String,
  pub catch_JPja: String,
  pub catch_KRko: String,
  pub catch_EUru: String,
}