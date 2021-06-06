use std::clone::Clone;

#[derive(Clone)]
pub struct AnimeTestSuite {
  pub id: u32,
  pub name: String,
}

pub fn get_valid_animes() -> Vec<AnimeTestSuite> {
  vec![
    AnimeTestSuite {
      id: 20,
      name: String::from("NARUTO"),
    },
    AnimeTestSuite {
      id: 136,
      name: String::from("HUNTER×HUNTER"),
    },
  ]
}

pub fn get_invalid_animes() -> Vec<AnimeTestSuite> {
  vec![AnimeTestSuite {
    id: 4207,
    name: String::from("¯\\_(ツ)_/¯"),
  }]
}

pub fn get_pages() -> Vec<u32> {
  vec![1, 999]
}
