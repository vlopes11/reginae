use crate::Board;

#[derive(Default, Clone)]
pub struct Evaluator {
    evaluators: Vec<WeightedEvaluator>,
}

impl Evaluator {
    pub fn score(&self, board: &Board, last_move: usize) -> u64 {
        let total_weight = self
            .evaluators
            .iter()
            .map(|w| w.weight.abs())
            .sum::<f64>()
            .max(f64::MIN_POSITIVE);

        let score: f64 = self
            .evaluators
            .iter()
            .map(|w| (w.f)(board, last_move) * w.weight / total_weight)
            .sum();

        let score = score.max(f64::MIN_POSITIVE).min(1.0);

        #[cfg(feature = "tracing")]
        tracing::debug!("computed weighted score {score}");

        (score * u64::MAX as f64) as u64
    }

    pub fn inject_evaluator(&mut self, f: fn(&Board, usize) -> f64, weight: f64) -> &mut Self {
        self.evaluators.push(WeightedEvaluator { f, weight });
        self
    }

    pub fn reset(&mut self) -> &mut Self {
        self.evaluators.clear();
        self
    }
}

#[derive(Clone)]
struct WeightedEvaluator {
    pub f: fn(&Board, usize) -> f64,
    pub weight: f64,
}
