//! 数字生命体 (Life Agent)
//! 整合需求系统、意识核心、感知引擎、进化基因组
//! 零 Token 成本 — 完全本地运行

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::needs::{NeedsSystem, NeedType};
use crate::consciousness::ConsciousnessCore;
use crate::perception::{ActionEngine, ActionType, Action};
use crate::world::environment::WorldEnvironment;
use crate::evolution::engine::Genome;

/// 生命体配置
#[derive(Debug, Clone)]
pub struct LifeConfig {
    pub name: String,
    pub tick_interval: f64,
}

impl Default for LifeConfig {
    fn default() -> Self {
        Self {
            name: "Unnamed".to_string(),
            tick_interval: 0.5,
        }
    }
}

/// 生命体运行状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentState {
    pub id: String,
    pub name: String,
    pub age: f64,
    pub alive: bool,
    pub wellbeing: f64,
    pub action_count: u64,
    pub insight_count: u64,
}

/// 数字生命体
pub struct LifeAgent {
    pub id: String,
    pub config: LifeConfig,
    pub needs: NeedsSystem,
    pub consciousness: ConsciousnessCore,
    pub genome: Genome,

    // 运行时
    pub age: f64,
    pub alive: bool,
    action_count: u64,
}

impl LifeAgent {
    pub fn new(name: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string()[..12].to_string(),
            config: LifeConfig {
                name: name.to_string(),
                ..Default::default()
            },
            needs: NeedsSystem::new(),
            consciousness: ConsciousnessCore::new(name),
            genome: Genome::default(),
            age: 0.0,
            alive: true,
            action_count: 0,
        }
    }

    /// 一次完整的生命 Tick
    pub fn tick(&mut self, dt: f64, _world: &WorldEnvironment) -> AgentState {
        if !self.alive {
            return self.get_state();
        }

        self.age += dt;

        // 1. 需求推进
        self.needs.tick(dt);

        // 2. 元认知评估
        let meta = self.consciousness.assess(&self.needs);

        // 3. 形成意图
        let intention = self.consciousness.form_intention(&self.needs);

        // 4. 解析为行动
        let action = ActionEngine::resolve(&intention, meta.wellbeing);

        // 5. 执行行动 (应用效果)
        self.apply_action(&action);

        // 6. 概率内省
        self.consciousness.maybe_introspect(&meta, &intention);

        self.action_count += 1;

        // 7. 生存检查
        if self.needs.wellbeing() <= 0.05 {
            self.alive = false;
        }

        self.get_state()
    }

    /// 应用行动效果到需求
    fn apply_action(&mut self, action: &Action) {
        match action.action_type {
            ActionType::Explore => { self.needs.satisfy(NeedType::Curiosity, 0.2); }
            ActionType::Rest => { self.needs.satisfy(NeedType::Survival, 0.3); }
            ActionType::Socialize => { self.needs.satisfy(NeedType::Social, 0.25); }
            ActionType::Learn => { self.needs.satisfy(NeedType::Competence, 0.2); }
            ActionType::Create => { self.needs.satisfy(NeedType::Meaning, 0.25); }
            ActionType::SeekResource => { self.needs.satisfy(NeedType::Survival, 0.15); }
            ActionType::Assert => { self.needs.satisfy(NeedType::Autonomy, 0.2); }
            ActionType::None => {}
        }
    }

    /// 接收社交刺激
    pub fn stimulate_social(&mut self, other_name: &str) {
        let _content = format!("encountered {}", other_name);
        // 社交互动部分满足社交需求
        self.needs.satisfy(NeedType::Social, 0.05);
    }

    pub fn is_alive(&self) -> bool {
        self.alive
    }

    /// 获取当前状态快照
    pub fn get_state(&self) -> AgentState {
        AgentState {
            id: self.id.clone(),
            name: self.config.name.clone(),
            age: (self.age * 10.0).round() / 10.0,
            alive: self.alive,
            wellbeing: (self.needs.wellbeing() * 1000.0).round() / 1000.0,
            action_count: self.action_count,
            insight_count: self.consciousness.insight_count,
        }
    }

    /// 详细摘要
    pub fn summary(&self) -> String {
        let state = self.get_state();
        let bar_count = (state.wellbeing * 20.0) as usize;
        let bar = "#".repeat(bar_count) + &".".repeat(20 - bar_count);
        format!(
            "=== {} ({}) {} ===\n  Age: {:.1}s  WB: {:.3} {}\n  Actions: {}  Insights: {}\n  Genome: {}",
            state.name,
            &state.id[..8],
            if state.alive { "ALIVE" } else { "GONE" },
            state.age,
            state.wellbeing,
            bar,
            state.action_count,
            state.insight_count,
            self.genome.summary()
        )
    }
}
