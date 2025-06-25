use ereea::simulation::simulation::Simulation;

#[test]
fn test_simulation_initialization() {
    let sim = Simulation::new(123);
    let map = sim.map.read().unwrap();
    assert_eq!(map.seed, 123);
}
