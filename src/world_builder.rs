use crate::blob::Blob;
use crate::doublet::Doublet;
use crate::entity::Entity;
use crate::genome::Genome;
use crate::grid::Grid;
use crate::neighbors::Neighbors;
use crate::position::Position;
use crate::tile_id_builder::TileIdBuilder;
use crate::triplet_i::TripletI;
use crate::triplet_l::TripletL;
use crate::world::World;

#[derive(Debug, Clone)]
pub struct WorldBuilder {
    x_size: usize,
    y_size: usize,
    seed_rate: f32,
    mutation_rate: f32,
    plants: Vec<(Genome, Position)>,
}

impl WorldBuilder {
    pub fn new(x_size: usize, y_size: usize) -> Self {
        WorldBuilder {
            x_size,
            y_size,
            seed_rate: 0.1,
            mutation_rate: 0.1,
            plants: Vec::new(),
        }
    }

    pub fn add_plant(&mut self, genome: Genome, cell_position: Position) -> &mut Self {
        self.plants.push((genome, cell_position));
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

    pub fn build(self) -> World {
        let mut grid = Grid::default();
        for x in 0..self.x_size {
            for y in 0..self.y_size {
                let position = Position::new(x, y);
                let tile_id_builder = TileIdBuilder::new(position, self.x_size, self.y_size);
                let neighbors = Neighbors::build(tile_id_builder.clone());
                let doublets = Doublet::build(tile_id_builder.clone());
                let triplets_l = TripletL::build(tile_id_builder.clone());
                let triplets_i = TripletI::build(tile_id_builder.clone());
                let blob = Blob::build(tile_id_builder.clone());
                let entity = Entity::Empty;

                grid.push(entity, neighbors, doublets, triplets_l, triplets_i, blob);
            }
        }
        let mut world = World::new(self.y_size, grid, self.seed_rate, self.mutation_rate);
        self.plants.into_iter().for_each(|(genome, cell_position)| {
            let tile_id_builder = TileIdBuilder::new(cell_position, self.x_size, self.y_size);
            let tile_id = tile_id_builder.build();
            let genome_id = world.add_genome(genome);
            world.add_plant(genome_id, tile_id);
        });
        world
    }
}
