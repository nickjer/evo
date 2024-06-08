use crate::genomes::GenomeId;
use crate::genomes::Genomes;
use crate::owner::Owner;
use crate::plants::PlantId;
use crate::simple_graph::{all_connected, components, SimpleGraph};
use crate::tile_list::TileId;
use crate::tiles::Tiles;
use derive_more::Constructor;
use getset::CopyGetters;
use itertools::Itertools;
use nohash::{IntMap, IntSet};

#[derive(Debug, Clone, CopyGetters, Default)]
pub struct Plant {
    id: PlantId,
    cells: SimpleGraph,
    surface_map: IntMap<TileId, usize>,
    #[get_copy = "pub"]
    genome_id: GenomeId,
}

impl Plant {
    pub fn new(id: PlantId, genome_id: GenomeId) -> Self {
        Plant {
            id,
            genome_id,
            ..Default::default()
        }
    }

    pub fn id(&self) -> PlantId {
        self.id
    }

    pub fn cells(&self) -> Vec<TileId> {
        self.cells.nodes()
    }

    pub fn add_cell(&mut self, tile_id: TileId, tiles: &Tiles) {
        let unlinked_neighbor_ids = self.cells.add_node(tile_id, tiles.neighbors(tile_id));
        self.surface_map.remove(&tile_id);
        unlinked_neighbor_ids.into_iter().for_each(|neighbor_id| {
            self.surface_map
                .entry(neighbor_id)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        });
    }

    fn remove_cell(&mut self, tile_id: TileId, tiles: &Tiles) -> IntSet<TileId> {
        let neighboring_cells = self.cells.remove_node(tile_id);
        if !neighboring_cells.is_empty() {
            self.surface_map.insert(tile_id, neighboring_cells.len());
        }

        let original_neighbors = tiles.neighbors(tile_id).as_set();
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

    pub fn remove_cell_and_branch(&mut self, tile_id: TileId, tiles: &Tiles) -> Vec<TileId> {
        let neighboring_cells: Vec<_> = self.remove_cell(tile_id, tiles).into_iter().collect();

        // Make sure all neighboring cells are connected to each other
        if !all_connected(&self.cells, &neighboring_cells, tiles.size()) {
            let mut components = components(&self.cells, tiles.size());
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
                self.remove_cell(dead_tile_id, tiles);
            });
            return dead_cells;
        }

        Vec::new()
    }

    fn energy_usage(&self) -> usize {
        self.cells.node_count()
    }

    fn energy_yield(&self, tiles: &Tiles) -> usize {
        self.surface_map
            .iter()
            .filter(|(&surface_tile_id, _count)| tiles.is_empty(surface_tile_id))
            .map(|(_surface_tile_id, &count)| count)
            .sum()
    }

    pub fn next_tile_ids(&self, tiles: &Tiles, genomes: &mut Genomes) -> Vec<TileId> {
        let energy_yield = self.energy_yield(tiles);
        let energy_usage = self.energy_usage();
        if energy_yield <= energy_usage {
            return Vec::new();
        }

        let genome = &mut genomes[self.genome_id];
        let mut k = energy_yield / energy_usage;
        self.surface_map
            .keys()
            .filter_map(|&tile_id| match tiles.owner(tile_id) {
                Owner::Empty => Some(ScoredTile::new(
                    tile_id,
                    genome.score(self.id, tiles, tile_id),
                    1,
                )),
                Owner::Cell(plant_id) if plant_id == self.id => None,
                Owner::Cell(_) => Some(ScoredTile::new(
                    tile_id,
                    genome.score(self.id, tiles, tile_id),
                    2,
                )),
            })
            .sorted_unstable()
            .filter(|tile| {
                if k >= tile.cost {
                    k -= tile.cost;
                    true
                } else {
                    false
                }
            })
            .map(|ranked_tile| ranked_tile.id())
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
