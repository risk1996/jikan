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
  let naruto_characters_and_staff: CharactersStaff = naruto.characters_staff().await?;
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

## Maintainers

[William Darian (@risk1996)](https://github.com/risk1996)

## License

[MIT](https://github.com/risk1996/jikan/blob/master/LICENSE)
