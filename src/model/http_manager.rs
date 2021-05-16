/*
    Copyright 2021 Volt Contributors

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
*/

// Std Imports
use std::io;

// Library Imports
use thiserror::Error;

// Crate Level Imports
use crate::classes::package::Package;

#[derive(Error, Debug)]
pub enum GetPackageError {
    #[error("network request failed with registry")]
    Request(chttp::Error),
    #[error("unable to read network response")]
    IO(io::Error),
    #[error("unable to deserialize network response: {0:?}")]
    JSON(serde_json::Error),
}

#[allow(dead_code)]
/// Request a package from `registry.yarnpkg.com`
///
/// Uses `chttp` async implementation to send a `get` request for the package
/// ## Arguments
/// * `name` - Name of the package to request from `registry.yarnpkg.com`
/// ## Examples
/// ```
/// // Await an async response
/// get_package("react").await;
/// ```
/// ## Returns
/// * `Result<Option<Package>, GetPackageError>`
pub async fn get_package(name: &str) -> Result<Option<Package>, GetPackageError> {
    let resp = chttp::get_async(format!("http://registry.yarnpkg.com/{}", name))
        .await
        .map_err(GetPackageError::Request)?;

    if resp.status().is_client_error() {
        return Ok(None);
    }

    let mut body = resp.into_body();
    let body_string = body.text().map_err(GetPackageError::IO)?;

    let package: Package = serde_json::from_str(&body_string).map_err(GetPackageError::JSON)?;

    Ok(Some(package))
}

/// Get all dependencies of a package
pub async fn get_dependencies(package_name: &str) -> String {
    // Temporary CDN
    let resp = chttp::get_async(format!("http://registry.voltpkg.com/{}.json", package_name))
        .await
        .map_err(GetPackageError::Request)
        .unwrap();

    let mut body = resp.into_body();
    body.text().unwrap()
}
