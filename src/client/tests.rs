use super::*;

async fn test_client() -> NanoClient {
    NanoClient::new_user(env!("NANO_USERNAME"), env!("NANO_PASSWORD")).await
        .expect("Couldn't create and log-in a new NanoClient")
}

#[tokio::test]
async fn connect() {
    test_client().await;
}

#[tokio::test]
async fn test_current_user() {
    let client = test_client().await;

    client.current_user()
        .await
        .expect("Couldn't get current user");
}

#[tokio::test]
async fn test_fundometer() {
    let client = test_client().await;

    client.fundometer().await.expect("Couldn't get Nano Fundometer");
}

#[tokio::test]
async fn test_notifications() {
    let client = test_client().await;

    client.notifications()
        .await
        .expect("Couldn't get user notifications");
}

#[tokio::test]
async fn test_pages() {
    let client = test_client().await;

    for &i in &[
        "what-is-camp-nanowrimo", "nano-prep-101", "pep-talks", "dei", "come-write-in",
        "about-nano", "staff", "board-of-directors", "writers-board", "terms-and-conditions",
        "writers-board", "brought-to-you-by"
    ] {
        client.pages(i)
            .await
            .expect("Couldn't get page that was expected to exist");
    }
}

#[tokio::test]
async fn test_daily_aggregates() {
    let client = test_client().await;

    client.daily_aggregates(2617284)
        .await
        .expect("Couldn't get daily aggregates");
}

#[tokio::test]
async fn test_get_all_filtered() {
    let client = test_client().await;
    let user_id = client.current_user().await.unwrap().data.id();

    let projects = client.get_all_filtered(NanoKind::Project, &[("user_id", user_id)])
        .await
        .expect("Couldn't get all filtered projects of the current user");

    for i in projects.data {
        assert_eq!(i.kind(), NanoKind::Project, "get_all_filtered with Project kind didn't return all projects");
    }
}

#[tokio::test]
async fn test_get_id() {
    let client = test_client().await;

    let badge = client.get_id(NanoKind::Badge, 1)
        .await
        .expect("Couldn't get by ID an example Badge");

    assert_eq!(badge.data.kind(), NanoKind::Badge, "get_id with Badge kind didn't return a badge")
}

/*
TODO: Make tests for more stuff. Some examples used while writing all this:

client.search("craftspider").await.unwrap();

client.random_offer().await.unwrap();

client.store_items().await.unwrap();

client.offers().await.unwrap();

for (_, link) in &user.data.relationships.relations {
    dbg!(link);
    let _ = dbg!(client.get_all_related(link).await);
    tokio::time::delay_for(std::time::Duration::from_secs(1)).await;
}

dbg!(client.primary_project(user.data.id).await.unwrap());

let _ = dbg!(client.pages("pep-talks").await);
let _ = dbg!(client.pages("nano-prep-101").await);
*/
