use super::error::{Error, ErrorInfo};
use serde::{de::DeserializeOwned};
use std::{fmt::Debug, collections::HashMap};

// const API_ROOT: String = String::from("localhost:9876");

/// build all kinds of http request: post/get/delete etc.
pub async fn request<T>(method: reqwest::Method, url: String, body: String, allow: bool) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + Default + Debug,
{
    let allow_body = method == reqwest::Method::POST || method == reqwest::Method::PUT || allow;
    let mut builder = reqwest::Client::new()
        .request(method, url)
        .header("Content-Type", "application/x-www-form-urlencoded; charset=utf-8")
        .header("Authorization", format!("APPCODE {}", body.clone()));
    if allow_body {
        let mut map: HashMap<&str, &str> = HashMap::new();
        map.insert("code", &body);
        builder = builder.form(&map);
    }
    let response = builder.send().await;
    if let Ok(data) = response {
        if data.status().is_success() {
            let data: Result<T, _> = data.json::<T>().await;
            if let Ok(data) = data {
                Ok(data)
            } else {
                Err(Error::DeserializeError)
            }
        } else {
            match data.status().as_u16() {
                401 => Err(Error::Unauthorized),
                403 => Err(Error::Forbidden),
                404 => Err(Error::NotFound),
                500 => Err(Error::InternalServerError),
                422 => {
                    let data: Result<ErrorInfo, _> = data.json::<ErrorInfo>().await;
                    if let Ok(data) = data {
                        Err(Error::UnprocessableEntity(data))
                    } else {
                        Err(Error::DeserializeError)
                    }
                }
                _ => Err(Error::RequestError),
            }
        }
    } else {
        Err(Error::RequestError)
    }
}
