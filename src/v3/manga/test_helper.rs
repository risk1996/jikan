use std::clone::Clone;

#[derive(Clone)]
pub struct MangaTestSuite {
  pub id: u32,
  pub name: String,
}

pub fn get_valid_mangas() -> Vec<MangaTestSuite> {
  vec![
    MangaTestSuite {
      id: 13,
      name: String::from("One Piece"),
    },
    MangaTestSuite {
      id: 25,
      name: String::from("Fullmetal Alchemist"),
    },
  ]
}

pub fn get_invalid_mangas() -> Vec<MangaTestSuite> {
  vec![MangaTestSuite {
    id: 189,
    name: String::from("¯\\_(ツ)_/¯"),
  }]
}

pub fn get_pages() -> Vec<u32> {
  vec![1, 999]
}
