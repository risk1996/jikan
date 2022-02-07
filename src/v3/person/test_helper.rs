use std::clone::Clone;

#[derive(Clone)]
pub struct PersonTestSuite {
  pub id: u32,
  pub name: String,
}

pub fn get_valid_persons() -> Vec<PersonTestSuite> {
  vec![
    PersonTestSuite {
      id: 167,
      name: String::from("Tetsuya Kakihara"),
    },
    PersonTestSuite {
      id: 1880,
      name: String::from("Tite Kubo"),
    },
  ]
}

pub fn get_invalid_persons() -> Vec<PersonTestSuite> {
  vec![PersonTestSuite {
    id: 33,
    name: String::from("¯\\_(ツ)_/¯"),
  }]
}
