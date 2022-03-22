use sea_orm::{entity::prelude::*, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::utils::ColumnFinder;
use validator::{Validate};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize, Validate)]
#[sea_orm(table_name = "todo")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Uuid")]
    #[serde(skip_deserializing)]
    pub id: Uuid,

    #[sea_orm(column_type = "Text")]
    #[validate(length(min = 1))]
    pub name: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {
    fn new() -> Self {
        Self {
            id: Set(Uuid::new_v4()),
            ..ActiveModelTrait::default()
        }
    }
}

impl ColumnFinder<Column> for Column {
    fn find_col_by_name(col_name: &str) -> Option<Column> {
        match col_name {
            "name" => {
                Some(Self::Name)
            },
            _ => None,
        }
    }
}


