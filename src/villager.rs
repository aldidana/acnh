use serde::{Serialize, Deserialize};

use super::name;
use super::catch_translations;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Villager {
  pub id: isize,
  pub file_name: String,
  pub name: name::Name,
  pub personality: String,
  pub birthday_string: String,
  pub birthday: String,
  pub species: String,
  pub gender: String,
  pub subtype: String,
  pub hobby: String,
  pub catch_phrase: String,
  #[serde(alias = "image_uri")]
  pub image_uri: String,
  #[serde(alias = "icon_uri")]
  pub icon_uri: String,
  pub bubble_color: String,
  pub text_color: String,
  pub saying: String,
  pub catch_translations: catch_translations::CatchTranslations,
}