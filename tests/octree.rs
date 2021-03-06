extern crate spatial;

use spatial::octree::{Octree, Index, Volume};

#[derive(Clone)]
struct Object {
    x: f32,
    y: f32,
    z: f32
}

impl Object {
    pub fn new(x: f32, y: f32, z: f32) -> Object {
        Object {
            x: x,
            y: y,
            z: z
        }
    }
}

impl Index<f32> for Object {
    fn octree_index(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}


#[test]
fn octree_insert_query() {
    let vol = Volume::new([0.0, 0.0, 0.0], [1.0, 1.0, 1.0]);
    let mut tree = Octree::new(vol);
    
    assert_eq!(tree.insert(Object::new(0.25, 0.25, 0.25)), true);
    assert_eq!(tree.insert(Object::new(0.75, 0.25, 0.25)), true);
    assert_eq!(tree.insert(Object::new(0.25, 0.75, 0.25)), true);
    assert_eq!(tree.insert(Object::new(0.75, 0.75, 0.25)), true);
    assert_eq!(tree.insert(Object::new(0.25, 0.25, 0.75)), true);
    assert_eq!(tree.insert(Object::new(0.75, 0.25, 0.75)), true);
    assert_eq!(tree.insert(Object::new(0.25, 0.75, 0.75)), true);
    assert_eq!(tree.insert(Object::new(0.75, 0.75, 0.75)), true);
    
    assert_eq!(tree.get_in_volume(&Volume::new([0.0, 0.0, 0.0], [0.5, 0.5, 0.5])).len(), 1);
    assert_eq!(tree.get_in_volume(&Volume::new([0.5, 0.0, 0.0], [1.0, 0.5, 0.5])).len(), 1);
    assert_eq!(tree.get_in_volume(&Volume::new([0.0, 0.5, 0.0], [0.5, 1.0, 0.5])).len(), 1);
    assert_eq!(tree.get_in_volume(&Volume::new([0.5, 0.5, 0.0], [1.0, 1.0, 0.5])).len(), 1);
    assert_eq!(tree.get_in_volume(&Volume::new([0.0, 0.0, 0.5], [0.5, 0.5, 1.0])).len(), 1);
    assert_eq!(tree.get_in_volume(&Volume::new([0.5, 0.0, 0.5], [1.0, 0.5, 1.0])).len(), 1);
    assert_eq!(tree.get_in_volume(&Volume::new([0.0, 0.5, 0.5], [0.5, 1.0, 1.0])).len(), 1);
    assert_eq!(tree.get_in_volume(&Volume::new([0.5, 0.5, 0.5], [1.0, 1.0, 1.0])).len(), 1);
    
    assert_eq!(tree.get_in_radius([0.5, 0.5, 0.5], 0f32).len(), 0);
    assert_eq!(tree.get_in_radius([0.25, 0.25, 0.25], 0.25).len(), 1);
    assert_eq!(tree.get_in_radius([0.5, 0.5, 0.5], 0.5).len(), 8);
    
    assert_eq!(tree.len(), 8);
}
