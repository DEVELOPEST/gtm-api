use diesel::PgConnection;
use crate::user::model::UserJson;
use crate::user;

pub fn find_all(conn: &PgConnection) -> Vec<UserJson> {
    let user_dwhs = user::db::find_all(conn);
    let mut user_jsons : Vec<UserJson> = Vec::new();
    for user_dwh in user_dwhs {
        let mut role_added_to_user = false;
        for i in 0..user_jsons.len() {
            if user_dwh.id == user_jsons[i].id {
                user_jsons[i].roles.push(user_dwh.role.clone());
                role_added_to_user = true;
                break;
            }
        }
        if !role_added_to_user {
            user_jsons.push(UserJson{
                id: user_dwh.id as i32,
                username: user_dwh.email.to_string(),
                roles: vec![user_dwh.role.clone()]
            })
        }
    }
    user_jsons
}