use std::*;

mod fun;
mod imp;
mod mcr;
mod str;
mod trt;
mod tst;

pub(crate) use crate::str::attribute::O as Attribute;
pub use crate::str::catalog::O as Catalog;
pub use crate::str::comparison::E as Comparison;
pub use crate::str::condition::E as Condition;
pub use crate::str::entity::O as Entity;
pub use crate::str::entity_state::EntityState;
pub use crate::str::failed_to::FailedTo;
pub use crate::str::filter::O as Filter;
pub(crate) use crate::str::value::Value;
pub use crate::trt::eav::T as EAV;

mod sqlite {
    pub use crate::str::sqlite_failed_to::FailedTo;
    pub mod build {
        pub use crate::fun::sqlite_build_params::run as params;
        pub use crate::fun::sqlite_build_statement::run as statement;
    }
    pub mod init {
        pub use crate::fun::sqlite_init_db::run as db;
    }
    pub mod load {
        pub use crate::fun::sqlite_load_attributes::run as attributes;
        pub use crate::fun::sqlite_load_by_class::run as by_class;
        pub use crate::fun::sqlite_load_by_filter::run as by_filter;
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
