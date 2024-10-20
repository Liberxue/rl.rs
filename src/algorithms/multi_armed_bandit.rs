use rand::Rng;

pub struct MultiArmedBandit {
    pub n_arms: usize,
    pub q_values: Vec<f64>,
    pub n_pulls: Vec<usize>,
    pub epsilon: f64,
}

impl MultiArmedBandit {
    pub fn new(n_arms: usize, epsilon: f64) -> Self {
        MultiArmedBandit {
            n_arms,
            q_values: vec![0.0; n_arms],
            n_pulls: vec![0; n_arms],
            epsilon,
        }
    }

    pub fn select_arm(&mut self) -> usize {
        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() < self.epsilon {
            rng.gen_range(0..self.n_arms)
        } else {
            self.q_values
                .iter()
                .enumerate()
                .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
                .map(|(i, _)| i)
                .unwrap()
        }
    }

    pub fn update_q_value(&mut self, arm: usize, reward: f64) {
        self.n_pulls[arm] += 1;
        let n = self.n_pulls[arm] as f64;
        let q = self.q_values[arm];
        self.q_values[arm] += (reward - q) / n;
    }

    pub fn simulate_reward(arm: usize) -> f64 {
        let true_rewards = [1.0, 1.5, 0.5, 2.0];
        let noise: f64 = rand::thread_rng().gen_range(0.0..0.1);
        true_rewards[arm] + noise
    }
}
