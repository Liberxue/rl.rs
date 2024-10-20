use rand::Rng;

pub struct BernoulliBandit {
    pub probs: Vec<f64>,
    pub best_idx: usize,
    pub best_prob: f64,
}

impl BernoulliBandit {
    pub fn new(k: usize) -> Self {
        let mut rng = rand::thread_rng();
        let probs: Vec<f64> = (0..k).map(|_| rng.gen_range(0.0..1.0)).collect(); // 生成 K 个 [0,1) 的随机数作为概率
        let best_idx = probs
            .iter()
            .cloned()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap()
            .0;

        let best_prob = probs[best_idx];

        BernoulliBandit {
            probs,
            best_idx,
            best_prob,
        }
    }

    pub fn step(&self, k: usize) -> u8 {
        let mut rng = rand::thread_rng();
        if rng.gen::<f64>() < self.probs[k] {
            1
        } else {
            0
        }
    }
}
