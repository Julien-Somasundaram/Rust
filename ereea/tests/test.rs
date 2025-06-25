use ereea::map::{Map, Ressources, Cellule};
use ereea::robot::{Robot, TypeRobot, Module};

#[test]
fn test_base_la_plus_proche() {
    let map = Map::new(10, 10, 42);
    let closest = map.base_la_plus_proche(0, 0);
    assert!(map.bases.contains(&closest));
}

#[test]
fn test_case_non_exploree() {
    let mut map = Map::new(10, 10, 42);
    map.explorée[1][1] = true;
    let result = map.case_non_exploree_la_plus_proche(1, 1);
    assert!(result.is_some());
}

#[test]
fn test_robot_capacity_limits() {
    let robot = Robot {
        id: 0,
        x: 0,
        y: 0,
        kind: TypeRobot::Recolteur,
        sac: Ressources { energie: 3, minerai: 2, science: 0 },
        capacite: 5,
        retour_base: false,
        modules: vec![Module::Foreuse],
        last_pos: None,
    };
    let total = robot.sac.energie + robot.sac.minerai + robot.sac.science;
    assert_eq!(total, robot.capacite);
    assert!(!robot.retour_base);
}

#[test]
fn test_depot_ressources() {
    let mut map = Map::new(10, 10, 123);
    let base = map.bases[0];
    let robot = map.robots.get_mut(0).unwrap();
    robot.x = base.0;
    robot.y = base.1;
    robot.retour_base = true;
    robot.sac.energie = 2;
    robot.sac.minerai = 1;
    robot.sac.science = 1;

    let max_ticks = 10;
    let mut depot_effectue = false;

    for _ in 0..max_ticks {
        map.tick();
        let robot = &map.robots[0];
        if robot.sac.energie == 0 && robot.sac.minerai == 0 && robot.sac.science == 0 {
            depot_effectue = true;
            break;
        }
    }

    assert!(depot_effectue, "Les ressources n'ont pas été déposées.");
    assert!(!map.robots[0].retour_base, "Le robot devrait être reparti en exploration.");
}

#[test]
fn test_deplacement_robot() {
    let mut map = Map::new(10, 10, 42);
    let robot = map.robots.get(0).unwrap();
    let (old_x, old_y) = (robot.x, robot.y);

    map.tick();

    let robot = map.robots.get(0).unwrap();
    let (new_x, new_y) = (robot.x, robot.y);
    assert!(
        old_x != new_x || old_y != new_y,
        "Le robot n'a pas bougé"
    );
}

#[test]
fn test_collecte_energie() {
    let mut map = Map::new(5, 5, 0);
    let robot = map.robots.get_mut(0).unwrap();
    robot.modules = vec![Module::Foreuse];
    robot.x = 1;
    robot.y = 1;
    map.grille[1][1] = Cellule::Energie;

    let max_ticks = 10;
    let mut collecte_effectuee = false;

    for _ in 0..max_ticks {
        map.tick();
        let robot = &map.robots[0];
        if robot.sac.energie > 0 {
            collecte_effectuee = true;
            break;
        }
    }

    assert!(collecte_effectuee, "Le robot n'a pas collecté d'énergie.");
    assert_eq!(map.grille[1][1], Cellule::Vide);
}


#[test]
fn test_collecte_sans_module() {
    let mut map = Map::new(5, 5, 0);
    {
        let robot = map.robots.get_mut(0).unwrap();
        robot.modules = vec![]; // Aucun module
        robot.x = 2;
        robot.y = 2;
        map.grille[2][2] = Cellule::Energie;
    }

    for _ in 0..5 {
        map.tick();
    }

    let robot = &map.robots[0];
    assert_eq!(robot.sac.energie, 0);
    assert_eq!(map.grille[2][2], Cellule::Energie);
}

#[test]
fn test_retour_base_si_plein() {
    let mut map = Map::new(5, 5, 0);
    {
        let robot = map.robots.get_mut(0).unwrap();
        robot.modules = vec![Module::Foreuse];
        robot.x = 3;
        robot.y = 3;
        robot.capacite = 1;
        map.grille[3][3] = Cellule::Energie;
    }

    for _ in 0..5 {
        map.tick();
    }

    let robot = &map.robots[0];
    assert!(robot.retour_base, "Le robot devrait retourner à la base");
}
