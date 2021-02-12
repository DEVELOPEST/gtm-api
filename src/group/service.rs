use crate::{group, group_group_member, group_access};
use crate::db::Conn;
use crate::errors::Errors;
use crate::group::dwh::{GroupStats};
use crate::group::model::Group;
use crate::group_access::model::GroupAccess;
use crate::group_group_member::model::GroupRelation;

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

pub fn get_group_stats(
    conn: &Conn,
    group_name: &String,
    start: i64,
    end: i64,
) -> Result<GroupStats, Errors> {
    let user_stats = group::db::fetch_group_user_stats(conn, group_name, start, end);
    let file_stats = group::db::fetch_group_file_stats(conn, group_name, start, end);
    Ok(GroupStats {
        users: user_stats,
        files: file_stats.into_iter()
            .filter(|f| !f.path.starts_with(".gtm"))
            .collect(),
    })
}

pub fn get_groups_without_access(conn: &Conn, user_id: i32) -> Vec<Group> {
    let groups: Vec<Group> = group::db::find_all(conn);
    let groups_with_access: Vec<Group> = get_groups_with_access(conn, user_id);
    let mut res: Vec<Group> = Vec::new();
    for group in &groups {
        if !groups_with_access.contains(group) {
            res.push(group.clone());
        }
    }
    res
}

pub fn get_groups_with_access(conn: &Conn, user_id: i32) -> Vec<Group> {
    let groups: Vec<Group> = group::db::find_all(conn);
    let group_accesses: Vec<GroupAccess> = group_access::db::find_by_user(conn, user_id);
    let group_relations: Vec<GroupRelation> = group_group_member::db::find_all(conn);
    let mut res: Vec<Group> = Vec::new();
    for group_access in &group_accesses {
        if group_access.access_level_recursive {
            let group = groups.iter().find(|x| x.id == group_access.group).unwrap().clone();
            res.push(group.clone());
            let rec_res: Vec<Group> = get_groups_with_access_recursive(group, groups.clone(), group_relations.clone());
            res = [res, rec_res.clone()].concat();
        } else {
            res.push(groups.iter().find(|x| x.id == group_access.group).unwrap().clone())
        }
    }
    res
}

pub fn get_groups_with_access_recursive(
    group: Group,
    groups: Vec<Group>,
    group_relations: Vec<GroupRelation>
) -> Vec<Group> {
    let mut children = Vec::new();
    let relations: Vec<GroupRelation> = group_relations
        .clone()
        .into_iter()
        .filter(|x| x.parent == group.id)
        .collect::<Vec<GroupRelation>>();
    for group_relation in &relations {
        let child_group = groups.iter().find(|x| x.id == group_relation.child).unwrap().clone();
        children.push(child_group.clone());
        children = [children, get_groups_with_access_recursive(child_group, groups.clone(), group_relations.clone()).clone()].concat()
    }
    children
}