use rand::Rng;

pub fn rand7() -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=7)
}

pub fn rand10() -> i32 {
    let mut x = 41;
    while x > 40 {
        x = 7 * (rand7() - 1) + rand7();
    }
    x % 10 + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_rand10_range() {
        for _ in 0..10000 {
            let val = rand10();
            assert!(
                val >= 1 && val <= 10,
                "rand10() returned {} which is out of range [1, 10]",
                val
            );
        }
    }

    #[test]
    fn test_rand10_distribution() {
        let iterations = 100_000;
        let mut counts: HashMap<i32, i32> = HashMap::new();

        for _ in 0..iterations {
            let val = rand10();
            *counts.entry(val).or_insert(0) += 1;
        }

        // Ensure all values 1..=10 appeared
        for i in 1..=10 {
            assert!(
                counts.contains_key(&i),
                "Value {} did not appear in {} iterations",
                i,
                iterations
            );
        }

        // Check rough uniformity: each should be around iterations / 10
        let expected = iterations as f64 / 10.0;
        for i in 1..=10 {
            let count = *counts.get(&i).unwrap() as f64;
            let deviation = (count - expected).abs() / expected;
            assert!(
                deviation < 0.15,
                "Value {} has count {} which deviates too far from expected {} (deviation {:.2}%)",
                i,
                count as i32,
                expected as i32,
                deviation * 100.0
            );
        }
    }
}
