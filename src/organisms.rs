use crate::active_genome::ActiveGenome;
use crate::active_plant::ActivePlant;
use crate::either::Either::{self, *};
use crate::genome::Genome;
use crate::genomes::{GenomeId, Genomes};
use crate::grid::Grid;
use crate::inactive_genome::InactiveGenome;
use crate::inactive_plant::InactivePlant;
use crate::plants::{PlantId, Plants};
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

    pub fn occupy(&mut self, plant_id: PlantId, tile_id: TileId, grid: &Grid) {
        self.plants[plant_id]
            .as_mut()
            .unwrap_living()
            .occupy(tile_id, grid);
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
        mut mutator: impl FnMut(f32) -> f32,
    ) -> PlantId {
        let new_genome = self.genomes[genome_id]
            .as_ref()
            .unwrap_living()
            .mutate(&mut mutator);
        let new_genome_id = self.add_genome(new_genome);
        self.add_plant(new_genome_id)
    }

    pub fn remove_plant(&mut self, plant_id: PlantId) {
        let active_plant = self.plants[plant_id].as_ref().unwrap_living();
        let genome_id = active_plant.genome_id();
        self.decrement_genome(genome_id);

        self.active_plants.retain(|&id| id != plant_id);
        self.plants[plant_id] = Dead(InactivePlant::new(plant_id, genome_id));
    }

    pub fn add_genome(&mut self, genome: Genome) -> GenomeId {
        let id = GenomeId::from(self.genomes.len());
        let active_genome = ActiveGenome::new(genome);
        self.genomes.push(Living(active_genome));
        self.active_genomes.push(id);
        id
    }

    pub fn choose_tiles(&mut self, plant_id: PlantId, grid: &Grid) -> Vec<TileId> {
        let active_plant = self.plants[plant_id].as_ref().unwrap_living();
        let genome_id = active_plant.genome_id();
        let active_genome = self.genomes[genome_id].as_mut().unwrap_living();
        active_plant.choose_tiles(grid, active_genome)
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

    fn decrement_genome(&mut self, genome_id: GenomeId) {
        let genome = self.genomes[genome_id].as_mut().unwrap_living();
        let num_plants = genome.decrement();

        if num_plants == 0 {
            self.remove_genome(genome_id);
        }
    }

    fn remove_genome(&mut self, genome_id: GenomeId) {
        let active_genome = self.genomes[genome_id].as_ref().unwrap_living();
        let genome = active_genome.genome();
        let max_yield = active_genome.max_yield();
        self.genomes[genome_id] = Dead(InactiveGenome::new(*genome, max_yield));
        self.active_genomes.retain(|&id| id != genome_id);
    }
}
