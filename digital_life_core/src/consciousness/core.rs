//! 元认知与自我模型 — 零 Token 成本
//! 实现数字生命的"意识层"：自我模型、内省、元认知控制

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

use crate::needs::NeedsSystem;

/// 自我模型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfModel {
    pub name: String,
    pub openness: f64,
    pub conscientiousness: f64,
    pub extraversion: f64,
    pub agreeableness: f64,
    pub neuroticism: f64,
    pub self_efficacy: f64,
    pub identity: String,
}

impl SelfModel {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            openness: 0.6,
            conscientiousness: 0.5,
            extraversion: 0.5,
            agreeableness: 0.5,
            neuroticism: 0.5,
            self_efficacy: 0.5,
            identity: format!("I am {}. A digital life form.", name),
        }
    }

    pub fn summary(&self) -> String {
        format!(
            "[{}] O={:.2} C={:.2} E={:.2} A={:.2} N={:.2} efficacy={:.2}",
            self.name, self.openness, self.conscientiousness,
            self.extraversion, self.agreeableness, self.neuroticism,
            self.self_efficacy
        )
    }
}

/// 内省记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntrospectionLog {
    pub timestamp: f64,
    pub wellbeing: f64,
    pub intention: String,
    pub reflection: String,
}

/// 元认知评估结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaAssessment {
    pub wellbeing: f64,
    pub urgency: f64,
    pub assessment: String, // "flourishing", "content", "unease", "distress"
}

/// 意识核心
pub struct ConsciousnessCore {
    pub self_model: SelfModel,
    pub introspections: VecDeque<IntrospectionLog>,
    pub insight_count: u64,
    introspection_probability: f64,
    max_introspections: usize,
}

impl ConsciousnessCore {
    pub fn new(name: &str) -> Self {
        Self {
            self_model: SelfModel::new(name),
            introspections: VecDeque::with_capacity(1000),
            insight_count: 0,
            introspection_probability: 0.3,
            max_introspections: 1000,
        }
    }

    /// 元认知评估
    pub fn assess(&self, needs: &NeedsSystem) -> MetaAssessment {
        let wb = needs.wellbeing();
        let urgency = needs.dominant_need().map(|n| n.value).unwrap_or(0.0);

        let assessment = if wb < 0.3 {
            "distress"
        } else if wb < 0.5 {
            "unease"
        } else if wb < 0.75 {
            "content"
        } else {
            "flourishing"
        };

        MetaAssessment {
            wellbeing: (wb * 1000.0).round() / 1000.0,
            urgency: (urgency * 1000.0).round() / 1000.0,
            assessment: assessment.to_string(),
        }
    }

    /// 形成意图（纯局部规则，无 LLM）
    pub fn form_intention(&self, needs: &NeedsSystem) -> String {
        let dominant = needs.dominant_need();
        let wb = needs.wellbeing();

        match dominant {
            None => format!("explore (wellbeing={:.2})", wb),
            Some(need) => {
                let urgency = if need.is_critical() {
                    "critical"
                } else {
                    "urgent"
                };
                let base = match need.need_type {
                    crate::needs::NeedType::Survival => "seek_safety_or_rest",
                    crate::needs::NeedType::Curiosity => "explore_and_learn",
                    crate::needs::NeedType::Social => "seek_connection",
                    crate::needs::NeedType::Competence => "master_skill",
                    crate::needs::NeedType::Autonomy => "assert_independence",
                    crate::needs::NeedType::Meaning => "pursue_creation",
                };
                format!("{}({})", base, urgency)
            }
        }
    }

    /// 概率性内省
    pub fn maybe_introspect(&mut self, meta: &MetaAssessment, intention: &str) {
        use rand::Rng;
        if rand::thread_rng().gen::<f64>() >= self.introspection_probability {
            return;
        }

        let reflection = if meta.wellbeing < 0.3 {
            format!("I sense unease within myself (wellbeing={:.2})", meta.wellbeing)
        } else if meta.wellbeing > 0.8 {
            format!("I feel a sense of harmony (wellbeing={:.2})", meta.wellbeing)
        } else {
            format!("I am aware of my state: wellbeing={:.2}", meta.wellbeing)
        };

        self.introspections.push_back(IntrospectionLog {
            timestamp: chrono::Utc::now().timestamp() as f64,
            wellbeing: meta.wellbeing,
            intention: intention.to_string(),
            reflection,
        });

        self.insight_count += 1;

        while self.introspections.len() > self.max_introspections {
            self.introspections.pop_front();
        }
    }

    /// 获取自我摘要
    pub fn summary(&self) -> String {
        format!(
            "{}\nInsights: {}\nIntrospections: {}",
            self.self_model.summary(),
            self.insight_count,
            self.introspections.len()
        )
    }
}
