use anyhow::Result;
use chrono::{DateTime, Utc};
use reqwest::blocking::get;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
const FED_ENTRYPOINT: &str = "https://www.federalreserve.gov";

#[allow(dead_code)]
const FED_SPEECH_URL: &str = "https://www.federalreserve.gov/json/ne-speeches.json";

#[allow(dead_code)]
pub const JEROME_POWELL: &str = "Jerome H. Powell";

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct FedSpeech {
    #[serde(alias = "d")]
    #[serde(with = "eastern_to_utc")]
    pub timestamp: DateTime<Utc>,

    #[serde(alias = "t")]
    pub talk: String,

    #[serde(alias = "s")]
    pub speaker: String,

    #[serde(alias = "lo")]
    pub location: String,

    #[serde(alias = "l")]
    pub link: String,

    #[serde(alias = "a")]
    pub a: String,

    #[serde(alias = "o")]
    pub o: String,

    #[serde(alias = "v")]
    pub video_link: String,

    #[serde(alias = "video")]
    #[serde(deserialize_with = "has_inline_video::deserialize")]
    pub has_inline_video: bool,
}

#[allow(dead_code)]
pub struct FilterOption {
    pub speaker: Option<String>,
}

#[allow(dead_code)]
impl FilterOption {
    fn filter(&self, speech: FedSpeech) -> Option<FedSpeech> {
        match &self.speaker {
            Some(filter_speaker) if speech.speaker.contains(filter_speaker) => Some(speech),
            Some(_) => None,
            None => Some(speech),
        }
    }
}

#[allow(dead_code)]
pub fn fetch_fed_speech(filter_option: Option<FilterOption>) -> Result<Vec<FedSpeech>> {
    let body = get(FED_SPEECH_URL)?.text()?;
    let speeches: Vec<FedSpeech> = serde_json::from_str::<Vec<Fed>>(&body)?
        .into_iter()
        .filter_map(|fed| match fed {
            Fed::FedSpeech(speech) => match &filter_option {
                Some(filter_option) => filter_option.filter(speech),
                None => Some(speech),
            },
            _ => None,
        })
        .map(|mut speech| {
            speech.link = format!("{FED_ENTRYPOINT}{}", speech.link);
            speech
        })
        .collect();

    Ok(speeches)
}

mod eastern_to_utc {
    use chrono::{DateTime, NaiveDateTime, SecondsFormat, Utc};
    use chrono_tz::US::Eastern;
    use serde::{Deserialize, Deserializer, Serializer};

    const ET_FORMAT: &str = "%m/%d/%Y %I:%M:%S %p";

    pub fn serialize<S>(datetime: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&datetime.to_rfc3339_opts(SecondsFormat::Secs, true))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDateTime::parse_from_str(&s, ET_FORMAT);
        let dt = match dt {
            Err(_) => NaiveDateTime::parse_from_str(&format!("{} 12:00:00 AM", &s), ET_FORMAT),
            dt => dt,
        }
        .map_err(serde::de::Error::custom)?
        .and_local_timezone(Eastern)
        .single()
        .ok_or(serde::de::Error::custom(
            "TimeZone transition is not single",
        ))?;
        Ok(dt.with_timezone(&Utc))
    }
}

mod has_inline_video {
    use serde::{Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;

        match s.to_lowercase() {
            s if s == "no" => Ok(false),
            s if s == "yes" => Ok(true),
            _ => Err(serde::de::Error::custom("video field is ambiguous")),
        }
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct UpdateDateField {
    update_date: String,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum Fed {
    FedSpeech(FedSpeech),
    UpdateDateField(UpdateDateField),
}

#[cfg(test)]
mod tests {
    use crate::fetch_utils::fed;
    use anyhow::Result;

    #[test]
    fn fetch_fed_speech() -> Result<()> {
        let speeches = fed::fetch_fed_speech(None)?;
        // println!("{:?} {}", speeches, speeches.len());
        assert!(speeches.len() >= 1034);
        Ok(())
    }

    #[test]
    fn fetch_fed_speech_with_option() -> Result<()> {
        let speeches = fed::fetch_fed_speech(Some(fed::FilterOption {
            speaker: Some(fed::JEROME_POWELL.to_string()),
        }))?;
        // println!("{:?} {}", speeches, speeches.len());
        assert!(speeches.len() >= 105);
        Ok(())
    }
}
