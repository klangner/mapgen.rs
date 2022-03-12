//! Connect nearest rooms on the map with corridors
//! 
use rand::prelude::StdRng;
use crate::BuilderData;
use crate::MapFilter;
use crate::Map;
use std::collections::HashSet;
use std::marker::PhantomData;


pub struct NearestCorridors<D: BuilderData> {
    phantom: PhantomData<D>,
}

impl<D: BuilderData> MapFilter<D> for NearestCorridors<D> {
    fn modify_map(&self, _: &mut StdRng, map: &Map<D>)  -> Map<D> {
        self.corridors(map)
    }
}

impl<D: BuilderData> NearestCorridors<D> {

    pub fn new() -> Box<NearestCorridors<D>> {
        Box::new(NearestCorridors {
            phantom: PhantomData,
        })
    }

    fn corridors(&self, map: &Map<D>) -> Map<D> {
        let mut new_map = map.clone();

        let mut connected : HashSet<usize> = HashSet::new();
        for (i,room) in map.rooms.iter().enumerate() {
            let mut room_distance : Vec<(usize, f32)> = Vec::new();
            let room_center = room.center();
            for (j,other_room) in new_map.rooms.iter().enumerate() {
                if i != j && !connected.contains(&j) {
                    let other_center = other_room.center();
                    let distance = room_center.distance_to(&other_center);
                    room_distance.push((j, distance));
                }
            }

            if !room_distance.is_empty() {
                room_distance.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap() );
                let dest_center = new_map.rooms[room_distance[0].0].center();
                new_map.add_corridor(room_center, dest_center);
                connected.insert(i);
            }
        }
        new_map
    }
}