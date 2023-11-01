use anyhow::Result;
use moguls::prelude::*;

fn main() -> Result<()> {
    let speeches = fetch_fed_speech(Some(FilterOption {
        speaker: Some(JEROME_POWELL.to_string()),
    }))?;
    println!("{:?}", speeches);
    Ok(())
}
