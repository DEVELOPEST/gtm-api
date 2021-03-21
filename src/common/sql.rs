

pub const GROUP_CHILDREN_QUERY: &str =
    "WITH RECURSIVE group_repos_query AS
     (
        SELECT  group_group_members.child, 0 AS depth
        FROM    group_group_members
        WHERE   group_group_members.parent = (
            SELECT groups.id
            FROM groups
            WHERE groups.name = $1)
        UNION
        SELECT  m.child, group_repos_query.depth + 1
        FROM    group_group_members m
        JOIN    group_repos_query
        ON      m.parent = group_repos_query.child
        WHERE   group_repos_query.depth < 100
     )";

pub const GROUP_PARENTS_QUERY: &str =
    "WITH RECURSIVE group_parents_query AS
     (
         SELECT group_group_members.parent, 0 AS depth
         FROM group_group_members
         WHERE group_group_members.child = (
             SELECT groups.id
             FROM groups
             WHERE groups.name = $1)
         UNION
         SELECT m.parent, group_parents_query.depth + 1
         FROM group_group_members m
                  JOIN group_parents_query
                       ON m.child = group_parents_query.parent
         WHERE group_parents_query.depth < 100
     )";
