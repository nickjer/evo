use crate::neighbors::Neighbors;
use crate::neighbors::NEIGHBOR_COUNT;
use crate::tile_list::TileId;
use derive_more::IntoIterator;
use nohash::{IntMap, IntSet};
use std::collections::VecDeque;

#[derive(Debug, Clone, IntoIterator, Default)]
pub struct SimpleGraph {
    #[into_iterator(owned, ref, ref_mut)]
    node_map: IntMap<TileId, IntSet<TileId>>,
}

impl SimpleGraph {
    pub fn node_count(&self) -> usize {
        self.node_map.len()
    }

    pub fn nodes(&self) -> Vec<TileId> {
        self.node_map.keys().copied().collect()
    }

    pub fn neighbors(&self, node_id: TileId) -> &IntSet<TileId> {
        &self.node_map[&node_id]
    }

    pub fn add_node(&mut self, node_id: TileId, neighbor_ids: &Neighbors) -> Vec<TileId> {
        let mut links = IntSet::with_capacity_and_hasher(
            NEIGHBOR_COUNT,
            std::hash::BuildHasherDefault::default(),
        );
        let mut unlinked_neighbors = Vec::with_capacity(NEIGHBOR_COUNT);
        neighbor_ids.into_iter().for_each(|neighbor_id| {
            match self.node_map.get_mut(&neighbor_id) {
                Some(reverse_links) => {
                    links.insert(neighbor_id);
                    reverse_links.insert(node_id);
                }
                None => {
                    unlinked_neighbors.push(neighbor_id);
                }
            }
        });
        self.node_map.insert(node_id, links);
        unlinked_neighbors
    }

    pub fn remove_node(&mut self, node_id: TileId) -> IntSet<TileId> {
        let neighbor_ids = self.node_map[&node_id].clone();
        neighbor_ids.iter().for_each(|neighbor_id| {
            let connections = self.node_map.get_mut(neighbor_id).unwrap();
            connections.remove(&node_id);
        });
        self.node_map.remove(&node_id);
        neighbor_ids
    }
}

pub fn components(graph: &SimpleGraph, capacity: usize) -> Vec<Vec<TileId>> {
    let mut visited = fixedbitset::FixedBitSet::with_capacity(capacity);
    let mut components = Vec::new();
    let node_count = graph.node_count();
    for node_id in graph.nodes() {
        if !visited.contains(node_id.into()) {
            let mut component = Vec::with_capacity(node_count);
            let mut stack = Vec::with_capacity(node_count);
            stack.push(node_id);
            visited.insert(node_id.into());
            while let Some(node_id) = stack.pop() {
                component.push(node_id);
                for &neighbor_id in graph.neighbors(node_id) {
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

pub fn all_connected(graph: &SimpleGraph, node_ids: &[TileId], capacity: usize) -> bool {
    let (first, rest) = match node_ids.split_first() {
        Some((_first, [])) => return true,
        Some((&first, rest)) => (first, rest),
        None => return true,
    };
    let mut visited = fixedbitset::FixedBitSet::with_capacity(capacity);
    let mut stack = VecDeque::new();
    let mut search: IntSet<_> = rest.iter().copied().collect();

    stack.push_front(first);
    visited.insert(first.into());
    while let Some(node_id) = stack.pop_front() {
        for &neighbor_id in graph.neighbors(node_id) {
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
