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
pub use crate::trt::to_eav::T as ToEAV;

mod sqlite {
    pub mod init {
        pub use crate::fun::sqlite_init_db::run as db;
    }
    pub mod persist {
        pub use crate::fun::sqlite_persist_catalog::run as catalog;
    }
}
