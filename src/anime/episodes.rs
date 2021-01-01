use super::super::utils::httpc::JikanHttpClient;
use chrono::{DateTime, FixedOffset};
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct Episode {
  #[serde(rename = "aired")]
  aired_at: DateTime<FixedOffset>,
  #[serde(rename = "episode_id")]
  id: u32,
  filler: bool,
  forum_url: Option<String>,
  recap: bool,
  title: String,
  title_japanese: Option<String>,
  title_romanji: Option<String>,
  video_url: Option<String>,
}

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct Episodes {
  episodes: Vec<Episode>,
  episodes_last_page: u32,
}

impl Episodes {
  pub async fn from_id_at_page(
    client: &JikanHttpClient,
    id: u32,
    page: u32,
  ) -> Result<Self, Box<dyn Error>> {
    let response = client
      .get(&format!("anime/{}/episodes/{}", id, page)[..])
      .send()
      .await?;
    println!("{:?}", response);

    Ok(response.json().await?)
  }

  pub async fn from_id(client: &JikanHttpClient, id: u32) -> Result<Self, Box<dyn Error>> {
    Episodes::from_id_at_page(client, id, 1).await
  }
}

#[cfg(test)]
mod tests {
  use super::super::{test_helper, test_helper::AnimeTestSuite};
  use super::*;
  use rand::Rng;
  use serial_test::serial;
  use std::error::Error;
  use std::thread;

  #[actix_rt::test]
  #[serial]
  async fn can_get_episodes_by_id() -> Result<(), Box<dyn Error>> {
    let client = JikanHttpClient::new();

    for AnimeTestSuite { id, name } in test_helper::get_valid_animes(10) {
      let episodes = Episodes::from_id(&client, id).await;

      match episodes {
        Ok(_) => assert!(episodes.is_ok(), "{}", name),
        Err(_) => continue,
      }

      thread::sleep(test_helper::REQUEST_DELAY);
    }

    Ok(())
  }

  #[actix_rt::test]
  #[serial]
  async fn can_handle_episodes_empty_episodes() -> Result<(), Box<dyn Error>> {
    let client = JikanHttpClient::new();

    for AnimeTestSuite { id, name } in test_helper::get_invalid_animes() {
      assert_eq!(
        Episodes::from_id(&client, id).await?.episodes.len(),
        0,
        "Episodes for anime \"{}\" is not 0",
        name,
      );

      thread::sleep(test_helper::REQUEST_DELAY);
    }

    Ok(())
  }

  #[actix_rt::test]
  #[serial]
  async fn can_get_episodes_by_id_at_page() -> Result<(), Box<dyn Error>> {
    let client = JikanHttpClient::new();

    for AnimeTestSuite { id, name } in test_helper::get_valid_animes(10) {
      let mut rng = rand::thread_rng();
      let page = rng.gen_range(2..10);
      let episodes = Episodes::from_id_at_page(&client, id, page).await;

      match episodes {
        Ok(_) => assert!(episodes.is_ok(), "{}", name),
        Err(_) => continue,
      }

      thread::sleep(test_helper::REQUEST_DELAY);
    }

    Ok(())
  }

  #[actix_rt::test]
  #[serial]
  async fn can_handle_episodes_empty_episodes_at_page() -> Result<(), Box<dyn Error>> {
    let client = JikanHttpClient::new();

    for AnimeTestSuite { id, name } in test_helper::get_invalid_animes() {
      let mut rng = rand::thread_rng();
      let page = rng.gen_range(2..10);
      assert_eq!(
        Episodes::from_id_at_page(&client, id, page)
          .await?
          .episodes
          .len(),
        0,
        "Episodes for anime \"{}\" is not 0",
        name,
      );

      thread::sleep(test_helper::REQUEST_DELAY);
    }

    Ok(())
  }
}
