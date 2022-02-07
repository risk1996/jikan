use serde::{Deserialize, Serialize};
use std::cmp::PartialEq;

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
