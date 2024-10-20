use rl_rs::algorithms::{bernoulli_bandit::BernoulliBandit, multi_armed_bandit::MultiArmedBandit};

fn main() {
    multi_armed_bandit_example();
    bernoulli_bandit_example();
}

fn multi_armed_bandit_example() {
    // Create a bandit with 4 arms and ε = 0.1
    let mut bandit = MultiArmedBandit::new(4, 0.1);

    // Run 10 selections and updates
    for _ in 0..10 {
        let selected_arm = bandit.select_arm();
        let reward = MultiArmedBandit::simulate_reward(selected_arm);
        bandit.update_q_value(selected_arm, reward);

        println!(
            "Selected arm: {}, Reward: {:.2}, Q-values: {:?}",
            selected_arm, reward, bandit.q_values
        );
    }
}

fn bernoulli_bandit_example() {
    let k = 10;
    let bandit = BernoulliBandit::new(k);

    println!("Generated Bernoulli bandit with {} arms", k);
    println!(
        "Best arm: {}, Probability: {:.4}",
        bandit.best_idx, bandit.best_prob
    );

    // Simulate pulling each arm
    for i in 0..k {
        let result = bandit.step(i);
        println!("Pulled arm {}, Result: {}", i, result);
    }
}

