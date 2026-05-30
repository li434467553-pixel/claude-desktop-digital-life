//! 感知与行动核心 — 零 Token 成本
//! 实现感知、行动解析、意图到行动的映射

use serde::{Deserialize, Serialize};

/// 感知通道
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PerceptionChannel {
    Internal,
    Environment,
    Social,
    Memory,
    Meta,
}

impl PerceptionChannel {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Internal => "internal",
            Self::Environment => "env",
            Self::Social => "social",
            Self::Memory => "memory",
            Self::Meta => "meta",
        }
    }
}

/// 感知刺激
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stimulus {
    pub channel: PerceptionChannel,
    pub content: String,
    pub source: String,
    pub intensity: f64,
    pub priority: i32,
    pub timestamp: f64,
}

impl Stimulus {
    pub fn new(channel: PerceptionChannel, content: &str) -> Self {
        Self {
            channel,
            content: content.to_string(),
            source: String::new(),
            intensity: 0.5,
            priority: 0,
            timestamp: chrono::Utc::now().timestamp() as f64,
        }
    }
}

/// 行动类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionType {
    Explore,
    Rest,
    Socialize,
    Learn,
    Create,
    SeekResource,
    Assert,
    None,
}

impl ActionType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Explore => "explore",
            Self::Rest => "rest",
            Self::Socialize => "socialize",
            Self::Learn => "learn",
            Self::Create => "create",
            Self::SeekResource => "seek_resource",
            Self::Assert => "assert",
            Self::None => "none",
        }
    }
}

/// 行动
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub action_type: ActionType,
    pub description: String,
    pub priority: i32,
    pub duration: f64,
}

impl Action {
    pub fn new(action_type: ActionType, description: &str, duration: f64) -> Self {
        Self {
            action_type,
            description: description.to_string(),
            priority: 0,
            duration,
        }
    }
}

/// 行动引擎 — 将意图解析为行动
pub struct ActionEngine;

impl ActionEngine {
    pub fn resolve(intention: &str, _wellbeing: f64) -> Action {
        let i = intention.to_lowercase();
        if i.contains("explore") || i.contains("curios") {
            Action::new(ActionType::Explore, intention, 2.0)
        } else if i.contains("social") || i.contains("connect") {
            Action::new(ActionType::Socialize, intention, 1.5)
        } else if i.contains("learn") || i.contains("compet") {
            Action::new(ActionType::Learn, intention, 2.5)
        } else if i.contains("create") || i.contains("meaning") {
            Action::new(ActionType::Create, intention, 3.0)
        } else if i.contains("rest") || i.contains("safe") || i.contains("surviv") {
            Action::new(ActionType::Rest, intention, 1.0)
        } else if i.contains("assert") || i.contains("auton") {
            Action::new(ActionType::Assert, intention, 1.0)
        } else {
            Action::new(ActionType::None, "no clear intention", 0.5)
        }
    }

    pub fn action_effect(action_type: ActionType) -> Option<(&'static str, f64)> {
        match action_type {
            ActionType::Explore => Some(("curiosity", 0.2)),
            ActionType::Rest => Some(("survival", 0.3)),
            ActionType::Socialize => Some(("social", 0.25)),
            ActionType::Learn => Some(("competence", 0.2)),
            ActionType::Create => Some(("meaning", 0.25)),
            ActionType::SeekResource => Some(("survival", 0.15)),
            ActionType::Assert => Some(("autonomy", 0.2)),
            ActionType::None => None,
        }
    }
}
