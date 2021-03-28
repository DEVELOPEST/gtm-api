use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use crate::group::resource::{GroupFileStatsJson, GroupUserStatsJson, GroupExportDataJson};
use crate::group::dwh::{GroupFileStats, GroupFileStatsWrapper, GroupUserStats, GroupExportData};
use crate::timeline::mapper::cut_path;

pub fn map_group_file_stats(data: &Vec<GroupFileStats>, depth: i32) -> Vec<GroupFileStatsJson> {
    let mut result: HashMap<String, GroupFileStatsWrapper> = Default::default();
    for file in data {
        if file.path.ends_with(".app") {
            continue;
        }
        let path = cut_path(&file.path, depth);
        let entry = result.get_mut(&path);
        if entry.is_some() {
            let entry = entry.unwrap();
            entry.commits += file.commits;
            entry.users.insert(file.user.clone());
            entry.lines_removed += file.lines_removed;
            entry.lines_added += file.lines_added;
            entry.total_time += file.total_time;
        } else {
            result.insert(path.clone(), GroupFileStatsWrapper {
                path,
                total_time: file.total_time,
                lines_added: file.lines_added,
                lines_removed: file.lines_removed,
                commits: file.commits,
                users: HashSet::from_iter(vec![file.user.clone()].into_iter()),
            });
        }
    }
    let mut res_values: Vec<GroupFileStatsJson> = result.values()
        .map(|v| {
            let users_count = v.users.len() as i64;
            GroupFileStatsJson {
                path: v.path.clone(),
                total_time: (v.total_time as f64 / 60.0 / 60.0 * 10.0).round() / 10.0,
                time_per_user: (v.total_time as f64 / users_count as f64 / 60.0 / 60.0 * 10.0).round() / 10.0,
                lines_added: v.lines_added / users_count,
                lines_removed: v.lines_removed / users_count,
                total_commits: v.commits,
                commits_per_user: (v.commits as f64 / users_count as f64 * 10.0).round() / 10.0,
                commits_per_hour: if v.total_time == 0 { 0.0 } else {
                    (v.commits as f64 * 60.0 * 60.0 / users_count as f64 / v.total_time as f64 * 10.0).round() / 10.0
                },
                users: users_count,
                lines_per_hour: if v.total_time == 0 { 0 } else {
                    ((v.lines_added + v.lines_removed) * 60 * 60 / v.total_time) as i32
                },
            }
        }).collect();
    res_values.sort_by(|a, b| b.total_time.partial_cmp(&a.total_time).unwrap());
    res_values
}

pub fn map_group_user_stats(data: &Vec<GroupUserStats>) -> Vec<GroupUserStatsJson> {
    data.iter()
        .map(|u| GroupUserStatsJson {
            name: u.name.clone(),
            total_time: (u.total_time as f64 / 60.0 / 60.0 * 10.0).round() / 10.0,
            lines_added: u.lines_added,
            lines_removed: u.lines_removed,
            lines_per_hour: if u.total_time == 0 { 0 } else {
                ((u.lines_added + u.lines_removed) * 60 * 60 / u.total_time) as i32
            },
            commits: u.commits,
            commits_per_hour: if u.total_time == 0 { 0.0 } else {
                (u.commits as f64 * 60.0 * 60.0 / u.total_time as f64 * 10.0).round() / 10.0
            },
            lines_per_commit: if u.commits == 0 { 0.0 } else {
                ((u.lines_added + u.lines_removed) as f64 / u.commits as f64 * 20.0).round() / 10.0
            }
        })
        .collect()
}

pub fn map_group_export_data(data: &Vec<GroupExportData>, depth: i32) -> Vec<GroupExportDataJson> {
    let mut result: HashMap<String, GroupExportDataJson> = Default::default();
    for file in data {
        let path = cut_path(&file.path, depth);
        let entry = result.get_mut(&format!("{}-{}-{}-{}-{}",
                                            &path, file.user, file.provider, file.repository, file.timestamp));
        if entry.is_some() {
            let entry = entry.unwrap();
            entry.total_time += file.total_time;
            entry.lines_added += file.lines_added;
            entry.lines_removed += file.lines_removed;
            entry.files_count += 1;
        } else {
            result.insert(
                format!("{}-{}-{}-{}-{}", &path, file.user, file.provider, file.repository, file.timestamp),
                GroupExportDataJson {
                    user: file.user.clone(),
                    provider: file.provider.clone(),
                    repository: file.repository.clone(),
                    path,
                    is_app: file.path.ends_with(".app"),
                    files_count: 1,
                    timestamp: file.timestamp,
                    message: file.message.clone(),
                    total_time: file.total_time,
                    lines_added: file.lines_added,
                    lines_removed: file.lines_removed
                });
        }
    }
    result.into_iter().map(|(_, v)| v).collect()
}