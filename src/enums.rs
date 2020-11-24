use std::convert::TryFrom;

use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
#[serde(try_from = "u8", into = "u8")]
pub enum PrivacySetting {
    Private,
    Buddies,
    Anyone,
}

impl TryFrom<u8> for PrivacySetting {
    type Error = &'static str;

    fn try_from(val: u8) -> Result<PrivacySetting, Self::Error> {
        match val {
            0 => Ok(PrivacySetting::Private),
            1 => Ok(PrivacySetting::Buddies),
            2 => Ok(PrivacySetting::Anyone),
            _ => Err("Cannot convert u8 into PrivacySetting")
        }
    }
}

impl Into<u8> for PrivacySetting {
    fn into(self) -> u8 {
        match self {
            PrivacySetting::Private => 0,
            PrivacySetting::Buddies => 1,
            PrivacySetting::Anyone => 2,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
#[serde(try_from = "&str", into = "&'static str")]
pub enum ProjectStatus {
    InProgress,
    Drafted,
    Completed,
    Published,
}

impl TryFrom<&str> for ProjectStatus {
    type Error = &'static str;

    fn try_from(val: &str) -> Result<ProjectStatus, Self::Error> {
        match val {
            "In Progress" => Ok(ProjectStatus::InProgress),
            "Drafted" => Ok(ProjectStatus::Drafted),
            "Completed" => Ok(ProjectStatus::Completed),
            "Published" => Ok(ProjectStatus::Published),
            _ => Err("Cannot convert &str into ProjectStatus")
        }
    }
}

impl Into<&'static str> for ProjectStatus {
    fn into(self) -> &'static str {
        match self {
            ProjectStatus::InProgress => "In Progress",
            ProjectStatus::Drafted => "Drafted",
            ProjectStatus::Completed => "Completed",
            ProjectStatus::Published => "Published",
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
#[serde(try_from = "u8", into = "u8")]
pub enum EventType {
    NanoWrimo,
    CampNano
}

impl TryFrom<u8> for EventType {
    type Error = &'static str;

    fn try_from(val: u8) -> Result<EventType, Self::Error> {
        match val {
            0 => Ok(EventType::NanoWrimo),
            1 => Ok(EventType::CampNano),
            _ => Err("Cannot convert u8 into EventType")
        }
    }
}

impl Into<u8> for EventType {
    fn into(self) -> u8 {
        match self {
            EventType::NanoWrimo => 0,
            EventType::CampNano => 1,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
#[serde(try_from = "&str", into = "&'static str")]
pub enum GroupType {
    Everyone,
    Region,
    Buddies,
}

impl TryFrom<&str> for GroupType {
    type Error = &'static str;

    fn try_from(val: &str) -> Result<GroupType, Self::Error> {
        match val {
            "everyone" => Ok(GroupType::Everyone),
            "region" => Ok(GroupType::Region),
            "buddies" => Ok(GroupType::Buddies),
            _ => Err("Cannot convert &str into GroupType")
        }
    }
}

impl Into<&'static str> for GroupType {
    fn into(self) -> &'static str {
        match self {
            GroupType::Everyone => "everyone",
            GroupType::Region => "region",
            GroupType::Buddies => "buddies",
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
#[serde(try_from = "&str", into = "&'static str")]
pub enum EntryMethod {
    Join,
    Creator,
    Invited
}

impl TryFrom<&str> for EntryMethod {
    type Error = &'static str;

    fn try_from(val: &str) -> Result<EntryMethod, Self::Error> {
        match val {
            "join" => Ok(EntryMethod::Join),
            "creator" => Ok(EntryMethod::Creator),
            "invited" => Ok(EntryMethod::Invited),
            _ => Err("Cannot convert &str into EntryMethod")
        }
    }
}

impl Into<&'static str> for EntryMethod {
    fn into(self) -> &'static str {
        match self {
            EntryMethod::Join => "join",
            EntryMethod::Creator => "creator",
            EntryMethod::Invited => "invited",
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Copy, Clone)]
#[serde(try_from = "u8", into = "u8")]
pub enum AdminLevel {
    User,
    Admin
}

impl TryFrom<u8> for AdminLevel {
    type Error = &'static str;

    fn try_from(val: u8) ->  Result<AdminLevel, Self::Error> {
        match val {
            0 => Ok(AdminLevel::User),
            1 => Ok(AdminLevel::Admin),
            _ => Err("Cannot convert u8 into AdminLevel")
        }
    }
}

impl Into<u8> for AdminLevel {
    fn into(self) -> u8 {
        match self {
            AdminLevel::User => 0,
            AdminLevel::Admin => 1,
        }
    }
}
