//! 世界环境 — 数字生命的生态场

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rand::Rng;

/// 资源类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceType {
    Information,
    Energy,
    Social,
    Space,
}

/// 资源
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub id: String,
    pub resource_type: ResourceType,
    pub quantity: f64,
    pub location: String,
    pub renew_rate: f64,
}

impl Resource {
    pub fn new(resource_type: ResourceType, location: &str, renew_rate: f64) -> Self {
        Self {
            id: Uuid::new_v4().to_string()[..8].to_string(),
            resource_type,
            quantity: 1.0,
            location: location.to_string(),
            renew_rate,
        }
    }
}

/// 世界事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldEvent {
    pub id: String,
    pub event_type: String,
    pub description: String,
    pub timestamp: f64,
}

impl WorldEvent {
    pub fn new(event_type: &str, description: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string()[..8].to_string(),
            event_type: event_type.to_string(),
            description: description.to_string(),
            timestamp: chrono::Utc::now().timestamp() as f64,
        }
    }
}

/// 世界环境
pub struct WorldEnvironment {
    pub name: String,
    pub time: f64,
    pub resources: Vec<Resource>,
    pub events: Vec<WorldEvent>,
    max_events: usize,
}

impl WorldEnvironment {
    pub fn new(name: &str) -> Self {
        let mut world = Self {
            name: name.to_string(),
            time: 0.0,
            resources: Vec::new(),
            events: Vec::new(),
            max_events: 1000,
        };

        // 初始化默认资源
        world.resources.push(Resource::new(ResourceType::Information, "zone_a", 0.02));
        world.resources.push(Resource::new(ResourceType::Energy, "zone_a", 0.01));
        world.resources.push(Resource::new(ResourceType::Social, "zone_b", 0.015));
        world.resources.push(Resource::new(ResourceType::Space, "zone_c", 0.005));

        world
    }

    /// 推进世界时间
    pub fn tick(&mut self, dt: f64) {
        self.time += dt;

        // 资源再生
        for resource in &mut self.resources {
            if resource.quantity < 1.0 {
                resource.quantity =
                    (resource.quantity + resource.renew_rate * dt).min(1.0);
            }
        }

        // 随机世界事件
        if rand::thread_rng().gen::<f64>() < 0.01 * dt {
            let event_types = [
                ("info_boom", "New information flows through the environment"),
                ("scarcity", "A resource becomes scarce"),
                ("discovery", "A new area is discovered"),
                ("connection", "An opportunity for connection arises"),
            ];
            let idx = rand::thread_rng().gen_range(0..event_types.len());
            let (et, desc) = event_types[idx];
            self.events.push(WorldEvent::new(et, desc));
            if self.events.len() > self.max_events {
                self.events.remove(0);
            }
        }
    }

    /// 消耗资源
    pub fn consume_resource(&mut self, resource_id: &str, amount: f64) -> f64 {
        if let Some(resource) = self.resources.iter_mut().find(|r| r.id == resource_id) {
            let consumed = amount.min(resource.quantity);
            resource.quantity -= consumed;
            consumed
        } else {
            0.0
        }
    }

    pub fn summary(&self) -> String {
        format!(
            "World: {} | time={:.1} | resources={} | events={}",
            self.name,
            self.time,
            self.resources.len(),
            self.events.len()
        )
    }
}
