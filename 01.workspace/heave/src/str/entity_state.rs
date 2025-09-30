#[derive(Debug, Default, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash)]
pub enum E {
    #[default]
    New,
    Loaded,
}
