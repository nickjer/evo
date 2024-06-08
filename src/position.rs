use derive_more::Constructor;
use getset::CopyGetters;
use serde::Deserialize;

#[derive(Debug, Copy, Clone, Constructor, CopyGetters, Deserialize, Hash, PartialEq, Eq)]
pub struct Position {
    #[get_copy = "pub"]
    x: usize,
    #[get_copy = "pub"]
    y: usize,
}
