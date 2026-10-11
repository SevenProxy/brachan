use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "post_references")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub post_id: i64,
    #[sea_orm(primary_key, auto_increment = false)]
    pub referenced_id: i64,
    pub thread_id: i64,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::super::posts::entities::Entity",
        from = "Column::PostId",
        to = "super::super::posts::entities::Column::Id"
    )]
    Post,
    #[sea_orm(
        belongs_to = "super::super::posts::entities::Entity",
        from = "Column::ReferencedId",
        to = "super::super::posts::entities::Column::Id"
    )]
    ReferencedPost,
    #[sea_orm(
        belongs_to = "super::super::threads::entities::Entity",
        from = "Column::ThreadId",
        to = "super::super::threads::entities::Column::Id"
    )]
    Thread,
}

impl Related<super::super::posts::entities::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Post.def()
    }
}

impl Related<super::super::threads::entities::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Thread.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
