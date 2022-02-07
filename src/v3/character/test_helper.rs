use std::clone::Clone;

#[derive(Clone)]
pub struct CharacterTestSuite {
  pub id: u32,
  pub name: String,
}

pub fn get_valid_characters() -> Vec<CharacterTestSuite> {
  vec![
    CharacterTestSuite {
      id: 1907,
      name: String::from("Taichi Yagami"),
    },
    CharacterTestSuite {
      id: 5620,
      name: String::from("Tsubasa Oozora"),
    },
  ]
}

pub fn get_invalid_characters() -> Vec<CharacterTestSuite> {
  vec![CharacterTestSuite {
    id: 179,
    name: String::from("¯\\_(ツ)_/¯"),
  }]
}
