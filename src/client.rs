use super::data::*;
use super::error::Error;
use super::kind::NanoKind;

use std::collections::HashMap;
use std::cell::RefCell;

use reqwest::{Client, Method, StatusCode};
use serde::Serialize;
use serde::de::DeserializeOwned;

#[cfg(test)]
mod tests;

fn add_included(data: &mut Vec<(String, String)>, include: &[NanoKind]) {
    if !include.is_empty() {
        data.push(
            ("include".to_string(), include.iter().map(|kind| kind.api_name()).collect::<Vec<&str>>().join(","))
        )
    }
}

/// A client with which to connect to the Nano site. Can be used with or without login.
#[derive(Debug)]
pub struct NanoClient {
    client: Client,
    username: String,
    password: String,
    token: RefCell<Option<String>>,
}

impl NanoClient {
    const BASE_URL: &'static str = "https://api.nanowrimo.org/";

    fn new(user: &str, pass: &str) -> NanoClient {
        NanoClient {
            client: Client::new(),
            username: user.to_string(),
            password: pass.to_string(),
            token: RefCell::new(None),
        }
    }

    /// Create a new client with the 'anonymous' or 'guest' user, not logged in
    pub fn new_anon() -> NanoClient {
        NanoClient::new("", "")
    }

    /// Create a new client that is automatically logged in as a specific user
    pub async fn new_user(user: &str, pass: &str) -> Result<NanoClient, Error> {
        let client = NanoClient::new(user, pass);
        client.login().await?;
        Ok(client)
    }

    async fn make_request<T, U>(&self, path: &str, method: Method, data: &T) -> Result<U, Error>
        where
            T: Serialize + ?Sized,
            U: DeserializeOwned + std::fmt::Debug
    {
        let mut query = None;
        let mut json = None;

        match method {
            Method::GET => query = Some(data),
            _ => json = Some(data)
        }

        let mut req  = self.client.request(method, &format!("{}{}", NanoClient::BASE_URL, path));

        if let Some(token) = &*self.token.borrow() {
            req = req.header("Authorization", token)
        }

        if let Some(query) = query {
            req = req.query(query);
        }

        if let Some(json) = json {
            req = req.json(json)
        }

        let resp = req.send()
            .await?;

        let status = resp.status();

        match status {
            StatusCode::INTERNAL_SERVER_ERROR => return Err(
                Error::NanoError(status, "Internal Server Error".to_string())
            ),
            StatusCode::NOT_FOUND => return Err(
                Error::NanoError(status, "Page Not Found".to_string())
            ),
            _ => ()
        }

        // if path == "offers" {
        //     println!("Response: {}", resp.json::<serde_json::Value>().await.unwrap());
        //     todo!();
        // }

        let nano_resp = resp
            .json()
            .await?;

        match nano_resp {
            NanoResponse::Success(val) => Ok(val),
            NanoResponse::Error(err) => Err(Error::NanoError(status, err.error)),
            NanoResponse::Unknown(val) => panic!("Couldn't parse valid JSON as NanoResponse:\n{}", val)
        }
    }

    async fn retry_request<T, U>(&self, path: &str, method: Method, data: &T) -> Result<U, Error>
        where
            T: Serialize + ?Sized,
            U: DeserializeOwned + std::fmt::Debug
    {
        let res = self.make_request(path, method.clone(), data).await;

        match res {
            Err(Error::NanoError(code, _)) if code == StatusCode::UNAUTHORIZED && self.is_logged_in() => {
                self.login().await?;
                self.make_request(path, method, data).await
            },
            _ => res
        }
    }

    /// Check whether this client is currently logged in
    pub fn is_logged_in(&self) -> bool {
        self.token.borrow().is_none()
    }

    /// Log in this client, without logging out
    pub async fn login(&self) -> Result<(), Error> {
        let mut map = HashMap::new();
        map.insert("identifier", &self.username);
        map.insert("password", &self.password);

        let res = self.make_request::<_, LoginResponse>("users/sign_in", Method::POST, &map)
            .await?;

        self.token.replace(Some(res.auth_token));

        Ok(())
    }

    /// Log out this client, without checking if it's logged in
    pub async fn logout(&self) -> Result<(), Error> {
        self.make_request::<_, ()>("users/logout", Method::POST, &()).await?;
        self.token.replace(None);

        Ok(())
    }

    /// Change the current user of the client. Logs out if necessary, and either logs in if provided
    /// with username/password, or stays logged out and shifts to the 'guest' user
    pub async fn change_user(&mut self, user: Option<&str>, pass: Option<&str>) -> Result<(), Error> {
        if self.is_logged_in() {
            self.logout().await?;
        }

        if user.is_some() && pass.is_some() {
            self.username = user.unwrap().to_string();
            self.password = pass.unwrap().to_string();
            self.login().await?;
        } else if user.is_none() && pass.is_none() {
            self.username = "".to_string();
            self.password = "".to_string();
            self.token.replace(None);
        } else {
            panic!("Either both user and pass must be provided, or neither")
        }

        Ok(())
    }

    // Commands

    /// Get information about the Nano fundometer
    pub async fn fundometer(&self) -> Result<Fundometer, Error> {
        self.retry_request("fundometer", Method::GET, &()).await
    }

    /// Search for users by username
    pub async fn search(&self, name: &str) -> Result<CollectionResponse<UserObject>, Error> {
        self.retry_request("search", Method::GET, &[("q", name)]).await
    }

    /// Get a random sponsor offer
    pub async fn random_offer(&self) -> Result<ItemResponse<PostObject>, Error> {
        self.retry_request("random_offer", Method::GET, &()).await
    }

    /// Get a list of all store items
    pub async fn store_items(&self) -> Result<Vec<StoreItem>, Error> {
        self.retry_request("store_items", Method::GET, &()).await
    }

    /// Get a list of all current sponsor offers
    pub async fn offers(&self) -> Result<Vec<ItemResponse<PostObject>>, Error> {
        self.retry_request("offers", Method::GET, &()).await
    }

    /// Get the currently logged in user, with included linked items
    pub async fn current_user_include(&self, include: &[NanoKind]) -> Result<ItemResponse<UserObject>, Error> {
        let mut data = Vec::new();

        add_included(&mut data, include);

        self.retry_request("users/current", Method::GET, &data).await
    }

    /// Get the currently logged in user
    pub async fn current_user(&self) -> Result<ItemResponse<UserObject>, Error> {
        self.current_user_include(&[]).await
    }

    /// Get info about a specific set of pages. Known valid values include:
    ///
    /// - `"what-is-camp-nanowrimo"`
    /// - `"nano-prep-101"`
    /// - `"pep-talks"`
    /// - `"dei"`
    /// - `"come-write-in"`
    /// - `"about-nano"`
    /// - `"staff"`
    /// - `"board-of-directors"`
    /// - `"writers-board"`
    /// - `"terms-and-conditions"`
    /// - `"writers-board"`
    /// - `"brought-to-you-by"`
    ///
    /// If you know of other valid values, please open an issue with the values to add to this list!
    pub async fn pages(&self, page: &str) -> Result<ItemResponse<PageObject>, Error> {
        self.retry_request(&format!("pages/{}", page), Method::GET, &()).await
    }

    /// Get the list of notifications for the current user
    pub async fn notifications(&self) -> Result<CollectionResponse<NotificationObject>, Error> {
        self.retry_request("notifications", Method::GET, &()).await
    }

    /// Get a set of all the challenges this user has access to (Possibly all they can make
    /// projects in)
    pub async fn available_challenges(&self) -> Result<CollectionResponse<ChallengeObject>, Error> {
        self.retry_request("challenges/available", Method::GET, &()).await
    }

    // Type queries

    /// Get all accessible items of a specific kind, with included linked items and filtering to
    /// certain related IDs.
    ///
    /// 'includes' will add more items in the response as part of an 'includes' list,
    /// so one request can get more items
    ///
    /// 'filter' will filter certain types of objects by IDs of other objects related to them.
    ///
    /// **Warning**: Many filter combinations are invalid, and the rules are not currently fully
    /// understood.
    pub async fn get_all_include_filtered(&self, ty: NanoKind, include: &[NanoKind], filter: &[(&str, u64)]) -> Result<CollectionResponse, Error> {
        let mut data = Vec::new();

        for i in filter {
            data.push(
                (format!("filter[{}]", i.0), i.1.to_string())
            )
        }

        add_included(&mut data, include);

        self.retry_request(ty.api_name(), Method::GET, &data).await
    }

    /// Get all accessible items of a specific kind, with filtering to certain related IDs
    /// (See [`Self::get_all_include_filtered`])
    pub async fn get_all_filtered(&self, ty: NanoKind, filter: &[(&str, u64)]) -> Result<CollectionResponse, Error> {
        self.get_all_include_filtered(ty, &[], filter).await
    }

    /// Get all accessible items of a specific kind, with included linked items
    /// (See [`Self::get_all_include_filtered`])
    pub async fn get_all_include(&self, ty: NanoKind, include: &[NanoKind]) -> Result<CollectionResponse, Error> {
        self.get_all_include_filtered(ty, include, &[]).await
    }

    /// Get all accessible items of a specific kind, neither filtering nor including linked items
    /// (See [`Self::get_all_include_filtered`])
    pub async fn get_all(&self, ty: NanoKind) -> Result<CollectionResponse, Error> {
        self.get_all_include_filtered(ty, &[], &[]).await
    }

    /// Get an item of a specific type and ID, with included linked items
    pub async fn get_id_include(&self, ty: NanoKind, id: u64, include: &[NanoKind]) -> Result<ItemResponse, Error> {
        let mut data = Vec::new();

        add_included(&mut data, include);

        self.retry_request(&format!("{}/{}", ty.api_name(), id), Method::GET, &data).await
    }

    /// Get an item of a specific type and ID, with no included items.
    /// (See [`Self::get_id_include`])
    pub async fn get_id(&self, ty: NanoKind, id: u64) -> Result<ItemResponse, Error> {
        self.get_id_include(ty, id, &[]).await
    }

    /// Get all items from a given RelationLink, a tie from one object to object(s) of a specific
    /// type that are related to it.
    ///
    /// **Warning**: Not all RelationLinks can be retrieved, some will return a 404 due to the
    /// way Nano handle them on its end, if you know ahead of time that you will need the relations,
    /// it's better to use [`Self::get_id_include`] or [`Self::get_all_include`]
    pub async fn get_all_related(&self, rel: &RelationLink) -> Result<CollectionResponse, Error> {
        if !rel.related.ends_with("s") {
            panic!("get_all_related can only get many-relation links")
        }

        self.retry_request(&rel.related, Method::GET, &()).await
    }

    /// Get a single item from a given RelationLink, a tie from one object to object(s) of a
    /// specific type that are related to it. Single relations tend to not have the same pitfalls as
    /// multiple relations, so this is less dangerous than [`Self::get_all_related`]
    pub async fn get_unique_related(&self, rel: &RelationLink) -> Result<ItemResponse, Error> {
        if rel.related.ends_with("s") {
            panic!("get_unique_related can only get single-relation links")
        }

        self.retry_request(&rel.related, Method::GET, &()).await
    }
}
