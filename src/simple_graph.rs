use crate::grid::Grid;
use crate::square_grid::Neighbors;
use crate::tiles::TileId;
use derive_more::derive::{Constructor, IsVariant};
use fixedbitset::FixedBitSet;
use nohash::{IntMap, IntSet};
use std::collections::hash_map::Entry;
use std::collections::VecDeque;
use std::hash::BuildHasherDefault;

#[derive(Debug, Copy, Clone, IsVariant)]
enum Occupancy {
    Occupied(TileId),
    Unoccupied(TileId),
}

impl Occupancy {
    fn occupied(&self) -> Option<TileId> {
        match self {
            Occupancy::Occupied(occupied_id) => Some(*occupied_id),
            _ => None,
        }
    }

    fn unoccupied(&self) -> Option<TileId> {
        match self {
            Occupancy::Unoccupied(unoccupied_id) => Some(*unoccupied_id),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Constructor)]
struct Node {
    id: TileId,
    neighbors: Neighbors<Occupancy>,
}

impl Node {
    fn connect_to(&mut self, neighbor_id: TileId) {
        (&mut self.neighbors)
            .into_iter()
            .for_each(|neighbor| match *neighbor {
                Occupancy::Unoccupied(unoccupied_neighbor_id)
                    if unoccupied_neighbor_id == neighbor_id =>
                {
                    *neighbor = Occupancy::Occupied(neighbor_id);
                }
                _ => {}
            });
    }

    fn disconnect_from(&mut self, neighbor_id: TileId) {
        (&mut self.neighbors)
            .into_iter()
            .for_each(|neighbor| match *neighbor {
                Occupancy::Occupied(occupied_neighbor_id)
                    if occupied_neighbor_id == neighbor_id =>
                {
                    *neighbor = Occupancy::Unoccupied(neighbor_id);
                }
                _ => {}
            });
    }
}

#[derive(Debug, Clone, Default)]
struct Surface {
    unoccupied_map: IntMap<TileId, IntSet<TileId>>,
}

impl Surface {
    fn add_node(&mut self, node: &Node) {
        let capacity = 4;
        (&node.neighbors)
            .into_iter()
            .filter_map(Occupancy::unoccupied)
            .for_each(|neighbor_id| {
                self.insert_link(neighbor_id, node.id, capacity);
            });
        self.unoccupied_map.remove(&node.id);
    }

    fn remove_node(&mut self, node: &Node) {
        let capacity = 4;
        (&node.neighbors)
            .into_iter()
            .for_each(|&neighbor| match neighbor {
                Occupancy::Occupied(neighbor_id) => {
                    self.insert_link(node.id, neighbor_id, capacity);
                }
                Occupancy::Unoccupied(neighbor_id) => {
                    self.remove_link(neighbor_id, node.id);
                }
            });
    }

    fn unoccupied_neighbors(&self) -> Vec<TileId> {
        self.unoccupied_map.keys().copied().collect()
    }

    fn insert_link(&mut self, unoccupied_id: TileId, occupied_id: TileId, capacity: usize) {
        self.unoccupied_map
            .entry(unoccupied_id)
            .or_insert_with(|| {
                IntSet::with_capacity_and_hasher(capacity, BuildHasherDefault::default())
            })
            .insert(occupied_id);
    }

    fn remove_link(&mut self, unoccupied_id: TileId, occupied_id: TileId) {
        match self.unoccupied_map.entry(unoccupied_id) {
            Entry::Occupied(mut entry) => {
                let occupied_neighbors = entry.get_mut();
                occupied_neighbors.remove(&occupied_id);
                if occupied_neighbors.is_empty() {
                    entry.remove();
                }
            }
            Entry::Vacant(_) => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SimpleGraph {
    node_map: IntMap<TileId, Node>,
    surface: Surface,
}

impl SimpleGraph {
    pub fn node_count(&self) -> usize {
        self.node_map.len()
    }

    pub fn nodes(&self) -> Vec<TileId> {
        self.node_map.keys().copied().collect()
    }

    fn neighbor_iter(&self, node_id: TileId) -> impl Iterator<Item = TileId> + '_ {
        (&self.node_map[&node_id].neighbors)
            .into_iter()
            .filter_map(Occupancy::occupied)
    }

    pub fn add_node(&mut self, node_id: TileId, grid: &Grid) {
        let neighbors =
            grid.neighbors(node_id)
                .map(|neighbor_id| match self.node_map.entry(neighbor_id) {
                    Entry::Occupied(mut occupied_neighbor_node) => {
                        occupied_neighbor_node.get_mut().connect_to(node_id);
                        Occupancy::Occupied(neighbor_id)
                    }
                    Entry::Vacant(_) => Occupancy::Unoccupied(neighbor_id),
                });
        let node = Node::new(node_id, neighbors);
        self.surface.add_node(&node);
        self.node_map.insert(node_id, node); // Add this node to the graph
    }

    pub fn remove_node(&mut self, node_id: TileId) -> Vec<TileId> {
        let node = self.node_map.remove(&node_id).unwrap();

        // Remove the reverse links pointing back to this node
        let occupied_neighbor_ids = (&node.neighbors)
            .into_iter()
            .filter_map(Occupancy::occupied)
            .collect::<Vec<_>>();
        occupied_neighbor_ids
            .iter()
            .for_each(|occupied_neighbor_id| {
                self.node_map
                    .get_mut(occupied_neighbor_id)
                    .unwrap()
                    .disconnect_from(node_id);
            });

        self.surface.remove_node(&node);

        occupied_neighbor_ids
    }

    pub fn all_unoccupied_neighbors(&self) -> Vec<TileId> {
        self.surface.unoccupied_neighbors()
    }

    pub fn unoccupied_neighbors_iter(
        &self,
    ) -> impl Iterator<Item = (TileId, impl Iterator<Item = TileId> + '_)> + '_ {
        self.surface
            .unoccupied_map
            .iter()
            .map(|(&node_id, occupied_neighbors)| (node_id, occupied_neighbors.iter().copied()))
    }
}

pub fn components(graph: &SimpleGraph, visited: &mut FixedBitSet) -> Vec<Vec<TileId>> {
    let mut components = Vec::new();
    let node_count = graph.node_count();

    let mut component = Vec::with_capacity(node_count);
    visited
        .ones()
        .for_each(|node_id| component.push(node_id.into()));
    components.push(component);

    for node_id in graph.nodes() {
        if !visited.contains(node_id.into()) {
            let mut component = Vec::with_capacity(node_count);
            let mut stack = Vec::with_capacity(node_count);
            stack.push(node_id);
            visited.insert(node_id.into());
            while let Some(node_id) = stack.pop() {
                component.push(node_id);
                for neighbor_id in graph.neighbor_iter(node_id) {
                    if !visited.put(neighbor_id.into()) {
                        stack.push(neighbor_id);
                    }
                }
            }
            components.push(component);
        }
    }
    components
}

pub fn all_connected(graph: &SimpleGraph, node_ids: &[TileId], visited: &mut FixedBitSet) -> bool {
    let (first, rest) = match node_ids.split_first() {
        Some((_first, [])) => return true,
        Some((&first, rest)) => (first, rest),
        None => return true,
    };
    let mut stack = VecDeque::new();
    let mut search: IntSet<_> = rest.iter().copied().collect();

    stack.push_front(first);
    visited.insert(first.into());
    while let Some(node_id) = stack.pop_front() {
        for neighbor_id in graph.neighbor_iter(node_id) {
            if !visited.put(neighbor_id.into()) {
                if search.remove(&neighbor_id) && search.is_empty() {
                    return true;
                }
                stack.push_back(neighbor_id);
            }
        }
    }
    false
}
