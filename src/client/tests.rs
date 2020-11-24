use super::*;

async fn test_client() -> NanoClient {
    NanoClient::new_user(env!("NANO_USERNAME"), env!("NANO_PASSWORD")).await
        .unwrap()
}

#[tokio::test]
async fn connect() {
    test_client().await;
}

#[tokio::test]
async fn test_current_user() {
    let client = test_client().await;

    let user = client.current_user().await.unwrap();

    assert_eq!(user.data.kind(), NanoKind::User, "current_user didn't return a User type");
}

#[tokio::test]
async fn test_notifications() {
    let client = test_client().await;

    let notifs = client.notifications().await.unwrap();

    for i in notifs.data {
        assert_eq!(i.kind(), NanoKind::Notification, "notifications didn't return only all notifications");
    }
}

#[tokio::test]
async fn test_get_all_filtered() {
    let client = test_client().await;
    let user_id = client.current_user().await.unwrap().data.id;

    let projects = client.get_all_filtered(NanoKind::Project, &[("user_id", user_id)])
        .await
        .unwrap();

    for i in projects.data {
        assert_eq!(i.kind(), NanoKind::Project, "get_all_filtered with Project kind didn't return all projects");
    }
}

#[tokio::test]
async fn test_get_id() {
    let client = test_client().await;

    let badge = client.get_id(NanoKind::Badge, 1).await.unwrap();

    assert_eq!(badge.data.kind(), NanoKind::Badge, "get_id with Badge kind didn't return a badge")
}

/*
TODO: Make tests for more stuff. Some examples used while writing all this:

client.fundometer().await.unwrap();

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
