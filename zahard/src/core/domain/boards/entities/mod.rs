use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "boards")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i16,
    #[sea_orm(unique)]
    pub slug: String,
    pub name: String,
    pub description: String,
    pub bump_limit: i16,
    pub max_threads: i16,
    pub is_nsfw: bool,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::super::threads::entities::Entity")]
    Threads,
}

impl Related<super::super::threads::entities::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Threads.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
