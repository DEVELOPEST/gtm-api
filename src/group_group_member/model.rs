#[derive(Queryable, Debug, Clone)]
pub struct GroupRelation {
    pub parent: i32,
    pub child: i32,
}