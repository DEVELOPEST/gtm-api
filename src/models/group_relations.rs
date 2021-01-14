#[derive(Queryable)]
pub struct GroupRelation {
    pub parent: i32,
    pub child: i32,
}