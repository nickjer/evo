use crate::genome::GenomeKind;
use crate::grid::Grid;
use crate::position::Position;
use crate::world::World;

#[derive(Debug, Clone)]
pub struct WorldBuilder {
    x_size: usize,
    y_size: usize,
    seed_rate: f32,
    mutation_rate: f32,
    plants: Vec<(GenomeKind, Position)>,
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

    pub fn add_plant(&mut self, genome: GenomeKind, cell_position: Position) -> &mut Self {
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
        let grid = Grid::new(self.x_size, self.y_size);
        let plants = self
            .plants
            .into_iter()
            .map(|(genome, cell_position)| {
                let tile_id = grid.id_at(cell_position);
                (genome, tile_id)
            })
            .collect::<Vec<_>>();

        let mut world = World::new(grid, self.seed_rate, self.mutation_rate);
        plants.into_iter().for_each(|(genome, tile_id)| {
            let genome_id = world.add_genome(genome);
            world.add_plant(genome_id, tile_id);
        });
        world
    }
}
