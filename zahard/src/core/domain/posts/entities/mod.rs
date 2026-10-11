use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "posts")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub thread_id: i64,
    pub local_number: i32,
    pub is_op: bool,
    pub body: String,
    pub image_id: Option<i64>,
    pub poster_id: Option<String>,
    pub ip_hash: Vec<u8>,
    pub is_sage: bool,
    pub is_deleted: bool,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::super::threads::entities::Entity",
        from = "Column::ThreadId",
        to = "super::super::threads::entities::Column::Id"
    )]
    Thread,
    #[sea_orm(
        belongs_to = "super::super::images::entities::Entity",
        from = "Column::ImageId",
        to = "super::super::images::entities::Column::Id"
    )]
    Image,
    #[sea_orm(
        has_many = "super::super::post_references::entities::Entity",
        from = "Column::Id",
        to = "super::super::post_references::entities::Column::PostId"
    )]
    PostReferences,
}

impl Related<super::super::threads::entities::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Thread.def()
    }
}

impl Related<super::super::images::entities::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Image.def()
    }
}

impl Related<super::super::post_references::entities::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::PostReferences.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
