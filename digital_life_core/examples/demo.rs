fn main() {
    use digital_life_core::DigitalLifeSimulation;
    use digital_life_core::evolution::engine::Genome;

    let mut sim = DigitalLifeSimulation::new("Test World");

    sim.spawn_agent("Explorer").genome = Genome {
        curiosity_drive: 0.9, risk_tolerance: 0.7, social_drive: 0.3,
        ..Genome::default()
    };
    sim.spawn_agent("Socialite").genome = Genome {
        curiosity_drive: 0.3, social_drive: 0.9,
        ..Genome::default()
    };
    sim.spawn_agent("Survivor").genome = Genome {
        curiosity_drive: 0.3, energy_efficiency: 0.8, regeneration_rate: 0.7,
        ..Genome::default()
    };

    println!("=== Digital Life Engine (Rust) ===");
    println!("Created 3 digital lives\n");

    for tick in 0..30 {
        let snap = sim.tick(0.5);
        if tick % 10 == 0 || tick == 29 {
            println!(
                "Tick {:3} | Time: {:5.1}s | Alive: {}/{} | Avg WB: {:.3}",
                snap.tick, snap.time, snap.agent_count, snap.total_agents, snap.avg_wellbeing
            );
        }
    }

    println!("\n=== Final Report ===");
    for agent in &sim.agents {
        if agent.is_alive() {
            println!("{}", agent.summary());
        }
    }

    println!("\n=== Done. Zero tokens spent. ===");
}
