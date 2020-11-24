use crate::{NanoKind, RelationLink, ObjectRef};
use std::fmt;
use std::collections::HashMap;
use std::str::FromStr;
use serde::{Deserializer, Serializer, Deserialize, Serialize};

pub(crate) fn de_str_num<'de, T, D>(des: D) -> Result<T, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de> + FromStr,
        <T as FromStr>::Err: fmt::Display
{
    String::deserialize(des)?
        .parse::<T>()
        .map_err(serde::de::Error::custom)
}

pub(crate) fn de_rel_includes<'de, D>(des: D) -> Result<HashMap<NanoKind, Vec<ObjectRef>>, D::Error>
    where
        D: Deserializer<'de>
{
    #[derive(Deserialize, Debug)]
    struct DataWrap {
        data: Option<Vec<ObjectRef>>
    }

    HashMap::<String, DataWrap>::deserialize(des)
        .map(
            |table| table.into_iter()
                .filter(
                    |(_, val)| val.data.is_some()
                )
                .map(
                    |(key, val)| (NanoKind::from_name(&key), val.data.unwrap())
                )
                .collect()
        )
        .map_err(serde::de::Error::custom)
}

pub(crate) fn se_rel_includes<S>(val: &HashMap<NanoKind, Vec<ObjectRef>>, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
{
    val.iter()
        .map(|(key, val)| (key.api_name().to_string(), val.clone()))
        .collect::<HashMap<String, Vec<ObjectRef>>>()
        .serialize(ser)
}

pub(crate) fn de_relation<'de, D>(des: D) -> Result<HashMap<NanoKind, RelationLink>, D::Error>
    where
        D: Deserializer<'de>
{
    #[derive(Deserialize, Debug)]
    struct LinkWrap {
        links: RelationLink
    }

    HashMap::<String, LinkWrap>::deserialize(des)
        .map(
            |table| table.into_iter()
                .map(
                    |(key, val)| (NanoKind::from_name(&key), val.links)
                ).collect()
        )
        .map_err(serde::de::Error::custom)
}

pub(crate) fn se_relation<S>(val: &HashMap<NanoKind, RelationLink>, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
{
    val.iter()
        .map(|(key, val)| (key.api_name().to_string(), val.clone()))
        .collect::<HashMap<String, RelationLink>>()
        .serialize(ser)
}

pub(crate) fn de_heighten_img<'de, D>(des: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>
{
    #[derive(Deserialize, Debug)]
    struct ImageWrap {
        src: String
    }

    ImageWrap::deserialize(des)
        .map(|val| val.src)
}
