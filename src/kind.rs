
/// A representation of the known kinds of NanoWrimo Objects. This enum is marked non_exhaustive
/// because it tracks the private API, and thus it is unsure if this lists every possible type,
/// and new ones may be added or removed at any time by Nano.
#[derive(PartialEq, Eq, Hash, Debug, Copy, Clone)]
#[non_exhaustive]
pub enum NanoKind {
    Badge,
    Challenge,
    ChildPost,
    DailyAggregate,
    ExternalLink,
    FavoriteAuthor,
    FavoriteBook,
    Genre,
    Group,
    GroupExternalLink,
    Location,
    NanoMessage,
    Notification,
    Page,
    Post,
    Project,
    ProjectSession,
    StopWatch,
    Timer,
    User,
    WritingLocation,
    WritingMethod,
    // Link items, named like ItemAItemB
    ChildPostPost,
    GroupUser,
    LocationGroup,
    PostPage,
    ProjectChallenge,
    UserBadge,
    // It cannot be assumed that we know all possible NanoKinds as it stands
    __NonExhaustive
}

impl NanoKind {
    /// Convert the name of a type from the Nano API into a NanoKind
    pub fn from_name(name: &str) -> Result<NanoKind, String> {
        Ok(match name {
            "badges" | "badge" => NanoKind::Badge,
            "challenges" | "challenge" => NanoKind::Challenge,
            "child-posts" => NanoKind::ChildPost,
            "daily-aggregates" => NanoKind::DailyAggregate,
            "external-links" => NanoKind::ExternalLink,
            "favorite-authors" => NanoKind::FavoriteAuthor,
            "favorite-books" => NanoKind::FavoriteBook,
            "genres" => NanoKind::Genre,
            "groups" | "group" => NanoKind::Group,
            "group-external-links" => NanoKind::GroupExternalLink,
            "locations" | "location" => NanoKind::Location,
            "nanomessages" => NanoKind::NanoMessage,
            "notifications" => NanoKind::Notification,
            "pages" => NanoKind::Page,
            "posts" => NanoKind::Post,
            "projects" | "project" => NanoKind::Project,
            "project-sessions" => NanoKind::ProjectSession,
            "stopwatches" => NanoKind::StopWatch,
            "timers" => NanoKind::Timer,
            "users" | "user" => NanoKind::User,
            "writing-locations" => NanoKind::WritingLocation,
            "writing-methods" => NanoKind::WritingMethod,

            "child-post-posts" => NanoKind::ChildPostPost,
            "group-users" => NanoKind::GroupUser,
            "location-groups" => NanoKind::LocationGroup,
            "post-pages" => NanoKind::PostPage,
            "project-challenges" | "project-challenge" => NanoKind::ProjectChallenge,
            "user-badges" => NanoKind::UserBadge,
            kind => return Err(format!("Unknown/unimplemented NanoKind: {}", kind))
        })
    }

    /// Convert a NanoKind into the equivalent name of the type in the Nano API. These are always
    /// plural, use [`Self::api_unique_name`] for the singular variant.
    pub fn api_name(&self) -> &str {
        match self {
            NanoKind::Badge => "badges",
            NanoKind::Challenge => "challenges",
            NanoKind::ChildPost => "child-posts",
            NanoKind::DailyAggregate => "daily-aggregates",
            NanoKind::ExternalLink => "external-links",
            NanoKind::FavoriteAuthor => "favorite-authors",
            NanoKind::FavoriteBook => "favorite-books",
            NanoKind::Genre => "genres",
            NanoKind::Group => "groups",
            NanoKind::GroupExternalLink => "group-external-links",
            NanoKind::Location => "locations",
            NanoKind::NanoMessage => "nanomessages",
            NanoKind::Notification => "notifications",
            NanoKind::Page => "pages",
            NanoKind::Post => "posts",
            NanoKind::Project => "projects",
            NanoKind::ProjectSession => "project-sessions",
            NanoKind::StopWatch => "stopwatches",
            NanoKind::Timer => "timers",
            NanoKind::User => "users",
            NanoKind::WritingLocation => "writing-locations",
            NanoKind::WritingMethod => "writing-methods",

            NanoKind::ChildPostPost => "child-post-posts",
            NanoKind::GroupUser => "group-users",
            NanoKind::LocationGroup => "location-groups",
            NanoKind::PostPage => "post-pages",
            NanoKind::ProjectChallenge => "project-challenges",
            NanoKind::UserBadge => "user-badges",
            kind => panic!("Unknown/unimplemented NanoKind: {:?}", kind)
        }
    }

    /// Convert a NanoKind into the 'unique' name used for things like relations tied to a unique
    /// value (Strips the plural).
    pub fn api_unique_name(&self) -> &str {
        match self {
            NanoKind::Badge => "badge",
            NanoKind::Challenge => "challenge",
            NanoKind::ChildPost => "child-post",
            NanoKind::DailyAggregate => "daily-aggregate",
            NanoKind::ExternalLink => "external-link",
            NanoKind::FavoriteAuthor => "favorite-author",
            NanoKind::FavoriteBook => "favorite-book",
            NanoKind::Genre => "genre",
            NanoKind::Group => "group",
            NanoKind::GroupExternalLink => "group-external-link",
            NanoKind::Location => "location",
            NanoKind::NanoMessage => "nanomessage",
            NanoKind::Notification => "notification",
            NanoKind::Page => "page",
            NanoKind::Post => "post",
            NanoKind::Project => "project",
            NanoKind::ProjectSession => "project-session",
            NanoKind::StopWatch => "stopwatch",
            NanoKind::Timer => "timer",
            NanoKind::User => "user",
            NanoKind::WritingLocation => "writing-location",
            NanoKind::WritingMethod => "writing-method",

            NanoKind::ChildPostPost => "child-post-post",
            NanoKind::GroupUser => "group-user",
            NanoKind::LocationGroup => "location-group",
            NanoKind::PostPage => "post-page",
            NanoKind::ProjectChallenge => "project-challenge",
            NanoKind::UserBadge => "user-badge",
            kind => panic!("Unknown/unimplemented NanoKind: {:?}", kind)
        }
    }
}
