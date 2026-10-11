use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "images")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub disk_name: String,
    pub thumb_name: Option<String>,
    pub mime_type: String,
    pub size_bytes: i32,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub sha256: Vec<u8>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::super::posts::entities::Entity")]
    Posts,
}

impl Related<super::super::posts::entities::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Posts.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
