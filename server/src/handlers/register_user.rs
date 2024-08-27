// The MIT License (MIT)
//
// Copyright (c) 2024 Aliaksei Bialiauski
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
use axum::http::StatusCode;
use axum::Json;

use crate::objects::user::User;
use crate::xml::storage::Storage;

/// Register user.
///
/// # Fields
///
/// * `payload`: JSON payload
///
/// # Examples
///
/// ```
/// use axum::Json;
/// use server::handlers::register_user::register_user;
/// use server::objects::user::User;
/// use server::xml::storage::Storage;
/// let registration = register_user(Json(User::new(String::from("jeff"), Storage::new(None))));
/// ```
pub async fn register_user(Json(payload): Json<User>) -> Result<StatusCode, String> {
    let user = User::new(payload.username.clone(), Storage::new(None));
    match user.save().await {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(e) => Err(format!("Can't register {}: {}", payload.username, e)),
    }
}
