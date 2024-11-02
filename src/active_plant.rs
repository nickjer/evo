use crate::genomes::GenomeId;
use crate::grid::Grid;
use crate::plants::PlantId;
use crate::simple_graph::{all_connected, components, SimpleGraph};
use crate::tiles::TileId;
use fixedbitset::FixedBitSet;
use getset::CopyGetters;
use nohash::IntSet;

#[derive(Debug, Clone, CopyGetters, Default)]
pub struct ActivePlant {
    id: PlantId,
    #[get_copy = "pub"]
    genome_id: GenomeId,
    cells: SimpleGraph,
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
        self.cells.add_node(tile_id, grid.neighbors(tile_id));
        self.energy_yield(grid)
    }

    pub fn abandon(&mut self, tile_id: TileId, grid: &Grid) -> Vec<TileId> {
        let neighboring_cells: Vec<_> = self.remove_cell(tile_id).into_iter().collect();
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
                self.remove_cell(dead_tile_id);
            });
            return dead_cells;
        }

        Vec::new()
    }

    pub fn energy_points(&self, grid: &Grid) -> usize {
        let energy_usage = self.energy_usage();
        if energy_usage == 0 {
            return 0;
        }

        let energy_yield = self.energy_yield(grid);
        energy_yield / energy_usage
    }

    pub fn available_tiles(&self) -> Vec<TileId> {
        self.cells.all_unoccupied_neighbors()
    }

    fn energy_usage(&self) -> usize {
        self.cells.node_count()
    }

    fn energy_yield(&self, grid: &Grid) -> usize {
        self.cells
            .unoccupied_neighbors_iter()
            .map(|(unoccupied_node_id, occupied_id_iter)| {
                if !grid.is_empty(unoccupied_node_id) {
                    return 0;
                }

                occupied_id_iter
                    .map(|occupied_id| {
                        let (_plant_id, cell_kind) = grid.entity(occupied_id).unwrap_cell();
                        cell_kind.yield_per_empty_tile()
                    })
                    .sum()
            })
            .sum()
    }

    fn remove_cell(&mut self, tile_id: TileId) -> IntSet<TileId> {
        self.cells.remove_node(tile_id)
    }
}
