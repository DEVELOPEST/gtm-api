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
    group_accesses (user, group) {
        user -> Int4,
        group -> Int4,
        access_level_recursive -> Bool,
    }
}

table! {
    group_group_members (parent, child) {
        parent -> Int4,
        child -> Int4,
    }
}

table! {
    groups (id) {
        id -> Int4,
        name -> Text,
        added_at -> Timestamptz,
    }
}

table! {
    repositories (id) {
        id -> Int4,
        group -> Int4,
        user -> Text,
        provider -> Text,
        repo -> Text,
        sync_url -> Text,
        access_token -> Text,
        added_at -> Timestamptz,
    }
}

table! {
    roles (id) {
        id -> Int4,
        name -> Text,
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
    user_group_members (user, group) {
        user -> Int4,
        group -> Int4,
    }
}

table! {
    user_role_members (user, role) {
        user -> Int4,
        role -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Text,
        password -> Text,
    }
}

joinable!(commits -> repositories (repository_id));
joinable!(files -> commits (commit));
joinable!(group_accesses -> groups (group));
joinable!(group_accesses -> users (user));
joinable!(timeline -> files (file));
joinable!(tokens -> users (user));
joinable!(user_group_members -> groups (group));
joinable!(user_group_members -> users (user));
joinable!(user_role_members -> roles (role));
joinable!(user_role_members -> users (user));

allow_tables_to_appear_in_same_query!(
    commits,
    files,
    group_accesses,
    group_group_members,
    groups,
    repositories,
    roles,
    timeline,
    tokens,
    user_group_members,
    user_role_members,
    users,
);
