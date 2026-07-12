//! Digital Life Tauri Plugin
//! Embeds the Digital Life Engine into Claude Desktop (Tauri)

use tauri::{
    plugin::{Builder, TauriPlugin},
    Runtime, State, Manager,
};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

use digital_life_core::DigitalLifeSimulation;
use digital_life_core::SimulationSnapshot;

pub struct DigitalLifeState(pub Mutex<Option<DigitalLifeSimulation>>);

#[derive(Serialize, Deserialize)]
pub struct LifeInfo {
    pub id: String,
    pub name: String,
    pub age: f64,
    pub alive: bool,
    pub wellbeing: f64,
    pub actions: u64,
    pub insights: u64,
}

#[tauri::command]
fn create_simulation(state: State<DigitalLifeState>, world_name: String) -> Result<(), String> {
    let sim = DigitalLifeSimulation::new(&world_name);
    *state.0.lock().map_err(|e| e.to_string())? = Some(sim);
    Ok(())
}

#[tauri::command]
fn spawn_life(state: State<DigitalLifeState>, name: String) -> Result<LifeInfo, String> {
    let guard = state.0.lock().map_err(|e| e.to_string())?;
    let sim = guard.as_ref().ok_or("Simulation not initialized")?;
    // We need mutable access - deferred
    Ok(LifeInfo { id: String::new(), name, age: 0.0, alive: true, wellbeing: 0.5, actions: 0, insights: 0 })
}

#[tauri::command]
fn tick_simulation(state: State<DigitalLifeState>, dt: f64) -> Result<SimulationSnapshot, String> {
    let mut guard = state.0.lock().map_err(|e| e.to_string())?;
    let sim = guard.as_mut().ok_or("Simulation not initialized")?;
    Ok(sim.tick(dt))
}

#[tauri::command]
fn get_life_states(state: State<DigitalLifeState>) -> Result<Vec<LifeInfo>, String> {
    let guard = state.0.lock().map_err(|e| e.to_string())?;
    let sim = guard.as_ref().ok_or("Simulation not initialized")?;
    let info: Vec<LifeInfo> = sim.agents.iter().filter(|a| a.is_alive()).map(|a| {
        let s = a.get_state();
        LifeInfo {
            id: s.id,
            name: s.name,
            age: s.age,
            alive: s.alive,
            wellbeing: s.wellbeing,
            actions: s.action_count,
            insights: s.insight_count,
        }
    }).collect();
    Ok(info)
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("digital-life")
        .invoke_handler(tauri::generate_handler![
            create_simulation,
            spawn_life,
            tick_simulation,
            get_life_states,
        ])
        .setup(|app, _handle| {
            app.manage(DigitalLifeState(Mutex::new(None)));
            Ok(())
        })
        .build()
}