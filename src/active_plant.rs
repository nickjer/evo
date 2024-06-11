use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::active_genome::ActiveGenome;
use crate::genomes::GenomeId;
use crate::grid::Grid;
use crate::owner::Owner;
use crate::plants::PlantId;
use crate::simple_graph::{all_connected, components, SimpleGraph};
use crate::tiles::TileId;
use derive_more::Constructor;
use getset::CopyGetters;
use itertools::Itertools;
use nohash::{IntMap, IntSet};

#[derive(Debug, Clone, CopyGetters, Default)]
pub struct ActivePlant {
    id: PlantId,
    #[get_copy = "pub"]
    genome_id: GenomeId,
    cells: SimpleGraph,
    surface_map: IntMap<TileId, usize>,
}

impl ActivePlant {
    pub fn new(id: PlantId, genome_id: GenomeId) -> Self {
        ActivePlant {
            id,
            genome_id,
            ..Default::default()
        }
    }

    pub fn cell_tiles(&self) -> Vec<TileId> {
        self.cells.nodes()
    }

    pub fn occupy(&mut self, tile_id: TileId, grid: &Grid) {
        let unlinked_neighbor_ids = self.cells.add_node(tile_id, grid.neighbors(tile_id));
        self.surface_map.remove(&tile_id);
        unlinked_neighbor_ids.into_iter().for_each(|neighbor_id| {
            self.surface_map
                .entry(neighbor_id)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        });
    }

    pub fn abandon(&mut self, tile_id: TileId, grid: &Grid) -> Vec<TileId> {
        let neighboring_cells: Vec<_> = self.remove_cell(tile_id, grid).into_iter().collect();

        // Make sure all neighboring cells are connected to each other
        if !all_connected(&self.cells, &neighboring_cells, grid.size()) {
            let mut components = components(&self.cells, grid.size());
            components.sort_by_key(|component| component.len());
            let dead_cells: Vec<TileId> = components
                .split_last()
                .unwrap()
                .1
                .iter()
                .flat_map(|component| component.iter())
                .copied()
                .collect();
            dead_cells.iter().for_each(|&dead_tile_id| {
                self.remove_cell(dead_tile_id, grid);
            });
            return dead_cells;
        }

        Vec::new()
    }

    pub fn choose_tiles(&self, grid: &Grid, genome: &mut ActiveGenome) -> Vec<TileId> {
        let energy_yield = self.energy_yield(grid);
        let energy_usage = self.energy_usage();
        if energy_yield <= energy_usage {
            return Vec::new();
        }

        genome.set_max_yield(energy_yield);

        let mut k = energy_yield / energy_usage;
        if k == 0 {
            return Vec::new();
        }

        let mut heap_cost_1 = MaxHeap::with_size(k);
        let mut heap_cost_2 = MaxHeap::with_size(k / 2);

        self.surface_map
            .keys()
            .for_each(|&tile_id| match grid.owner(tile_id) {
                Owner::Empty => {
                    heap_cost_1.push_with(|| {
                        ScoredTile::new(tile_id, genome.score(self.id, grid, tile_id), 1)
                    });
                }
                Owner::Cell(plant_id) if plant_id == self.id => (),
                Owner::Cell(_) => {
                    heap_cost_2.push_with(|| {
                        ScoredTile::new(tile_id, genome.score(self.id, grid, tile_id), 2)
                    });
                }
            });

        [heap_cost_1.into_vec(), heap_cost_2.into_vec()]
            .concat()
            .into_iter()
            .sorted_unstable()
            .rev()
            .filter(|tile| {
                if k >= tile.cost {
                    k -= tile.cost;
                    true
                } else {
                    false
                }
            })
            .map(|tile| tile.id())
            .collect()
    }

    fn energy_usage(&self) -> usize {
        self.cells.node_count()
    }

    fn energy_yield(&self, grid: &Grid) -> usize {
        self.surface_map
            .iter()
            .filter(|(&surface_tile_id, _count)| grid.is_empty(surface_tile_id))
            .map(|(_surface_tile_id, &count)| count)
            .sum()
    }

    fn remove_cell(&mut self, tile_id: TileId, grid: &Grid) -> IntSet<TileId> {
        let neighboring_cells = self.cells.remove_node(tile_id);
        if !neighboring_cells.is_empty() {
            self.surface_map.insert(tile_id, neighboring_cells.len());
        }

        let original_neighbors = grid.neighbors(tile_id).as_set();
        original_neighbors
            .difference(&neighboring_cells)
            .for_each(|&unowned_tile_id| {
                if let Some(count) = self.surface_map.get_mut(&unowned_tile_id) {
                    if *count == 1 {
                        self.surface_map.remove(&unowned_tile_id);
                    } else {
                        *count -= 1;
                    }
                }
            });
        neighboring_cells
    }
}

#[derive(Debug, Clone)]
struct MaxHeap {
    size: usize,
    heap: BinaryHeap<Reverse<ScoredTile>>,
}

impl MaxHeap {
    fn with_size(size: usize) -> Self {
        MaxHeap {
            size,
            heap: BinaryHeap::with_capacity(size),
        }
    }

    fn push_with(&mut self, mut f: impl FnMut() -> ScoredTile) {
        if self.size == 0 {
            return;
        }

        let scored_tile = f();
        if self.heap.len() < self.size {
            self.heap.push(Reverse(scored_tile));
        } else if let Some(Reverse(min)) = self.heap.peek() {
            if scored_tile > *min {
                *self.heap.peek_mut().unwrap() = Reverse(scored_tile);
            }
        }
    }

    fn into_vec(self) -> Vec<ScoredTile> {
        self.heap
            .into_sorted_vec()
            .into_iter()
            .map(|Reverse(tile)| tile)
            .collect()
    }
}

#[derive(Debug, Copy, Clone, Constructor, CopyGetters)]
struct ScoredTile {
    #[get_copy = "pub"]
    id: TileId,
    score: f32,
    cost: usize,
}

impl Eq for ScoredTile {}

impl PartialEq for ScoredTile {
    fn eq(&self, other: &Self) -> bool {
        self.score == other.score
    }
}

impl Ord for ScoredTile {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.partial_cmp(&other.score).unwrap()
    }
}

impl PartialOrd for ScoredTile {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
