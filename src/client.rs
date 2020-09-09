use reqwest::{Client, Method, Response};
use std::collections::HashMap;
use std::{error};

use crate::fish;

#[derive(Debug, Clone)]
pub struct Acnh {
  client: Client,
  pub prefix: String,
}

impl Acnh {
  pub fn new() -> Self {
	Acnh {
	  client: Client::new(),
	  prefix: String::from("http://acnhapi.com/v1a/"),
	}
  }

  pub async fn fishes(&self) -> Result<Vec<fish::Fish>, Box<dyn error::Error>> {
	let response = self.get("fish").await?;

	match response.json::<Vec<fish::Fish>>().await {
	  Ok(result) => Ok(result),
	  Err(e) => {
		Err(Box::new(e))
	  }
	}
  }

  pub async fn fish(&self, id: &str) -> Result<fish::Fish, Box<dyn error::Error>> {
	let url = ["fish/", id].concat();

	let response = self.get(&url).await?;

	match response.json::<fish::Fish>().await {
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
	  let _result = acnh.fishes().await.unwrap();

	  assert!(true);
	}

	do_request()
  }

  #[test]
  fn test_get_fish() {
	#[tokio::main]
	async fn do_request() {
	  let acnh = Acnh::new();
	  let result = acnh.fish("1").await.unwrap();

	  assert_eq!(result.id, 1);
	}

	do_request()
  }
}
