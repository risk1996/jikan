# Jikan

Rust wrapper for [Jikan](https://jikan.moe) (unofficial MyAnimeList REST API).

## Table of Contents

- [TODO](#todo)
- [Installation](#installation)
- [Docs](#docs)
- [Usage](#usage)
  - [Anime](#anime)
- [Maintainers](#maintainers)

## TODO

- Mock doc test
- Make non-2xx HTTP response status code an error
- Add badges to README
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
- Add mandatory throttling checking

## Installation

Just add `jikan` to to your `Cargo.toml` file.

```toml
[dependencies]
jikan="<version>"
```

## Docs

Coming Soon

## Usage

### Anime

```rust
use jikan::anime::*;

static PAGE: u32 = 1;

async fn main() -> Result<(), Box<dyn Err>> {
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
}
```

### Manga

```rust
use jikan::manga::*;

static PAGE: u32 = 1;

async fn main() -> Result<(), Box<dyn Err>> {
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
}
```

## Maintainers

[William Darian (@risk1996)](https://github.com/risk1996)

## License

[MIT](https://github.com/risk1996/jikan/blob/master/LICENSE)
