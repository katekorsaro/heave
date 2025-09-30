use std::*;

mod fun;
mod imp;
mod mcr;
mod str;
mod trt;
mod tst;

pub use crate::str::attribute::O as Attribute;
pub use crate::str::catalog::O as Catalog;
pub use crate::str::entity::O as Entity;
pub use crate::str::value::E as Value;
pub use crate::trt::eav::T as EAV;

mod sqlite {
    pub mod init {
        pub use crate::fun::sqlite_init_db::run as db;
    }
    pub mod load {
        pub use crate::fun::sqlite_load_by_id::run as by_id;
    }
    pub mod map {
        pub use crate::fun::sqlite_map_row_to_attribute::run as row_to_attribute;
        pub use crate::fun::sqlite_map_row_to_entity::run as row_to_entity;
    }
    pub mod persist {
        pub use crate::fun::sqlite_persist_catalog::run as catalog;
    }
}
