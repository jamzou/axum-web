use chrono::NaiveDateTime;
use serde::{Deserialize, Deserializer, Serializer};
const DATE_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub fn serialize_naive<S>(date: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match date {
        Some(dt) => serializer.serialize_str(&dt.format(DATE_FORMAT).to_string()),
        None => serializer.serialize_none(),
    }
}

pub fn deserialize_naive<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(str) => {
            let naive_dt = NaiveDateTime::parse_from_str(&str, DATE_FORMAT)
                .map_err(serde::de::Error::custom)?;
            Ok(Some(naive_dt))
        }
        None => Ok(None),
    }
}
