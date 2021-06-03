#[cfg(test)]
use std::clone::Clone;

#[cfg(test)]
#[derive(Clone)]
pub struct MangaTestSuite {
  pub id: u32,
  pub name: String,
}

#[cfg(test)]
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

#[cfg(test)]
pub fn get_invalid_mangas() -> Vec<MangaTestSuite> {
  vec![MangaTestSuite {
    id: 189,
    name: String::from("¯\\_(ツ)_/¯"),
  }]
}

#[cfg(test)]
pub fn get_pages() -> Vec<u32> {
  vec![1, 999]
}
