//! 需求驱动系统 — 零 Token 成本
//! 基于马斯洛需求层次 + 内在动机理论

use serde::{Deserialize, Serialize};

/// 需求类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NeedType {
    Survival,   // 生存/安全
    Curiosity,  // 好奇心/探索
    Social,     // 社交/归属
    Competence, // 能力/成就
    Autonomy,   // 自主/自由意志
    Meaning,    // 意义/自我实现
}

impl NeedType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Survival => "survival",
            Self::Curiosity => "curiosity",
            Self::Social => "social",
            Self::Competence => "competence",
            Self::Autonomy => "autonomy",
            Self::Meaning => "meaning",
        }
    }
}

/// 单个需求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Need {
    pub need_type: NeedType,
    pub value: f64,        // 0.0 (满足) ~ 1.0 (极度渴望)
    pub decay_rate: f64,   // 每秒自然衰减率
    pub weight: f64,       // 对幸福感的权重
    threshold_action: f64, // 超过此值触发行动
    threshold_critical: f64, // 超过此值进入紧急状态
}

impl Need {
    pub fn new(need_type: NeedType, decay_rate: f64, weight: f64) -> Self {
        Self {
            need_type,
            value: 0.5,
            decay_rate,
            weight,
            threshold_action: 0.7,
            threshold_critical: 0.9,
        }
    }

    /// 随时间推移需求自然上升
    pub fn tick(&mut self, dt: f64) {
        self.value = (self.value + self.decay_rate * dt).min(1.0);
    }

    /// 满足需求，返回实际减少量
    pub fn satisfy(&mut self, amount: f64) -> f64 {
        let before = self.value;
        self.value = (self.value - amount).max(0.0);
        before - self.value
    }

    pub fn is_actionable(&self) -> bool {
        self.value >= self.threshold_action
    }

    pub fn is_critical(&self) -> bool {
        self.value >= self.threshold_critical
    }

    pub fn label(&self) -> &str {
        if self.is_critical() { "CRITICAL" }
        else if self.is_actionable() { "URGENT" }
        else if self.value > 0.4 { "moderate" }
        else { "satisfied" }
    }
}

/// 需求系统
pub struct NeedsSystem {
    pub needs: Vec<Need>,
    last_tick_time: f64,
}

impl Default for NeedsSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl NeedsSystem {
    pub fn new() -> Self {
        let mut needs = Vec::new();
        // 默认需求配置 (decay_rate, weight)
        let config: [(NeedType, f64, f64); 6] = [
            (NeedType::Survival,   0.005, 3.0),
            (NeedType::Curiosity,  0.020, 1.5),
            (NeedType::Social,     0.012, 1.5),
            (NeedType::Competence, 0.008, 1.0),
            (NeedType::Autonomy,   0.006, 1.0),
            (NeedType::Meaning,    0.003, 0.5),
        ];
        for (nt, decay, weight) in config {
            needs.push(Need::new(nt, decay, weight));
        }
        Self {
            needs,
            last_tick_time: 0.0,
        }
    }

    /// 推进时间，返回各需求变化量
    pub fn tick(&mut self, dt: f64) {
        self.last_tick_time += dt;
        for need in &mut self.needs {
            need.tick(dt);
        }
    }

    /// 满足特定需求
    pub fn satisfy(&mut self, need_type: NeedType, amount: f64) -> f64 {
        if let Some(need) = self.needs.iter_mut().find(|n| n.need_type == need_type) {
            need.satisfy(amount)
        } else {
            0.0
        }
    }

    /// 获取当前最迫切的需求
    pub fn dominant_need(&self) -> Option<&Need> {
        self.needs
            .iter()
            .filter(|n| n.is_actionable())
            .max_by(|a, b| {
                (a.value * a.weight)
                    .partial_cmp(&(b.value * b.weight))
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    }

    /// 综合幸福感 (0~1)
    pub fn wellbeing(&self) -> f64 {
        let total_weight: f64 = self.needs.iter().map(|n| n.weight).sum();
        if total_weight == 0.0 {
            return 1.0;
        }
        let weighted: f64 = self.needs.iter().map(|n| (1.0 - n.value) * n.weight).sum();
        weighted / total_weight
    }

    /// 内稳态快照
    pub fn homeostasis(&self) -> Vec<(String, f64)> {
        let mut states: Vec<_> = self
            .needs
            .iter()
            .map(|n| (n.need_type.as_str().to_string(), n.value))
            .collect();
        states.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        states
    }

    /// 需求摘要
    pub fn summary(&self) -> String {
        let mut lines: Vec<String> = self
            .needs
            .iter()
            .map(|n| {
                let bar_count = (n.value * 10.0) as usize;
                let bar = "#".repeat(bar_count) + &".".repeat(10 - bar_count);
                format!("{:12} |{}| {:.2} {}", n.need_type.as_str(), bar, n.value, n.label())
            })
            .collect();
        lines.push(format!("Wellbeing: {:.3}", self.wellbeing()));
        lines.join("\n")
    }
}
