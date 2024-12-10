//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.1

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, DeriveActiveEnum, Serialize, Deserialize)]
#[sea_orm(rs_type = "String", db_type = "Enum", enum_name = "permission")]
pub enum Permission {
    #[sea_orm(string_value = "Private")]
    Private,
    #[sea_orm(string_value = "Public")]
    Public,
    #[sea_orm(string_value = "Restricted")]
    Restricted,
}
