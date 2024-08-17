use crate::genomes::GenomeId;
use crate::grid::Grid;
use crate::plants::PlantId;
use crate::simple_graph::{all_connected, components, SimpleGraph};
use crate::tiles::TileId;
use fixedbitset::FixedBitSet;
use getset::CopyGetters;
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

    pub fn occupy(&mut self, tile_id: TileId, grid: &Grid) -> usize {
        let unlinked_neighbor_ids = self.cells.add_node(tile_id, grid.neighbors(tile_id));
        self.surface_map.remove(&tile_id);
        unlinked_neighbor_ids.into_iter().for_each(|neighbor_id| {
            self.surface_map
                .entry(neighbor_id)
                .and_modify(|count| *count = count.checked_add(1).unwrap())
                .or_insert(1);
        });
        self.energy_yield(grid)
    }

    pub fn abandon(&mut self, tile_id: TileId, grid: &Grid) -> Vec<TileId> {
        let neighboring_cells: Vec<_> = self.remove_cell(tile_id, grid).into_iter().collect();
        let mut visited = FixedBitSet::with_capacity(grid.size());

        // Make sure all neighboring cells are connected to each other
        if !all_connected(&self.cells, &neighboring_cells, &mut visited) {
            // Visited now contains a full component since it was not all connected
            let majority = self.cells.node_count() / 2;
            let dead_cells: Vec<TileId> = if visited.count_ones(..) > majority {
                self.cells
                    .nodes()
                    .into_iter()
                    .filter(|&tile_id| !visited.contains(tile_id.into()))
                    .collect()
            } else {
                let mut components = components(&self.cells, &mut visited);
                components.sort_by_key(|component| component.len());
                components
                    .split_last()
                    .unwrap()
                    .1
                    .iter()
                    .flat_map(|component| component.iter())
                    .copied()
                    .collect()
            };
            dead_cells.iter().for_each(|&dead_tile_id| {
                self.remove_cell(dead_tile_id, grid);
            });
            return dead_cells;
        }

        Vec::new()
    }

    pub fn points(&self, grid: &Grid) -> usize {
        let energy_usage = self.energy_usage();
        if energy_usage == 0 {
            return 0;
        }

        let energy_yield = self.energy_yield(grid);
        energy_yield / energy_usage
    }

    pub fn available_tiles(&self) -> IntSet<TileId> {
        self.surface_map.keys().copied().collect()
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
                        *count = count.checked_sub(1).unwrap();
                    }
                }
            });
        neighboring_cells
    }
}
