use crate::domain::db::Conn;
use crate::domain::{group, group_access, group_group_member};
use crate::domain::group::mapper::{map_group_file_stats, map_group_user_stats};
use crate::domain::group::model::Group;
use crate::domain::group::resource::{GroupExportDataJson, GroupStatsJson};
use crate::domain::group_access::model::GroupAccess;
use crate::domain::group_group_member::model::GroupRelation;
use crate::errors::Error;

pub fn add_group_relations(conn: &Conn, parents_vec: Vec<String>, children_vec: Vec<String>) -> Result<(), Error> {
    for child in children_vec {
        let relation_child = group::db::find(&conn, &child)
            .unwrap_or_else(|_| group::db::create(&conn, &child).unwrap());

        for parent in &parents_vec {
            let relation_parent = if !group::db::exists(&conn, &parent) {
                group::db::create(&conn, &parent)?
            } else {
                group::db::find(&conn, &parent)?
            };
            if !group_group_member::db::exists(&conn, &relation_parent.id, &relation_child.id) {
                group_group_member::db::create(&conn, relation_parent.id, relation_child.id);
            }
        }
    }
    Ok(())
}

pub fn get_group_stats(
    conn: &Conn,
    group_name: &String,
    start: i64,
    end: i64,
    depth: i32,
) -> Result<GroupStatsJson, Error> {
    let user_stats = group::db::fetch_group_user_stats(conn, group_name, start, end)?;
    let file_stats = group::db::fetch_group_file_stats(conn, group_name, start, end)?;
    Ok(GroupStatsJson {
        users: map_group_user_stats(&user_stats),
        files: map_group_file_stats(&file_stats, depth),
    })
}

pub fn export_group_data(conn: &Conn, group_name: &str, start: i64, end: i64, depth: i32) -> Result<Vec<GroupExportDataJson>, Error> {
    let data = group::db::fetch_group_export_data(conn, group_name, start, end)?;
    Ok(group::mapper::map_group_export_data(data, depth))
}

pub fn get_groups_without_access(conn: &Conn, user_id: i32) -> Result<Vec<Group>, Error> {
    let groups: Vec<Group> = group::db::find_all(conn)?;
    let groups_with_access: Vec<Group> = get_groups_with_access(conn, user_id)?;
    let mut res: Vec<Group> = Vec::new();
    for group in &groups {
        if !groups_with_access.contains(group) {
            res.push(group.clone());
        }
    }
    Ok(res)
}

pub fn get_groups_with_access(conn: &Conn, user_id: i32) -> Result<Vec<Group>, Error> {
    let groups: Vec<Group> = group::db::find_all(conn)?;
    let group_accesses: Vec<GroupAccess> = group_access::db::find_by_user(conn, user_id);
    let group_relations: Vec<GroupRelation> = group_group_member::db::find_all(conn);
    let mut res: Vec<Group> = vec![];
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
    res.sort_by_key(|e| e.id);
    res.dedup_by_key(|e| e.id);
    Ok(res)
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
