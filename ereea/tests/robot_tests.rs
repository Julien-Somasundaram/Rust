use ereea::robots::explorer::Explorer;
use ereea::robots::robot::Robot;
use ereea::maps::map::Map;

#[test]
fn test_robot_creation_and_movement() {
    let mut map = Map::new(10, 10, 1);
    let mut robot = Explorer::new(2, 3, 42);

    // Déplace le robot dans la map (supposons un mouvement simple ici)
    robot.move_to(4, 5, &mut map);

    let (x, y) = robot.get_position();
    assert_eq!(x, 4);
    assert_eq!(y, 5);
}

