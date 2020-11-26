use std::convert::TryFrom;

use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Copy, Clone)]
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

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Copy, Clone)]
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

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Copy, Clone)]
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

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Copy, Clone)]
#[serde(try_from = "&str", into = "&'static str")]
pub enum GroupType {
    Everyone,
    Region,
    Buddies,
    WritingGroup,
    Event,
}

impl TryFrom<&str> for GroupType {
    type Error = &'static str;

    fn try_from(val: &str) -> Result<GroupType, Self::Error> {
        match val {
            "everyone" => Ok(GroupType::Everyone),
            "region" => Ok(GroupType::Region),
            "buddies" => Ok(GroupType::Buddies),
            "writing group" => Ok(GroupType::WritingGroup),
            "event" => Ok(GroupType::Event),
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
            GroupType::WritingGroup => "writing group",
            GroupType::Event => "event",
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Copy, Clone)]
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

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Copy, Clone)]
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

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Copy, Clone)]
#[serde(try_from = "&str", into = "&'static str")]
pub enum ActionType {
    BadgeAwarded,
    BuddiesPage,
    NanoMessages,
    ProjectsPage,
}

impl TryFrom<&str> for ActionType {
    type Error = &'static str;

    fn try_from(val: &str) -> Result<ActionType, Self::Error> {
        match val {
            "BADGE_AWARDED" => Ok(ActionType::BadgeAwarded),
            "BUDDIES_PAGE" => Ok(ActionType::BuddiesPage),
            "NANOMESSAGES" => Ok(ActionType::NanoMessages),
            "PROJECTS_PAGE" => Ok(ActionType::ProjectsPage),
            _ => Err("Cannot convert &str into ActionType")
        }
    }
}

impl Into<&'static str> for ActionType {
    fn into(self) -> &'static str {
        match self {
            ActionType::BadgeAwarded => "BADGE_AWARDED",
            ActionType::BuddiesPage => "BUDDIES_PAGE",
            ActionType::NanoMessages => "NANOMESSAGES",
            ActionType::ProjectsPage => "PROJECTS_PAGE"
        }
    }
}

/// Whether to display the notification in the 'recent notifications'
#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Copy, Clone)]
#[serde(try_from = "u8", into = "u8")]
pub enum DisplayStatus {
    AllNotifs,
    RecentNotifs,
}

impl TryFrom<u8> for DisplayStatus {
    type Error = &'static str;

    fn try_from(val: u8) ->  Result<DisplayStatus, Self::Error> {
        match val {
            0 => Ok(DisplayStatus::AllNotifs),
            1 => Ok(DisplayStatus::RecentNotifs),
            _ => Err("Cannot convert u8 into DisplayStatus")
        }
    }
}

impl Into<u8> for DisplayStatus {
    fn into(self) -> u8 {
        match self {
            DisplayStatus::AllNotifs => 0,
            DisplayStatus::RecentNotifs => 1,
        }
    }
}

// TODO: This may be wrong
#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Copy, Clone)]
#[serde(try_from = "u8", into = "u8")]
pub enum WritingType {
    Novel,
    ShortStories,
    Memoir,
    Script,
    Nonfiction,
    Poetry,
    Other
}

impl TryFrom<u8> for WritingType {
    type Error = &'static str;

    fn try_from(val: u8) ->  Result<WritingType, Self::Error> {
        match val {
            0 => Ok(WritingType::Novel),
            1 => Ok(WritingType::ShortStories),
            2 => Ok(WritingType::Memoir),
            3 => Ok(WritingType::Script),
            4 => Ok(WritingType::Nonfiction),
            5 => Ok(WritingType::Poetry),
            6 => Ok(WritingType::Other),
            _ => Err("Cannot convert u8 into WritingType")
        }
    }
}

impl Into<u8> for WritingType {
    fn into(self) -> u8 {
        match self {
            WritingType::Novel => 0,
            WritingType::ShortStories => 1,
            WritingType::Memoir => 2,
            WritingType::Script => 3,
            WritingType::Nonfiction => 4,
            WritingType::Poetry => 5,
            WritingType::Other => 8
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Copy, Clone)]
#[serde(try_from = "&str", into = "&'static str")]
pub enum ContentType {
    GeneralContent,
    StackedContent,
    Plate,
    GroupOfPeople,
    GroupOfPageCards,
    PersonCard,
    PepTalk,
    PlainText,
}

impl TryFrom<&str> for ContentType {
    type Error = &'static str;

    fn try_from(val: &str) -> Result<ContentType, Self::Error> {
        match val {
            "General content" => Ok(ContentType::GeneralContent),
            "Stacked Content" => Ok(ContentType::StackedContent),
            "Plate" => Ok(ContentType::Plate),
            "Group of people" => Ok(ContentType::GroupOfPeople),
            "Group of page cards" => Ok(ContentType::GroupOfPageCards),
            "Person Card" => Ok(ContentType::PersonCard),
            "Pep Talk" => Ok(ContentType::PepTalk),
            "Plain Text" => Ok(ContentType::PlainText),
            _ => Err("Cannot convert &str into ContentType")
        }
    }
}

impl Into<&'static str> for ContentType {
    fn into(self) -> &'static str {
        match self {
            ContentType::GeneralContent => "General content",
            ContentType::StackedContent => "Stacked Content",
            ContentType::Plate => "Plate",
            ContentType::GroupOfPeople => "Group of people",
            ContentType::GroupOfPageCards => "Group of page cards",
            ContentType::PersonCard => "Person Card",
            ContentType::PepTalk => "Pep Talk",
            ContentType::PlainText => "Plain Text",
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Copy, Clone)]
#[serde(try_from = "&str", into = "&'static str")]
pub enum RegistrationPath {
    Email,
    Facebook,
    Google,
}

impl TryFrom<&str> for RegistrationPath {
    type Error = &'static str;

    fn try_from(val: &str) -> Result<RegistrationPath, Self::Error> {
        match val {
            "email" => Ok(RegistrationPath::Email),
            "Facebook" => Ok(RegistrationPath::Facebook),
            "Google" => Ok(RegistrationPath::Google),
            _ => Err("Cannot convert &str into RegistrationPath")
        }
    }
}

impl Into<&'static str> for RegistrationPath {
    fn into(self) -> &'static str {
        match self {
            RegistrationPath::Email => "email",
            RegistrationPath::Facebook => "Facebook",
            RegistrationPath::Google => "Google",
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Copy, Clone)]
#[serde(try_from = "&str", into = "&'static str")]
pub enum BadgeType {
    WordCount,
    SelfAwarded
}

impl TryFrom<&str> for BadgeType {
    type Error = &'static str;

    fn try_from(val: &str) -> Result<BadgeType, Self::Error> {
        match val {
            "word count" => Ok(BadgeType::WordCount),
            "self-awarded" => Ok(BadgeType::SelfAwarded),
            _ => Err("Cannot convert &str into BadgeType")
        }
    }
}

impl Into<&'static str> for BadgeType {
    fn into(self) -> &'static str {
        match self {
            BadgeType::WordCount => "word count",
            BadgeType::SelfAwarded => "self-awarded",
        }
    }
}

#[derive(Deserialize, Serialize, PartialEq, Eq, Debug, Copy, Clone)]
#[serde(try_from = "u8", into = "u8")]
pub enum JoiningRule {
    AdminOnly,
    AnyUser,
}

impl TryFrom<u8> for JoiningRule {
    type Error = &'static str;

    fn try_from(val: u8) -> Result<JoiningRule, Self::Error> {
        match val {
            0 => Ok(JoiningRule::AdminOnly),
            1 => Ok(JoiningRule::AnyUser),
            _ => Err("Cannot convert &str into JoiningRule")
        }
    }
}

impl Into<u8> for JoiningRule {
    fn into(self) -> u8 {
        match self {
            JoiningRule::AdminOnly => 0,
            JoiningRule::AnyUser => 1,
        }
    }
}
