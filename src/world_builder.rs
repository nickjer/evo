use crate::genome::GenomeKind;
use crate::genomes::{DoubletGenome, TripletGenome};
use crate::grid::Grid;
use crate::position::Position;
use crate::rand::Rng;
use crate::square_grid::SquareGrid;
use crate::tiles::TileId;
use crate::world::World;
use anyhow::Result;
use nohash::IntSet;

#[derive(Debug, Clone, Default)]
pub struct WorldBuilder {
    grid: SquareGrid,
    take_top: usize,
    seed_rate: f32,
    mutation_rate: f32,
    plants: Vec<(GenomeKind, TileId)>,
    unused_tiles: IntSet<TileId>,
}

impl WorldBuilder {
    pub fn new(x_size: usize, y_size: usize) -> Self {
        let grid = SquareGrid::new(x_size, y_size);
        let unused_tiles = grid.tile_id_iter().collect();
        WorldBuilder {
            grid,
            unused_tiles,
            ..Default::default()
        }
    }

    pub fn take_top(&mut self, take_top: usize) -> &mut Self {
        self.take_top = take_top;
        self
    }

    pub fn seed_rate(&mut self, seed_rate: f32) -> &mut Self {
        self.seed_rate = seed_rate;
        self
    }

    pub fn mutation_rate(&mut self, mutation_rate: f32) -> &mut Self {
        self.mutation_rate = mutation_rate;
        self
    }

    pub fn add_plant(&mut self, genome: GenomeKind, cell_position: Position) -> Result<&mut Self> {
        let tile_id = self.grid.id_at(cell_position);
        if self.unused_tiles.remove(&tile_id) {
            self.plants.push((genome, tile_id));
            Ok(self)
        } else {
            Err(anyhow::anyhow!("Tile already occupied: {cell_position:?}"))
        }
    }

    pub fn add_random_plants(
        &mut self,
        kind: &str,
        total: usize,
        rng: &mut Rng,
    ) -> Result<&mut Self> {
        let random_genomes = (0..total)
            .map(|_| match kind {
                "doublet_genome" => Ok(DoubletGenome::random(rng).into()),
                "triplet_genome" => Ok(TripletGenome::random(rng).into()),
                _ => Err(anyhow::anyhow!("Unknown plant kind: {}", kind)),
            })
            .collect::<Result<Vec<GenomeKind>>>()?;

        if random_genomes.len() > self.unused_tiles.len() {
            anyhow::bail!("Not enough empty tiles to place {total} random {kind} plants");
        }

        let mut empty_tiles = self.unused_tiles.iter().copied().collect::<Vec<_>>();
        rng.shuffle(&mut empty_tiles);
        random_genomes
            .into_iter()
            .zip(empty_tiles)
            .for_each(|(genome, tile_id)| {
                self.unused_tiles.remove(&tile_id);
                self.plants.push((genome, tile_id));
            });

        Ok(self)
    }

    pub fn build(self) -> World {
        let grid = Grid::new(self.grid);
        let mut world = World::new(grid, self.take_top, self.seed_rate, self.mutation_rate);
        self.plants.into_iter().for_each(|(genome, tile_id)| {
            let genome_id = world.add_genome(genome);
            world.add_plant(genome_id, tile_id);
        });
        world
    }
}
