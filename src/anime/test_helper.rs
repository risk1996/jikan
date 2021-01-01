#[cfg(test)]
use rand::Rng;
#[cfg(test)]
use std::clone::Clone;
#[cfg(test)]
use std::time::Duration;

#[cfg(test)]
#[derive(Clone)]
pub struct AnimeTestSuite {
  pub id: u32,
  pub name: String,
}

#[cfg(test)]
pub const REQUEST_DELAY: Duration = Duration::from_secs(4);
#[cfg(test)]
const MAX_ANIME_ID: u32 = 42826;

#[cfg(test)]
pub fn get_valid_animes(count: usize) -> Vec<AnimeTestSuite> {
  let mut result = vec![
    AnimeTestSuite {
      id: 20,
      name: String::from("Naruto"),
    },
    AnimeTestSuite {
      id: 21,
      name: String::from("One Piece"),
    },
    AnimeTestSuite {
      id: 136,
      name: String::from("Hunter x Hunter"),
    },
    AnimeTestSuite {
      id: 5114,
      name: String::from("Fullmetal Alchemist: Brotherhood"),
    },
    AnimeTestSuite {
      id: 34599,
      name: String::from("Made in Abyss"),
    },
  ];

  let mut rng = rand::thread_rng();
  while result.len() < count {
    let random_id = rng.gen_range(1..MAX_ANIME_ID);
    result.push(AnimeTestSuite {
      id: random_id,
      name: String::from(format!("Random ID: {}", random_id)),
    });
  }

  result[..count].to_vec()
}

#[cfg(test)]
pub fn get_invalid_animes() -> Vec<AnimeTestSuite> {
  vec![AnimeTestSuite {
    id: 4207,
    name: String::from("¯\\_(ツ)_/¯"),
  }]
}
