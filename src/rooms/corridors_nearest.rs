//! Connect nearest rooms on the map with corridors
//!
use std::collections::HashSet;

use super::RoomBasedMap;

pub struct NearestCorridors;

impl Default for NearestCorridors {
    fn default() -> Self {
        Self::new()
    }
}

impl NearestCorridors {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate(&self, map: &RoomBasedMap) -> RoomBasedMap {
        let mut new_map = map.clone();
        let mut connected: HashSet<usize> = HashSet::new();
        for (i, room) in map.rooms.iter().enumerate() {
            let mut room_distance: Vec<(usize, f32)> = Vec::new();
            let room_center = room.center();
            for (j, other_room) in map.rooms.iter().enumerate() {
                if i != j && !connected.contains(&j) {
                    let other_center = other_room.center();
                    let distance = room_center.distance_to(&other_center);
                    room_distance.push((j, distance));
                }
            }

            if !room_distance.is_empty() {
                room_distance.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
                let dest_center = map.rooms[room_distance[0].0].center();
                new_map.add_corridor(room_center, dest_center);
                connected.insert(i);
            }
        }

        new_map
    }
}

/// ------------------------------------------------------------------------------------------------
/// Module unit tests
/// ------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::{geometry::Rect, layer::WalkableLayer};

    use super::*;

    #[test]
    fn no_corridors_on_borders() {
        let mut rooms = RoomBasedMap::new(10, 5);
        rooms.add_room(Rect::new(1, 1, 3, 3));
        rooms.add_room(Rect::new(6, 1, 3, 3));
        let corridors = NearestCorridors::default();
        let map = corridors.generate(&rooms);

        let map_str = "
        ##########
        #   ##   #
        #        #
        #   ##   #
        ##########
        ";
        let expected = WalkableLayer::from_string(map_str);

        println!("{}", &map.walkable_layer);

        assert_eq!(map.walkable_layer, expected);
    }
}
