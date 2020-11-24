
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
    pub fn from_name(name: &str) -> NanoKind {
        match name {
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

            "child-post-posts" => NanoKind::ChildPostPost,
            "group-users" => NanoKind::GroupUser,
            "location-groups" => NanoKind::LocationGroup,
            "post-pages" => NanoKind::PostPage,
            "project-challenges" | "project-challenge" => NanoKind::ProjectChallenge,
            "user-badges" => NanoKind::UserBadge,
            kind => panic!("Unknown/unimplemented NanoKind: {}", kind)
        }
    }

    /// Convert a NanoKind into the equivalent name of the type in the Nano API
    pub fn api_name(&self) -> &str {
        match self {
            NanoKind::User => "users",
            NanoKind::Project => "projects",
            NanoKind::Badge => "badges",
            NanoKind::Genre => "genres",
            NanoKind::Group => "groups",
            NanoKind::Challenge => "challenges",
            NanoKind::NanoMessage => "nanomessages",
            NanoKind::Notification => "notifications",
            NanoKind::Location => "locations",
            NanoKind::ExternalLink => "external-links",
            NanoKind::GroupExternalLink => "group-external-links",
            NanoKind::FavoriteBook => "favorite-books",
            NanoKind::FavoriteAuthor => "favorite-authors",
            NanoKind::DailyAggregate => "daily-aggregates",
            NanoKind::UserBadge => "user-badges",
            NanoKind::ProjectSession => "project-sessions",
            NanoKind::ProjectChallenge => "project-challenges",
            NanoKind::GroupUser => "group-users",
            NanoKind::LocationGroup => "location-groups",
            NanoKind::Page => "pages",
            NanoKind::Post => "posts",
            NanoKind::Timer => "timers",
            NanoKind::StopWatch => "stopwatches",
            NanoKind::PostPage => "post-pages",
            NanoKind::ChildPost => "child-posts",
            NanoKind::ChildPostPost => "child-post-posts",
            kind => panic!("Unknown/unimplemented NanoKind: {:?}", kind)
        }
    }
}
