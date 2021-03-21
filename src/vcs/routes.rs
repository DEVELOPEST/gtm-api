use crate::user::model::AuthUser;
use crate::db::Conn;
use rocket_contrib::json::JsonValue;
use crate::errors::Error;
use crate::vcs::service::fetch_accessible_repositories;

#[get("/vcs/repositories")]
pub fn get_accessible_repositories(
    auth_user: AuthUser,
    conn: Conn
) -> Result<JsonValue, Error> {
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    let repos = rt.block_on(fetch_accessible_repositories(&conn, auth_user.user_id))?;
    Ok(json!(repos))
}
