# Jikan

Rust wrapper for [Jikan, unofficial MyAnimeList API](https://jikan.moe).
Currently we are supporting these version:

- [V4 API Docs](https://docs.api.jikan.moe)
- [V3 API Docs](https://jikan.docs.apiary.io) **DEPRECATED**

## Table of Contents

- [TODO](#todo)
- [Installation](#installation)
- [Docs](#docs)
- [Usage V3](#usage-v3)
  - [Anime](#anime)
  - [Manga](#manga)
  - [Person](#person)
- [Contributions](#contributions)
- [Maintainers](#maintainers)
- [License](#license)

## TODO

- Complete `anime` feature
- Complete `manga` feature
- Complete `person` feature
- Complete `character` feature
- Complete `search` feature
- Complete `season`-related feature
- Complete `schedule` feature
- Complete `top` feature
- Complete `genre` feature
- Complete `producer` feature
- Complete `magazine` feature
- Complete `user` feature
- Complete `club` feature
- Complete `meta` feature
- Mock doc test
- Make non-2xx HTTP response status code an error
- Add badges to README
- Add rate limiting rule
- Add caching
- Deprecate V3 when API is no longer maintained

## Installation

Just add `jikan` to to your `Cargo.toml` file.

```toml
[dependencies]
jikan="<version>"
```

## Docs

Coming Soon

## Usage (V3)

### Anime

```rust
use jikan::v3::anime::*;

static PAGE: u32 = 1;

async fn main() -> Result<(), JikanError> {
  let naruto: Anime = Anime::new(20);
  let naruto_info: Info = naruto.info().await?;
  let naruto_characters_staff: CharactersStaff = naruto.characters_staff().await?;
  let naruto_episodes: Episodes = naruto.episodes_at_page(PAGE).await?;
  let naruto_forums: Forums = naruto.forums().await?;
  let naruto_more_info: MoreInfo = naruto.more_info().await?;
  let naruto_news: News = naruto.news().await?;
  let naruto_pictures: Pictures = natuto.pictures().await?;
  let naruto_recommendations: Recommendations = naruto.recommendation().await?;
  let naruto_reviews: Reviews = naruto.reviews_at_page(PAGE).await?;
  let naruto_stats: Stats = naruto.stats().await?;
  let naruto_user_updates: UserUpdates = naruto.user_updates_at_page(PAGE).await?;
  let naruto_videos: Videos = naruto.videos().await?;

  Ok(())
}
```

### Manga

```rust
use jikan::v3::manga::*;

static PAGE: u32 = 1;

async fn main() -> Result<(), JikanError> {
  let one_piece: Manga = Manga::new(13);
  let one_piece_info: Info = one_piece.info().await?;
  let one_piece_characters: Characters = one_piece.characters().await?;
  let one_piece_forums: Forums = one_piece.forums().await?;
  let one_piece_more_info: MoreInfo = one_piece.more_info().await?;
  let one_piece_news: News = one_piece.news().await?;
  let one_piece_pictures: Pictures = natuto.pictures().await?;
  let one_piece_recommendations: Recommendations = one_piece.recommendation().await?;
  let one_piece_reviews: Reviews = one_piece.reviews_at_page(PAGE).await?;
  let one_piece_stats: Stats = one_piece.stats().await?;
  let one_piece_user_updates: UserUpdates = one_piece.user_updates_at_page(PAGE).await?;

  Ok(())
}
```

### Person

```rust
use jikan::v3::person::*;

async fn main() -> Result<(), JikanError> {
  let tetsuya_kakihara: Person = Person::new(167);
  let tetsuya_kakihara_info: Info = tetsuya_kakihara.info().await?;
  let tetsuya_kakihara_pictures: Pictures = natuto.pictures().await?;

  Ok(())
}
```

## Contributions

I am doing this as a side project but any PRs are welcome!
I might miss some things (especially some enums variants that does not show up in the test files).

## Maintainers

[William Darian (@risk1996)](https://github.com/risk1996)

## License

[MIT](https://github.com/risk1996/jikan/blob/master/LICENSE)
