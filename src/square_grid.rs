use crate::position::Position;
use crate::step::Step;
use crate::tiles::{TileId, Tiles};
use derive_more::derive::IntoIterator;
use getset::CopyGetters;

#[derive(Debug, Copy, Clone, IntoIterator)]
#[into_iterator(owned, ref, ref_mut)]
pub struct Neighbors<T>([T; 4]);

impl<T> Neighbors<T> {
    fn new(up: T, right: T, down: T, left: T) -> Self {
        Self([up, right, down, left])
    }

    fn up(&self) -> T
    where
        T: Copy,
    {
        self.0[0]
    }

    fn right(&self) -> T
    where
        T: Copy,
    {
        self.0[1]
    }

    fn down(&self) -> T
    where
        T: Copy,
    {
        self.0[2]
    }

    fn left(&self) -> T
    where
        T: Copy,
    {
        self.0[3]
    }

    pub fn map<F, U>(self, f: F) -> Neighbors<U>
    where
        F: FnMut(T) -> U,
        T: Copy,
    {
        Neighbors(self.0.map(f))
    }
}

#[derive(Debug, Clone, Default, CopyGetters)]
pub struct SquareGrid {
    #[get_copy = "pub"]
    x_size: usize,
    #[get_copy = "pub"]
    y_size: usize,
    neighbors: Tiles<Neighbors<TileId>>,
}

impl SquareGrid {
    pub fn new(x_size: usize, y_size: usize) -> Self {
        let mut neighbors = Tiles::default();
        let col_size = y_size;
        for x in 0..x_size {
            for y in 0..y_size {
                let up = Self::id(x, Self::increment(y, y_size), col_size);
                let down = Self::id(x, Self::decrement(y, y_size), col_size);
                let right = Self::id(Self::increment(x, x_size), y, col_size);
                let left = Self::id(Self::decrement(x, x_size), y, col_size);
                neighbors.push(Neighbors::new(up, right, down, left));
            }
        }
        SquareGrid {
            x_size,
            y_size,
            neighbors,
        }
    }

    pub fn size(&self) -> usize {
        self.x_size.checked_mul(self.y_size).unwrap()
    }

    pub fn tile_id_iter(&self) -> impl Iterator<Item = TileId> {
        (0..self.size()).map(TileId::from)
    }

    pub fn neighbors(&self, tile_id: TileId) -> &Neighbors<TileId> {
        &self.neighbors[tile_id]
    }

    pub fn id_at(&self, position: Position) -> TileId {
        Self::id(position.x(), position.y(), self.y_size)
    }

    pub fn id_from(&self, tile_id: TileId, walk: &[Step]) -> TileId {
        let mut tile_id = tile_id;
        for step in walk {
            tile_id = match step {
                Step::Right => self.neighbors[tile_id].right(),
                Step::Left => self.neighbors[tile_id].left(),
                Step::Up => self.neighbors[tile_id].up(),
                Step::Down => self.neighbors[tile_id].down(),
            }
        }
        tile_id
    }

    fn id(x: usize, y: usize, column_size: usize) -> TileId {
        let idx = x
            .checked_mul(column_size)
            .and_then(|idx| idx.checked_add(y))
            .unwrap();
        TileId::from(idx)
    }

    fn increment(value: usize, size: usize) -> usize {
        assert!(value < size);
        if value == size - 1 {
            0
        } else {
            value + 1
        }
    }

    fn decrement(value: usize, size: usize) -> usize {
        assert!(value < size);
        if value == 0 {
            size - 1
        } else {
            value - 1
        }
    }
}
