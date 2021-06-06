use crate::common::error::JikanError;
use crate::common::request::RequestMetadata;
use crate::utils::httpc::JikanHttpClient;
use derive_getters::Getters;
use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum VoiceActorLanguage {
  Brazilian,
  English,
  French,
  German,
  Hebrew,
  Hungarian,
  Italian,
  Japanese,
  Korean,
  Spanish,
  #[serde(other)]
  Unknown,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum CharacterRole {
  Main,
  Supporting,
  #[serde(other)]
  Unknown,
}

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct VoiceActor {
  #[serde(rename = "mal_id")]
  id: u32,
  image_url: String,
  language: VoiceActorLanguage,
  name: String,
  url: String,
}

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct Character {
  #[serde(rename = "mal_id")]
  id: u32,
  image_url: String,
  name: String,
  role: CharacterRole,
  url: String,
  voice_actors: Vec<VoiceActor>,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub enum StaffPosition {
  #[serde(alias = "ADR Director")]
  AdrDirector,
  #[serde(alias = "Animation Director")]
  AnimationDirector,
  #[serde(alias = "Art Director")]
  ArtDirector,
  #[serde(alias = "Assistant Animation Director")]
  AssistantAnimationDirector,
  #[serde(alias = "Assistant Director")]
  AssistantDirector,
  #[serde(alias = "Background Art")]
  BackgroundArt,
  #[serde(alias = "Character Design")]
  CharacterDesign,
  #[serde(alias = "Chief Animation Director")]
  ChiefAnimationDirector,
  #[serde(alias = "Color Design")]
  ColorDesign,
  #[serde(alias = "Director of Photography")]
  DirectorOfPhotography,
  Director,
  Editing,
  #[serde(alias = "Episode Director")]
  EpisodeDirector,
  #[serde(alias = "Executive Producer")]
  ExecutiveProducer,
  #[serde(alias = "In-Between Animation")]
  InBetweenAnimation,
  #[serde(alias = "Inserted Song Performance")]
  InsertedSongPerformance,
  #[serde(alias = "Key Animation")]
  KeyAnimation,
  Layout,
  Music,
  #[serde(alias = "Original Character Design")]
  OriginalCharacterDesign,
  #[serde(alias = "Original Creator")]
  OriginalCreator,
  Planning,
  Producer,
  Screenplay,
  Script,
  #[serde(alias = "2nd Key Animation")]
  SecondKeyAnimation,
  #[serde(alias = "Series Composition")]
  SeriesComposition,
  Setting,
  #[serde(alias = "Sound Director")]
  SoundDirector,
  #[serde(alias = "Sound Effects")]
  SoundEffects,
  Storyboard,
  #[serde(alias = "Theme Song Arrangement")]
  ThemeSongArrangement,
  #[serde(alias = "Theme Song Composition")]
  ThemeSongComposition,
  #[serde(alias = "Theme Song Lyrics")]
  ThemeSongLyrics,
  #[serde(alias = "Theme Song Performance")]
  ThemeSongPerformance,
  #[serde(other)]
  Unknown,
}

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct Staff {
  #[serde(rename = "mal_id")]
  id: u32,
  image_url: String,
  name: String,
  url: String,
  positions: Vec<StaffPosition>,
}

#[derive(Debug, Deserialize, Getters, PartialEq, Serialize)]
pub struct CharactersStaff {
  characters: Vec<Character>,
  #[serde(flatten)]
  metadata: RequestMetadata,
  staff: Vec<Staff>,
}

impl CharactersStaff {
  pub fn get_url_path(id: u32) -> String {
    format!("/anime/{}/characters_staff", id)
  }

  pub async fn from_id(client: &JikanHttpClient, id: u32) -> Result<CharactersStaff, JikanError> {
    let response = client
      .get::<Self>(&CharactersStaff::get_url_path(id))
      .await?;
    Ok(response.into_body())
  }
}

#[cfg(test)]
mod tests {
  use super::super::test_helper::{self, AnimeTestSuite};
  use super::*;
  use crate::utils::test_helper as utils_test_helper;
  use httpmock::MockServer;
  use std::error::Error;

  #[tokio::test]
  async fn can_get_characters_staff_by_id() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start();
    let client = JikanHttpClient::new(&server.base_url());

    for AnimeTestSuite { id, name } in test_helper::get_valid_animes() {
      let mock = server.mock(|when, then| {
        when.path(CharactersStaff::get_url_path(id));
        then
          .status(200)
          .body(utils_test_helper::file_to_string(&format!(
            "src/anime/__test__/characters_staff_{}.json",
            id
          )));
      });

      let characters_staff = CharactersStaff::from_id(&client, id).await;
      assert!(characters_staff.is_ok(), "{}", name);
      println!("{:?}", &characters_staff);
      mock.assert();
    }

    Ok(())
  }

  #[tokio::test]
  async fn can_handle_characters_staff_404() -> Result<(), Box<dyn Error>> {
    let server = MockServer::start();
    let client = JikanHttpClient::new(&server.base_url());

    for AnimeTestSuite { id, name } in test_helper::get_invalid_animes() {
      let mock = server.mock(|when, then| {
        when.path(CharactersStaff::get_url_path(id));
        then
          .status(404)
          .body(utils_test_helper::file_to_string(&format!(
            "src/anime/__test__/characters_staff_{}.json",
            id
          )));
      });

      let characters_staff = CharactersStaff::from_id(&client, id).await;
      mock.assert();
      assert!(
        characters_staff.is_err(),
        "Response for anime \"{}\" is not 404",
        name
      );
    }

    Ok(())
  }
}
