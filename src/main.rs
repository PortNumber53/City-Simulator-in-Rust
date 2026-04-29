// City Simulator - Entry Point
// Native application entry point for testing

use city_simulator::CitySimulator;

fn main() {
    println!("City Simulator - Starting up...");
    
    // Create the simulation
    let mut simulator = CitySimulator::new();
    
    println!("Simulation created");
    println!("Initial time: {}", simulator.time());
    println!("Initial budget: {}", simulator.budget().total_funds);
    
    // Run a few frames to test
    for frame in 0..10 {
        simulator.update(0.016); // 60 FPS
        if frame % 5 == 0 {
            println!("Frame {} - Time: {:.3}s", frame, simulator.time());
        }
    }
    
    println!("\nSimulation test complete!");
    println!("Final time: {:.3}s", simulator.time());
    println!("Final budget: ${:.2}", simulator.budget().total_funds);
}
