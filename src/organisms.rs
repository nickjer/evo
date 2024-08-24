use crate::active_genome::ActiveGenome;
use crate::active_plant::ActivePlant;
use crate::either::Either::{self, *};
use crate::genome::Genome;
use crate::genomes::{GenomeId, Genomes};
use crate::grid::Grid;
use crate::inactive_genome::InactiveGenome;
use crate::inactive_plant::InactivePlant;
use crate::plants::{PlantId, Plants};
use crate::rand::Rng;
use crate::tiles::TileId;
use itertools::Itertools;

#[derive(Debug, Clone, Default)]
pub struct Organisms {
    genomes: Genomes,
    active_genomes: Vec<GenomeId>,
    plants: Plants,
    active_plants: Vec<PlantId>,
}

impl Organisms {
    pub fn active_plants(&self) -> &[PlantId] {
        &self.active_plants
    }

    pub fn plant(&self, plant_id: PlantId) -> &ActivePlant {
        self.plants[plant_id].as_ref().unwrap_living()
    }

    pub fn genome(&self, genome_id: GenomeId) -> &ActiveGenome {
        self.genomes[genome_id].as_ref().unwrap_living()
    }

    pub fn occupy(&mut self, plant_id: PlantId, tile_id: TileId, grid: &Grid) {
        let active_plant = self.plants[plant_id].as_mut().unwrap_living();
        let energy_yield = active_plant.occupy(tile_id, grid);

        let genome = self.genomes[active_plant.genome_id()]
            .as_mut()
            .unwrap_living();
        genome.set_max_yield(energy_yield);
    }

    pub fn abandon(&mut self, plant_id: PlantId, tile_id: TileId, grid: &Grid) -> Vec<TileId> {
        self.plants[plant_id]
            .as_mut()
            .unwrap_living()
            .abandon(tile_id, grid)
    }

    pub fn add_plant(&mut self, genome_id: GenomeId) -> PlantId {
        self.increment_genome(genome_id);

        let id = PlantId::from(self.plants.len());
        let active_plant = ActivePlant::new(id, genome_id);
        self.plants.push(Living(active_plant));
        self.active_plants.push(id);
        id
    }

    pub fn add_mutated_plant(
        &mut self,
        genome_id: GenomeId,
        round: usize,
        mut mutator: impl FnMut(f32) -> f32,
    ) -> PlantId {
        let new_genome = self.genome(genome_id).mutate(&mut mutator);
        let new_genome_id = self.add_genome(new_genome, Some(genome_id), round);
        self.add_plant(new_genome_id)
    }

    pub fn remove_plant(&mut self, plant_id: PlantId, round: usize) {
        let active_plant = self.plant(plant_id);
        let genome_id = active_plant.genome_id();
        self.decrement_genome(genome_id, round);

        self.active_plants.retain(|&id| id != plant_id);
        self.plants[plant_id] = Dead(InactivePlant::new(plant_id, genome_id));
    }

    pub fn add_genome(
        &mut self,
        genome: Genome,
        parent_genome_id: Option<GenomeId>,
        round: usize,
    ) -> GenomeId {
        let id = GenomeId::from(self.genomes.len());
        let active_genome = ActiveGenome::new(genome, parent_genome_id, round);
        self.genomes.push(Living(active_genome));
        self.active_genomes.push(id);
        id
    }

    pub fn choose_tile(
        &self,
        plant_id: PlantId,
        grid: &Grid,
        points: usize,
        rng: &mut Rng,
    ) -> Option<TileId> {
        let active_plant = self.plant(plant_id);
        let available_tiles = active_plant.available_tiles();
        let genome_id = active_plant.genome_id();
        let active_genome = self.genome(genome_id);
        active_genome.choose_tile(grid, available_tiles, plant_id, points, rng)
    }

    pub fn top_genomes(&mut self, n: usize) -> Vec<&Either<ActiveGenome, InactiveGenome>> {
        (&self.genomes)
            .into_iter()
            .k_largest_by_key(n, |genome| match genome {
                Living(active_genome) => active_genome.max_yield(),
                Dead(inactive_genome) => inactive_genome.max_yield(),
            })
            .collect()
    }

    fn increment_genome(&mut self, genome_id: GenomeId) {
        self.genomes[genome_id].as_mut().unwrap_living().increment();
    }

    fn decrement_genome(&mut self, genome_id: GenomeId, round: usize) {
        let genome = self.genomes[genome_id].as_mut().unwrap_living();
        let num_plants = genome.decrement();

        if num_plants == 0 {
            self.remove_genome(genome_id, round);
        }
    }

    fn remove_genome(&mut self, genome_id: GenomeId, round: usize) {
        let active_genome = self.genome(genome_id);
        let genome = active_genome.genome();
        let max_yield = active_genome.max_yield();
        let created_at = active_genome.created_at();
        let parent_genome_id = active_genome.parent_genome_id();
        self.genomes[genome_id] = Dead(InactiveGenome::new(
            *genome,
            max_yield,
            created_at,
            round,
            parent_genome_id,
        ));
        self.active_genomes.retain(|&id| id != genome_id);
    }
}
