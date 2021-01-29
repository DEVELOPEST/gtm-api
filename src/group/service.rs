use crate::{group, group_group_member};
use crate::db::Conn;
use crate::errors::Errors;
use crate::group::dwh::GroupRepoStats;

pub fn add_group_relations(conn: &Conn, parents_vec: Vec<String>, children_vec: Vec<String>) {
    for child in children_vec {
        let mut relation_child = group::db::find(&conn, &child);
        if relation_child.is_none() {
            relation_child = Some(group::db::create(&conn, &child));
        }
        let relation_child = relation_child.unwrap();

        for parent in &parents_vec {
            let relation_parent = if !group::db::exists(&conn, &parent) {
                group::db::create(&conn, &parent)
            } else {
                group::db::find(&conn, &parent).unwrap()
            };
            if !group_group_member::db::exists(&conn, &relation_parent.id, &relation_child.id) {
                group_group_member::db::create(&conn, relation_parent.id, relation_child.id);
            }
        }
    }
}

pub fn get_group_repos(conn: &Conn, group_name: &String, start: i64, end: i64) -> Result<Vec<GroupRepoStats>, Errors> {
    let stats = group::db::fetch_group_repositories_stats(conn, group_name, start, end);
    Ok(stats)
}