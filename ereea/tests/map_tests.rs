use ereea::maps::{map::Map};

#[test]
fn test_map_generation_reproducibility() {
    let map1 = Map::new(20, 20, 42);
    let map2 = Map::new(20, 20, 42);
    
    assert_eq!(map1.seed, map2.seed);
    assert_eq!(map1.width, map2.width);
    assert_eq!(map1.height, map2.height);
}
