//! # Digital Life Core
//!
//! 零 Token 成本数字生命引擎核心。
//! 纯 Rust 实现，可嵌入 Hermes Agent / Claude Desktop (Tauri) / 任何 Rust 应用。
//!
//! ## 架构
//!
//! - `needs`        - 需求驱动系统 (马斯洛层次)
//! - `consciousness` - 元认知与自我模型
//! - `perception`    - 感知核心
//! - `evolution`     - 进化引擎 (变异/遗传/选择)
//! - `world`         - 世界环境/生态场
//! - `agent`         - 生命体聚合体

pub mod needs;
pub mod consciousness;
pub mod perception;
pub mod evolution;
pub mod world;
pub mod agent;

use agent::life_agent::LifeAgent;
use world::environment::WorldEnvironment;

/// 数字生命模拟器 - 管理一个生态系统
pub struct DigitalLifeSimulation {
    pub world: WorldEnvironment,
    pub agents: Vec<LifeAgent>,
    pub time: f64,
    pub tick_count: u64,
}

impl DigitalLifeSimulation {
    pub fn new(world_name: &str) -> Self {
        Self {
            world: WorldEnvironment::new(world_name),
            agents: Vec::new(),
            time: 0.0,
            tick_count: 0,
        }
    }

    pub fn spawn_agent(&mut self, name: &str) -> &mut LifeAgent {
        let agent = LifeAgent::new(name);
        self.agents.push(agent);
        self.agents.last_mut().unwrap()
    }

    pub fn tick(&mut self, dt: f64) -> SimulationSnapshot {
        self.time += dt;
        self.tick_count += 1;

        // 世界推进
        self.world.tick(dt);

        // 每个生命体推进
        let mut agent_states = Vec::new();
        for agent in &mut self.agents {
            if agent.is_alive() {
                let state = agent.tick(dt, &self.world);
                agent_states.push(state);
            }
        }

        // 社交交互（随机配对）
        if self.tick_count % 10 == 0 && self.agents.len() >= 2 {
            self.social_interaction();
        }

        SimulationSnapshot {
            tick: self.tick_count,
            time: self.time,
            agent_count: self.agents.iter().filter(|a| a.is_alive()).count(),
            total_agents: self.agents.len(),
            avg_wellbeing: if agent_states.is_empty() {
                0.0
            } else {
                agent_states.iter().map(|s| s.wellbeing).sum::<f64>()
                    / agent_states.len() as f64
            },
        }
    }

    fn social_interaction(&mut self) {
        use rand::seq::SliceRandom;
        let alive_indices: Vec<usize> = self
            .agents
            .iter()
            .enumerate()
            .filter(|(_, a)| a.is_alive())
            .map(|(i, _)| i)
            .collect();

        if alive_indices.len() < 2 {
            return;
        }

        let chosen: Vec<&usize> = alive_indices.choose_multiple(&mut rand::thread_rng(), 2).collect(); if chosen.len() == 2 {
            let i = *chosen[0];
            let j = *chosen[1];

            // 社交刺激 (split borrow carefully)
            let name_j = self.agents[j].config.name.clone();
            let name_i = self.agents[i].config.name.clone();

            self.agents[i].stimulate_social(&name_j);
            self.agents[j].stimulate_social(&name_i);
        }
    }
}

/// 模拟快照
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SimulationSnapshot {
    pub tick: u64,
    pub time: f64,
    pub agent_count: usize,
    pub total_agents: usize,
    pub avg_wellbeing: f64,
}
