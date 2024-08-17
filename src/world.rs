use crate::entity::Entity;
use crate::genome::Genome;
use crate::genomes::GenomeId;
use crate::grid::Grid;
use crate::organisms::Organisms;
use crate::plants::PlantId;
use crate::rand::Rng;
use crate::tiles::TileId;
use crate::trial_result;
use std::io::Write;

#[derive(Debug, Clone)]
pub struct World {
    col_size: usize,
    seed_rate: f32,
    mutation_rate: f32,
    grid: Grid,
    organisms: Organisms,
}

impl World {
    pub fn new(col_size: usize, grid: Grid, seed_rate: f32, mutation_rate: f32) -> Self {
        let organisms = Organisms::default();
        World {
            col_size,
            seed_rate,
            mutation_rate,
            grid,
            organisms,
        }
    }

    pub fn run(&mut self, rng: &mut Rng, max_steps: usize, snapshot_interval: usize) {
        let file = std::fs::File::create("data.js").unwrap();
        let mut file = std::io::LineWriter::new(file);

        let x = self.grid.size() / self.col_size;
        let y = self.col_size;
        writeln!(file, "const board = {{ x_size: {x}, y_size: {y} }};").unwrap();
        writeln!(file, "let tile_snapshots = [];").unwrap();
        Self::write_snapshot(&mut file, self.tile_snapshot());

        let mut tile_count = 0;
        while tile_count < max_steps {
            let plant_ids = self.organisms.active_plants().to_owned();
            plant_ids.into_iter().rev().for_each(|plant_id| {
                let mut points = self.organisms.points(plant_id, &self.grid);
                if points == 0 {
                    self.remove_plant(plant_id, rng);
                    return;
                }

                while let Some(tile_id) = self.organisms.choose_tile(plant_id, &self.grid, points) {
                    let old_entity = self.replace_entity(tile_id, Entity::Cell(plant_id));
                    if old_entity == Entity::Empty {
                        points = points.checked_sub(1).unwrap();
                    } else {
                        points = points.checked_sub(2).unwrap();
                    }

                    if points == 0 {
                        break;
                    }
                }
            });

            let plant_ids = self.organisms.active_plants();
            if plant_ids.len() < 2 {
                if plant_ids.is_empty() {
                    println!("No plants survived");
                } else {
                    println!("One plant survived");
                }
                break;
            }

            if tile_count % snapshot_interval == 0 {
                println!("Step: {}", tile_count);
                Self::write_snapshot(&mut file, self.tile_snapshot());
            }
            tile_count += 1;
        }

        let top_genomes = self.organisms.top_genomes(10);
        let trial_result = trial_result::TrialResult::new(top_genomes);
        let toml = toml::to_string_pretty(&trial_result).unwrap();
        std::fs::write("trial_result.toml", toml).unwrap();
    }

    fn replace_entity(&mut self, tile_id: TileId, new_entity: Entity) -> Entity {
        let old_entity = self.grid.replace_entity(tile_id, new_entity);

        if old_entity != new_entity {
            if let Entity::Cell(plant_id) = new_entity {
                self.organisms.occupy(plant_id, tile_id, &self.grid);
            }

            if let Entity::Cell(old_plant_id) = old_entity {
                let dead_cells = self.organisms.abandon(old_plant_id, tile_id, &self.grid);
                self.set_empty(&dead_cells);
            }
        }

        old_entity
    }

    fn remove_plant(&mut self, plant_id: PlantId, rng: &mut Rng) {
        let plant = self.organisms.plant(plant_id);
        let genome_id = plant.genome_id();
        let old_tiles = plant.cell_tiles();
        self.set_empty(&old_tiles);

        // Add plants after we clear out the old grid so the new plants are surrounded by the
        // correct environment
        for tile_id in old_tiles {
            // Should create a new plant?
            if rng.sample() < self.seed_rate {
                // Should create a new genome?
                let plant_id = if rng.sample() < self.mutation_rate {
                    let mutator = |value| value + rng.norm() * 0.1;
                    self.organisms.add_mutated_plant(genome_id, mutator)
                } else {
                    self.organisms.add_plant(genome_id)
                };
                self.replace_entity(tile_id, Entity::Cell(plant_id));
            }
        }
        self.organisms.remove_plant(plant_id);
    }

    fn set_empty(&mut self, tile_ids: &[TileId]) {
        tile_ids.iter().for_each(|&tile_id| {
            self.grid.replace_entity(tile_id, Entity::Empty);
        });
    }

    pub fn add_genome(&mut self, genome: Genome) -> GenomeId {
        self.organisms.add_genome(genome)
    }

    pub fn add_plant(&mut self, genome_id: GenomeId, tile_id: TileId) {
        let new_plant_id = self.organisms.add_plant(genome_id);
        self.replace_entity(tile_id, Entity::Cell(new_plant_id));
    }

    fn write_snapshot(writer: &mut impl Write, snapshot: Vec<Vec<usize>>) {
        writeln!(writer, "tile_snapshots.push([").unwrap();
        for row in snapshot {
            writeln!(
                writer,
                "  [{}],",
                row.iter()
                    .map(|&x| x.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
            .unwrap();
        }
        writeln!(writer, "]);").unwrap();
    }

    fn tile_snapshot(&self) -> Vec<Vec<usize>> {
        self.grid
            .entity_chunks(self.col_size)
            .into_iter()
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|&entity| match entity {
                        Entity::Empty => 0,
                        Entity::Cell(plant_id) => usize::from(plant_id) + 1,
                    })
                    .collect()
            })
            .collect()
    }
}
