# 👑moguls

Welcome to the Halls of Financial Wisdom, a sanctuary where the echoes of economic insight and financial prowess resonate. `Moguls` is a crate where the speeches of financial gurus with a treasure trove of knowledge come to life. Within our crate, you will find a curated collection of eloquent addresses, insightful talks, and visionary narratives delivered by the titans of the financial world.

_Let the words of financial moguls inspire and guide you in your quest for financial excellence and understanding._

## Usage

```rust
use anyhow::Result;
use moguls::prelude::*;

fn main() -> Result<()> {
    let speeches = fetch_fed_speech(Some(FilterOption {
        speaker: Some(JEROME_POWELL.to_string()),
    }))?;
    println!("{:?}", speeches);
    Ok(())
}
```

## Moguls List

- [x] Jerome Powell, the Chair of the Federal Reserve
- [ ] Warren Buffett, the Chairman and CEO of Berkshire Hathaway

## Sources

- [Speeches of Federal Reserve Officials](https://www.federalreserve.gov/newsevents/speeches.htm)