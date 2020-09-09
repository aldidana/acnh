use reqwest::{Client, Method, Response};
use std::{error};

use super::fish;
use super::sea_creature;
use super::bug;

#[derive(Debug, Clone)]
pub struct Acnh {
  client: Client,
  pub prefix: String,
}

impl Acnh {
  pub fn new() -> Self {
	Acnh {
	  client: Client::new(),
	  prefix: String::from("https://acnhapi.com/v1a/"),
	}
  }

  pub async fn fish(&self) -> Result<Vec<fish::Fish>, Box<dyn error::Error>> {
	let response = self.get("fish").await?;

	match response.json::<Vec<fish::Fish>>().await {
	  Ok(result) => Ok(result),
	  Err(e) => {
		Err(Box::new(e))
	  }
	}
  }

  pub async fn fish_by_id(&self, id: &str) -> Result<fish::Fish, Box<dyn error::Error>> {
	let url = ["fish/", id].concat();

	let response = self.get(&url).await?;

	match response.json::<fish::Fish>().await {
	  Ok(result) => Ok(result),
	  Err(e) => {
		Err(Box::new(e))
	  }
	}
  }

  pub async fn sea_creatures(&self) -> Result<Vec<sea_creature::SeaCreature>, Box<dyn error::Error>> {
	let response = self.get("sea").await?;

	match response.json::<Vec<sea_creature::SeaCreature>>().await {
	  Ok(result) => Ok(result),
	  Err(e) => {
		Err(Box::new(e))
	  }
	}
  }

  pub async fn sea_creature_by_id(&self, id: &str) -> Result<sea_creature::SeaCreature, Box<dyn error::Error>> {
	let url = ["sea/", id].concat();

	let response = self.get(&url).await?;

	match response.json::<sea_creature::SeaCreature>().await {
	  Ok(result) => Ok(result),
	  Err(e) => {
		Err(Box::new(e))
	  }
	}
  }

  pub async fn bugs(&self) -> Result<Vec<bug::Bug>, Box<dyn error::Error>> {
	let response = self.get("bugs").await?;

	match response.json::<Vec<bug::Bug>>().await {
	  Ok(result) => Ok(result),
	  Err(e) => {
		Err(Box::new(e))
	  }
	}
  }

  pub async fn bug_by_id(&self, id: &str) -> Result<bug::Bug, Box<dyn error::Error>> {
	let url = ["bugs/", id].concat();

	let response = self.get(&url).await?;

	match response.json::<bug::Bug>().await {
	  Ok(result) => Ok(result),
	  Err(e) => {
		Err(Box::new(e))
	  }
	}
  }

  async fn get(&self, url: &str) -> Result<Response, Box<dyn error::Error>> {
	let prefix = self.prefix.to_string();
	let url = [prefix, url.to_string()].concat();

	let builder = self
	  .client
	  .request(Method::GET, &url);

	match builder.send().await?.error_for_status() {
	  Ok(res) => {
		Ok(res)
	  }
	  Err(e) => {
		Err(Box::new(e))
	  }
	}
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_all_fish() {
	#[tokio::main]
	async fn do_request() {
	  let acnh = Acnh::new();
	  let _result = acnh.fish().await.unwrap();

	  assert!(true);
	}

	do_request()
  }

  #[test]
  fn test_get_fish() {
	#[tokio::main]
	async fn do_request() {
	  let acnh = Acnh::new();
	  let result = acnh.fish_by_id("1").await.unwrap();

	  assert_eq!(result.id, 1);
	}

	do_request()
  }

  #[test]
  fn test_get_sea_creatures() {
	#[tokio::main]
	async fn do_request() {
	  let acnh = Acnh::new();
	  let _result = acnh.sea_creatures().await.unwrap();

	  assert!(true);
	}

	do_request()
  }

  #[test]
  fn test_get_sea_creature() {
	#[tokio::main]
	async fn do_request() {
	  let acnh = Acnh::new();
	  let result = acnh.sea_creature_by_id("1").await.unwrap();

	  assert_eq!(result.id, 1);
	}

	do_request()
  }

  #[test]
  fn test_get_bugs() {
	#[tokio::main]
	async fn do_request() {
	  let acnh = Acnh::new();
	  let _result = acnh.bugs().await.unwrap();

	  assert!(true);
	}

	do_request()
  }

  #[test]
  fn test_get_bug() {
	#[tokio::main]
	async fn do_request() {
	  let acnh = Acnh::new();
	  let result = acnh.bug_by_id("1").await.unwrap();

	  assert_eq!(result.id, 1);
	}

	do_request()
  }
}
