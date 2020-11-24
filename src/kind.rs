
#[derive(PartialEq, Eq, Hash, Debug)]
#[derive(Copy, Clone)]
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
    Location,
    NanoMessage,
    Notification,
    Page,
    Post,
    Project,
    StopWatch,
    Timer,
    User,
    // Link items, named like ItemAItemB
    ChildPostPost,
    GroupExternalLink,
    GroupUser,
    LocationGroup,
    PostPage,
    ProjectChallenge,
    ProjectSession,
    UserBadge,
    // It cannot be assumed that we know all possible NanoKinds as it stands
    __NonExhaustive
}

impl NanoKind {
    pub fn from_name(name: &str) -> NanoKind {
        match name {
            "badges" => NanoKind::Badge,
            "challenges" | "challenge" => NanoKind::Challenge,
            "child-posts" => NanoKind::ChildPost,
            "daily-aggregates" => NanoKind::DailyAggregate,
            "external-links" => NanoKind::ExternalLink,
            "favorite-authors" => NanoKind::FavoriteAuthor,
            "favorite-books" => NanoKind::FavoriteBook,
            "genres" => NanoKind::Genre,
            "groups" | "group" => NanoKind::Group,
            "locations" => NanoKind::Location,
            "nanomessages" => NanoKind::NanoMessage,
            "notifications" => NanoKind::Notification,
            "pages" => NanoKind::Page,
            "posts" => NanoKind::Post,
            "projects" | "project" => NanoKind::Project,
            "stopwatches" => NanoKind::StopWatch,
            "timers" => NanoKind::Timer,
            "users" | "user" => NanoKind::User,

            "child-post-posts" => NanoKind::ChildPostPost,
            "group-external-links" => NanoKind::GroupExternalLink,
            "group-users" => NanoKind::GroupUser,
            "location-groups" => NanoKind::LocationGroup,
            "post-pages" => NanoKind::PostPage,
            "project-challenges" => NanoKind::ProjectChallenge,
            "project-sessions" => NanoKind::ProjectSession,
            "user-badges" => NanoKind::UserBadge,
            kind => panic!("Unknown/unimplemented NanoKind: {}", kind)
        }
    }

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
