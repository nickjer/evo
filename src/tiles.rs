use derive_more::{From, Into};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, From, Into)]
pub struct TileId(usize);

impl std::hash::Hash for TileId {
    fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
        hasher.write_usize(self.0)
    }
}

impl nohash::IsEnabled for TileId {}

#[derive(Debug, Clone)]
pub struct Tiles<T>(Vec<T>);

impl<T> Tiles<T> {
    pub fn push(&mut self, value: T) {
        self.0.push(value);
    }

    pub fn chunks(&self, chunk_size: usize) -> impl Iterator<Item = &[T]> {
        self.0.chunks(chunk_size)
    }
}

impl<T> Default for Tiles<T> {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl<T> std::ops::Index<TileId> for Tiles<T> {
    type Output = T;

    #[inline]
    fn index(&self, tile_id: TileId) -> &Self::Output {
        self.0.index(usize::from(tile_id))
    }
}

impl<T> std::ops::IndexMut<TileId> for Tiles<T> {
    #[inline]
    fn index_mut(&mut self, tile_id: TileId) -> &mut Self::Output {
        self.0.index_mut(usize::from(tile_id))
    }
}
