use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "threads")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub board_id: i16,
    pub subject: Option<String>,
    pub is_pinned: bool,
    pub is_locked: bool,
    pub is_archived: bool,
    pub reply_count: i32,
    pub image_count: i32,
    pub bumped_at: DateTimeWithTimeZone,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::super::boards::entities::Entity",
        from = "Column::BoardId",
        to = "super::super::boards::entities::Column::Id"
    )]
    Board,
    #[sea_orm(has_many = "super::super::posts::entities::Entity")]
    Posts,
}

impl Related<super::super::boards::entities::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Board.def()
    }
}

impl Related<super::super::posts::entities::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Posts.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
