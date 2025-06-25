mod simulation;
mod maps;
mod robots;
mod UI;

use simulation::simulation::Simulation;

fn main() {
    let mut simulation = Simulation::new(4);
    simulation.run();
}