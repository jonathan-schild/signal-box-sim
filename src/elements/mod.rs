/*
 * Copyright (c) 2025 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Deserializer, Serialize, de::Visitor};

#[derive(Debug)]
pub struct ElementName {
    name: String,
}

impl ElementName {
    fn parse(name: &str) -> ElementName {
        const TRACK_NAME_REGEX: &'static str = r"^\d+$";
        const POINT_NAME_REGEX: &'static str = r"^W \d+$";
        const CROSSING_NAME_REGEX: &'static str = r"^(?:K \d+)|(?:X{0,3})(?:IX|IV|V?I{0,3})$"; // FIXME can be empty
        const DERAILER_NAME_REGEX: &'static str = r"^Gs (?:X{0,3})(?:IX|IV|V?I{0,3})$";

        const MAIN_SIGNAL_NAME_REGEX: &'static str = r"^[A-Z]+ ?\d*$";
        const DISTANT_SIGNAL_NAME_REGEX: &'static str = r"^[a-z]+(?:/[a-z]+)*(?: ?\d+(?:/\d+)*)?$";
        const SHUNTING_SIGNAL_NAME_REGEX: &'static str =
            r"^Ls \d+(?:-(?:X{0,3})(?:IX|IV|V?I{0,3}))?$";

        const LOGICAL_ELEMENT_NAME_REGEX: &'static str = r"^ID [a-zA-Z0-9]+$";

        static TRACK_NAME_MATCHER: Lazy<Regex> =
            Lazy::new(|| Regex::new(TRACK_NAME_REGEX).unwrap());
        static POINT_NAME_MATCHER: Lazy<Regex> =
            Lazy::new(|| Regex::new(POINT_NAME_REGEX).unwrap());
        static CROSSING_NAME_MATCHER: Lazy<Regex> =
            Lazy::new(|| Regex::new(CROSSING_NAME_REGEX).unwrap());
        static DERAILER_NAME_MATCHER: Lazy<Regex> =
            Lazy::new(|| Regex::new(DERAILER_NAME_REGEX).unwrap());

        static MAIN_SIGNAL_MATCHER: Lazy<Regex> =
            Lazy::new(|| Regex::new(MAIN_SIGNAL_NAME_REGEX).unwrap());
        static DISTANT_SIGNAL_NAME_MATCHER: Lazy<Regex> =
            Lazy::new(|| Regex::new(DISTANT_SIGNAL_NAME_REGEX).unwrap());
        static SHUNTING_SIGNAL_NAME_MATCHER: Lazy<Regex> =
            Lazy::new(|| Regex::new(SHUNTING_SIGNAL_NAME_REGEX).unwrap());

        static LOGICAL_ELEMENT_NAME_MATCHER: Lazy<Regex> =
            Lazy::new(|| Regex::new(LOGICAL_ELEMENT_NAME_REGEX).unwrap());

        if !(TRACK_NAME_MATCHER.is_match(name)
            || POINT_NAME_MATCHER.is_match(name)
            || CROSSING_NAME_MATCHER.is_match(name)
            || DERAILER_NAME_MATCHER.is_match(name)
            || MAIN_SIGNAL_MATCHER.is_match(name)
            || DISTANT_SIGNAL_NAME_MATCHER.is_match(name)
            || SHUNTING_SIGNAL_NAME_MATCHER.is_match(name)
            || LOGICAL_ELEMENT_NAME_MATCHER.is_match(name))
        {
            panic!("Invalid name")
        };

        #[cfg(debug_assertions)]
        {
            let mut count = 0u8;
            if TRACK_NAME_MATCHER.is_match(name) {
                count += count;
            }
            if POINT_NAME_MATCHER.is_match(name) {
                count += count;
            }
            if CROSSING_NAME_MATCHER.is_match(name) {
                count += count;
            }
            if DERAILER_NAME_MATCHER.is_match(name) {
                count += count;
            }
            if MAIN_SIGNAL_MATCHER.is_match(name) {
                count += count;
            }
            if DISTANT_SIGNAL_NAME_MATCHER.is_match(name) {
                count += count;
            }
            if SHUNTING_SIGNAL_NAME_MATCHER.is_match(name) {
                count += count;
            }
            if LOGICAL_ELEMENT_NAME_MATCHER.is_match(name) {
                count += count;
            }
            debug_assert!(count < 2)
        }

        Self {
            name: name.to_owned(),
        }
    }
}

impl ToString for ElementName {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

struct CommonNameVisitor;

impl<'de> Visitor<'de> for CommonNameVisitor {
    type Value = ElementName;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a valid element name")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(ElementName::parse(&v))
    }
}

impl<'de> Deserialize<'de> for ElementName {
    fn deserialize<D>(deserializer: D) -> Result<ElementName, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(CommonNameVisitor)
    }
}

impl Serialize for ElementName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.name)
    }
}
