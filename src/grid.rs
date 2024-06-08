use crate::genome::Genome;
use crate::genomes::{GenomeId, Genomes};
use crate::owner::Owner;
use crate::plants::{PlantId, Plants};
use crate::rand::Rng;
use crate::tile_list::TileId;
use crate::tiles::Tiles;
use std::io::Write;

#[derive(Debug, Clone)]
pub struct Grid {
    col_size: usize,
    seed_rate: f32,
    mutation_rate: f32,
    tiles: Tiles,
    genomes: Genomes,
    plants: Plants,
}

impl Grid {
    pub fn new(col_size: usize, tiles: Tiles, seed_rate: f32, mutation_rate: f32) -> Self {
        let genomes = Genomes::default();
        let plants = Plants::default();
        Grid {
            col_size,
            seed_rate,
            mutation_rate,
            tiles,
            genomes,
            plants,
        }
    }

    pub fn run(&mut self, rng: &mut Rng, max_steps: usize, snapshot_interval: usize) {
        let file = std::fs::File::create("data.js").unwrap();
        let mut file = std::io::LineWriter::new(file);

        let x = self.tiles.size() / self.col_size;
        let y = self.col_size;
        writeln!(file, "const board = {{ x_size: {x}, y_size: {y} }};").unwrap();
        writeln!(file, "let tile_snapshots = [];").unwrap();
        Self::write_snapshot(&mut file, self.tile_snapshot());

        let mut tile_count = 0;
        while tile_count < max_steps {
            let plant_ids = self.plants.active_ids();
            plant_ids.into_iter().rev().for_each(|plant_id| {
                let plant = &self.plants[plant_id];
                let replacement_tile_ids = plant.next_tile_ids(&self.tiles, &mut self.genomes);

                if replacement_tile_ids.is_empty() {
                    self.remove_plant(plant_id, rng);
                } else {
                    replacement_tile_ids.into_iter().for_each(|tile_id| {
                        self.replace_owner(tile_id, Owner::Cell(plant_id));
                    });
                }
            });

            if self.plants.active_len() < 2 {
                if let Some(&plant_id) = self.plants.active_ids().first() {
                    let plant = &self.plants[plant_id];
                    let genome = &self.genomes[plant.genome_id()];
                    println!("[{}] survived with {:?}", plant.id(), genome);
                } else {
                    println!("All plants died");
                }
                break;
            }

            if tile_count % snapshot_interval == 0 {
                println!("Step: {}", tile_count);
                Self::write_snapshot(&mut file, self.tile_snapshot());
            }
            tile_count += 1;
        }
    }

    fn replace_owner(&mut self, tile_id: TileId, new_owner: Owner) -> Owner {
        let old_owner = self.tiles.replace_owner(tile_id, new_owner);

        if old_owner != new_owner {
            if let Owner::Cell(plant_id) = new_owner {
                self.plants[plant_id].add_cell(tile_id, &self.tiles);
            }

            if let Owner::Cell(old_plant_id) = old_owner {
                let dead_cells =
                    self.plants[old_plant_id].remove_cell_and_branch(tile_id, &self.tiles);
                self.remove_branch(&dead_cells);
            }
        }

        old_owner
    }

    fn remove_plant(&mut self, plant_id: PlantId, rng: &mut Rng) {
        let plant = self.plants.remove(plant_id);

        let old_tile_ids = plant.cells();
        self.remove_branch(&old_tile_ids);

        // Add plants after we clear out the old tiles so the new plants are surrounded by the
        // correct environment
        let genome_id = plant.genome_id();
        for tile_id in old_tile_ids {
            if rng.sample() < self.seed_rate {
                let new_genome_id = if rng.sample() < self.mutation_rate {
                    let new_genome =
                        self.genomes[genome_id].mutate(|value| value + rng.norm() * 0.1);
                    self.add_genome(new_genome)
                } else {
                    genome_id
                };
                self.add_plant(new_genome_id, tile_id);
            }
        }
    }

    fn remove_branch(&mut self, tile_ids: &[TileId]) {
        tile_ids.iter().for_each(|&tile_id| {
            self.tiles.replace_owner(tile_id, Owner::Empty);
        });
    }

    pub fn add_genome(&mut self, genome: Genome) -> GenomeId {
        self.genomes.add(genome)
    }

    pub fn add_plant(&mut self, genome_id: GenomeId, tile_id: TileId) {
        let new_plant_id = self.plants.add(genome_id).id();
        self.replace_owner(tile_id, Owner::Cell(new_plant_id));
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
        self.tiles
            .owner_chunks(self.col_size)
            .into_iter()
            .map(|chunk| {
                chunk
                    .iter()
                    .map(|&owner| match owner {
                        Owner::Empty => 0,
                        Owner::Cell(plant_id) => usize::from(plant_id) + 1,
                    })
                    .collect()
            })
            .collect()
    }
}
