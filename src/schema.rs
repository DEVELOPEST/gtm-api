table! {
    commits (id) {
        id -> Int4,
        repository_id -> Int4,
        hash -> Text,
        message -> Text,
        email -> Text,
        branch -> Text,
        timestamp -> Int8,
    }
}

table! {
    files (id) {
        id -> Int4,
        commit -> Int4,
        path -> Text,
        status -> Text,
        time -> Int8,
        lines_added -> Int8,
        lines_deleted -> Int8,
    }
}

table! {
    git_group_repository_members (repository, git_group) {
        repository -> Int4,
        git_group -> Int4,
    }
}

table! {
    git_groups (id) {
        id -> Int4,
        name -> Text,
        added_at -> Timestamptz,
    }
}

table! {
    repositories (id) {
        id -> Int4,
        username -> Text,
        provider -> Text,
        repo -> Text,
        sync_url -> Text,
        access_token -> Text,
        added_at -> Timestamptz,
    }
}

table! {
    timeline (id) {
        id -> Int4,
        file -> Int4,
        timestamp -> Int8,
        time -> Int8,
    }
}

table! {
    tokens (id) {
        id -> Int4,
        user -> Nullable<Int4>,
        access_token -> Text,
        added_at -> Timestamptz,
    }
}

table! {
    user_git_group_members (user, git_group) {
        user -> Int4,
        git_group -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Text,
        hash -> Text,
    }
}

joinable!(commits -> repositories (repository_id));
joinable!(files -> commits (commit));
joinable!(git_group_repository_members -> git_groups (git_group));
joinable!(git_group_repository_members -> repositories (repository));
joinable!(timeline -> files (file));
joinable!(tokens -> users (user));
joinable!(user_git_group_members -> git_groups (git_group));
joinable!(user_git_group_members -> users (user));

allow_tables_to_appear_in_same_query!(
    commits,
    files,
    git_group_repository_members,
    git_groups,
    repositories,
    timeline,
    tokens,
    user_git_group_members,
    users,
);
