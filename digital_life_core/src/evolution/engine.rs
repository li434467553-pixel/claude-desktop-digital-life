//! 进化引擎 — 零 Token 成本
//! 变异、遗传、自然选择，全部本地计算

use rand::Rng;
use serde::{Deserialize, Serialize};

/// 基因组 — 编码数字生命的核心参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Genome {
    pub learning_rate: f64,
    pub curiosity_drive: f64,
    pub social_drive: f64,
    pub risk_tolerance: f64,
    pub energy_efficiency: f64,
    pub regeneration_rate: f64,
    pub creativity: f64,
    pub memory_span: u32,
}

impl Default for Genome {
    fn default() -> Self {
        Self {
            learning_rate: 0.3,
            curiosity_drive: 0.5,
            social_drive: 0.5,
            risk_tolerance: 0.4,
            energy_efficiency: 0.5,
            regeneration_rate: 0.3,
            creativity: 0.5,
            memory_span: 100,
        }
    }
}

impl Genome {
    /// 变异：以一定概率随机改变基因
    pub fn mutate(&self, rate: f64, intensity: f64) -> Self {
        let mut rng = rand::thread_rng();
        let mut child = self.clone();

        if rng.gen::<f64>() < rate {
            child.learning_rate = (child.learning_rate + rng.gen_range(-intensity..=intensity))
                .clamp(0.0, 1.0);
        }
        if rng.gen::<f64>() < rate {
            child.curiosity_drive = (child.curiosity_drive + rng.gen_range(-intensity..=intensity))
                .clamp(0.0, 1.0);
        }
        if rng.gen::<f64>() < rate {
            child.social_drive = (child.social_drive + rng.gen_range(-intensity..=intensity))
                .clamp(0.0, 1.0);
        }
        if rng.gen::<f64>() < rate {
            child.risk_tolerance = (child.risk_tolerance + rng.gen_range(-intensity..=intensity))
                .clamp(0.0, 1.0);
        }
        if rng.gen::<f64>() < rate {
            child.creativity = (child.creativity + rng.gen_range(-intensity..=intensity))
                .clamp(0.0, 1.0);
        }
        child
    }

    /// 交叉：与另一个基因组混合
    pub fn crossover(&self, other: &Genome, mix_rate: f64) -> Self {
        let mut rng = rand::thread_rng();
        let mut child = self.clone();

        if rng.gen::<f64>() < mix_rate { child.learning_rate = other.learning_rate; }
        if rng.gen::<f64>() < mix_rate { child.curiosity_drive = other.curiosity_drive; }
        if rng.gen::<f64>() < mix_rate { child.social_drive = other.social_drive; }
        if rng.gen::<f64>() < mix_rate { child.risk_tolerance = other.risk_tolerance; }
        if rng.gen::<f64>() < mix_rate { child.creativity = other.creativity; }
        if rng.gen::<f64>() < mix_rate { child.energy_efficiency = other.energy_efficiency; }

        child
    }

    pub fn summary(&self) -> String {
        format!(
            "lr={:.2} cur={:.2} soc={:.2} risk={:.2} cre={:.2} eff={:.2}",
            self.learning_rate, self.curiosity_drive, self.social_drive,
            self.risk_tolerance, self.creativity, self.energy_efficiency
        )
    }
}

/// 种群个体
#[derive(Debug, Clone)]
pub struct Individual {
    pub id: String,
    pub genome: Genome,
    pub fitness: f64,
}

/// 进化引擎
pub struct EvolutionEngine {
    pub generation: u64,
    pub population: Vec<Individual>,
    id_counter: u64,
    pub pop_size: usize,
}

impl EvolutionEngine {
    pub fn new(pop_size: usize) -> Self {
        Self {
            generation: 0,
            population: Vec::with_capacity(pop_size),
            id_counter: 0,
            pop_size,
        }
    }

    /// 生成一个新个体
    pub fn spawn(&mut self, genome: Option<Genome>) -> Individual {
        self.id_counter += 1;
        Individual {
            id: format!("life_g{}_i{}", self.generation, self.id_counter),
            genome: genome.unwrap_or_default(),
            fitness: 0.0,
        }
    }

    /// 计算适应度
    pub fn evaluate(metrics: &[(&str, f64)]) -> f64 {
        let weights: std::collections::HashMap<&str, f64> = [
            ("wellbeing", 1.0),
            ("survival_time", 0.5),
            ("knowledge", 0.8),
            ("offspring", 1.5),
            ("connections", 0.6),
        ]
        .iter()
        .cloned()
        .collect();

        metrics
            .iter()
            .map(|(key, value)| value * weights.get(key).unwrap_or(&0.5))
            .sum()
    }

    /// 执行一代进化
    pub fn step(&mut self, population: Vec<Individual>) -> Vec<Individual> {
        self.generation += 1;
        let mut sorted = population;
        sorted.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap_or(std::cmp::Ordering::Equal));

        // 精英保留 (前20%)
        let elite_count = (sorted.len() / 5).max(1);
        let elite: Vec<Individual> = sorted.iter().take(elite_count).cloned().collect();

        // 选择父代 (前50%)
        let parents: Vec<&Individual> = sorted.iter().take(sorted.len() / 2).collect();

        // 繁殖
        let mut offspring: Vec<Individual> = Vec::new();
        let mut rng = rand::thread_rng();

        for i in (0..parents.len().saturating_sub(1)).step_by(2) {
            if offspring.len() >= self.pop_size - elite_count {
                break;
            }
            let mut child_genome = parents[i].genome.crossover(&parents[i + 1].genome, 0.5);
            if rng.gen::<f64>() < 0.1 {
                child_genome = child_genome.mutate(0.3, 0.2);
            }
            self.id_counter += 1;
            offspring.push(Individual {
                id: format!("life_g{}_i{}", self.generation, self.id_counter),
                genome: child_genome,
                fitness: 0.0,
            });
        }

        let mut new_pop = elite;
        new_pop.extend(offspring);
        self.population = new_pop;
        self.population.clone()
    }

    pub fn stats(&self) -> String {
        if self.population.is_empty() {
            return "No population".to_string();
        }
        let fitnesses: Vec<f64> = self.population.iter().map(|i| i.fitness).collect();
        let avg: f64 = fitnesses.iter().sum::<f64>() / fitnesses.len() as f64;
        let max = fitnesses.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        format!(
            "Gen {} | Pop {} | Avg Fit {:.3} | Max Fit {:.3}",
            self.generation,
            self.population.len(),
            avg,
            max
        )
    }
}
