/// A categorical distribution for sampling from a discrete set of outcomes with associated probabilities.
/// This is useful for random selection where different outcomes have different likelihoods.
#[derive(Debug, Clone)]
pub struct CategoricalDistribution<T> {
    /// The possible outcomes and their cumulative weights
    outcomes: Vec<T>,
    cumulative_weights: Vec<f64>,
    total_weight: f64,
}

impl<T: Clone> CategoricalDistribution<T> {
    /// Create a new categorical distribution from outcomes and their weights.
    /// 
    /// # Arguments
    /// * `outcomes_and_weights` - A vector of (outcome, weight) pairs where weight > 0
    /// 
    /// # Panics
    /// Panics if any weight is <= 0 or if the input is empty
    pub fn new(outcomes_and_weights: Vec<(T, f64)>) -> Self {
        assert!(!outcomes_and_weights.is_empty(), "Cannot create empty categorical distribution");
        
        let mut outcomes = Vec::new();
        let mut cumulative_weights = Vec::new();
        let mut cumulative_sum = 0.0;
        
        for (outcome, weight) in outcomes_and_weights {
            assert!(weight > 0.0, "All weights must be positive, got {}", weight);
            
            outcomes.push(outcome);
            cumulative_sum += weight;
            cumulative_weights.push(cumulative_sum);
        }
        
        CategoricalDistribution {
            outcomes,
            cumulative_weights,
            total_weight: cumulative_sum,
        }
    }
    
    /// Create a uniform categorical distribution where all outcomes have equal probability.
    /// 
    /// # Arguments
    /// * `outcomes` - A vector of possible outcomes
    /// 
    /// # Panics
    /// Panics if the input is empty
    pub fn uniform(outcomes: Vec<T>) -> Self {
        assert!(!outcomes.is_empty(), "Cannot create empty categorical distribution");
        
        let weight = 1.0 / outcomes.len() as f64;
        let outcomes_and_weights = outcomes.into_iter()
            .map(|outcome| (outcome, weight))
            .collect();
            
        Self::new(outcomes_and_weights)
    }
    
    /// Sample a single outcome from the distribution.
    /// 
    /// # Arguments
    /// * `rng` - A random number generator
    /// 
    /// # Returns
    /// A reference to the sampled outcome
    pub fn sample(&self, rng: &mut impl rand::Rng) -> &T {
        let random_value = rng.random::<f64>() * self.total_weight;
        
        // Binary search for the first cumulative weight >= random_value
        let index = self.cumulative_weights
            .binary_search_by(|&weight| {
                if weight < random_value {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            })
            .unwrap_or_else(|i| i);
            
        &self.outcomes[index]
    }
    
    /// Sample a single outcome from the distribution and return an owned copy.
    /// 
    /// # Arguments
    /// * `rng` - A random number generator
    /// 
    /// # Returns
    /// A cloned copy of the sampled outcome
    pub fn sample_owned(&self, rng: &mut impl rand::Rng) -> T {
        self.sample(rng).clone()
    }
    
    /// Get the number of possible outcomes in this distribution.
    pub fn len(&self) -> usize {
        self.outcomes.len()
    }
    
    /// Check if the distribution is empty (should never happen after construction).
    pub fn is_empty(&self) -> bool {
        self.outcomes.is_empty()
    }
    
    /// Get all possible outcomes as a slice.
    pub fn outcomes(&self) -> &[T] {
        &self.outcomes
    }
    
    /// Get the probability of each outcome.
    /// Returns a vector where each element is the probability of the corresponding outcome.
    pub fn probabilities(&self) -> Vec<f64> {
        let mut probabilities = Vec::new();
        let mut prev_cumulative = 0.0;
        
        for &cumulative in &self.cumulative_weights {
            let probability = (cumulative - prev_cumulative) / self.total_weight;
            probabilities.push(probability);
            prev_cumulative = cumulative;
        }
        
        probabilities
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    #[test]
    fn test_categorical_distribution_creation() {
        let dist = CategoricalDistribution::new(vec![
            ("A", 1.0),
            ("B", 2.0),
            ("C", 3.0),
        ]);
        
        assert_eq!(dist.len(), 3);
        assert_eq!(dist.total_weight, 6.0);
        assert_eq!(dist.outcomes(), &["A", "B", "C"]);
    }
    
    #[test]
    fn test_uniform_distribution() {
        let dist = CategoricalDistribution::uniform(vec!["X", "Y", "Z"]);
        
        assert_eq!(dist.len(), 3);
        let probs = dist.probabilities();
        for prob in probs {
            assert!((prob - 1.0/3.0).abs() < 1e-10);
        }
    }
    
    #[test]
    fn test_sampling() {
        let dist = CategoricalDistribution::new(vec![
            ("Low", 1.0),
            ("High", 9.0),
        ]);
        
        let mut rng = StdRng::seed_from_u64(42);
        
        // Sample many times and check distribution roughly matches expectations
        let mut low_count = 0;
        let mut high_count = 0;
        let samples = 1000;
        
        for _ in 0..samples {
            match *dist.sample(&mut rng) {
                "Low" => low_count += 1,
                "High" => high_count += 1,
                _ => panic!("Unexpected outcome"),
            }
        }
        
        // With 1:9 ratio, we expect roughly 10% low, 90% high
        let low_ratio = low_count as f64 / samples as f64;
        let high_ratio = high_count as f64 / samples as f64;
        
        assert!(low_ratio < 0.2, "Low ratio too high: {}", low_ratio);   // Should be around 0.1
        assert!(high_ratio > 0.8, "High ratio too low: {}", high_ratio); // Should be around 0.9
        
        println!("Low: {:.1}%, High: {:.1}%", low_ratio * 100.0, high_ratio * 100.0);
    }
    
    #[test]
    fn test_sample_owned() {
        let dist = CategoricalDistribution::uniform(vec![String::from("Hello"), String::from("World")]);
        let mut rng = StdRng::seed_from_u64(123);
        
        let sample = dist.sample_owned(&mut rng);
        assert!(sample == "Hello" || sample == "World");
    }
    
    #[test]
    fn test_probabilities() {
        let dist = CategoricalDistribution::new(vec![
            ("A", 2.0),
            ("B", 3.0),
            ("C", 5.0),
        ]);
        
        let probs = dist.probabilities();
        assert_eq!(probs.len(), 3);
        assert!((probs[0] - 0.2).abs() < 1e-10); // 2/10 = 0.2
        assert!((probs[1] - 0.3).abs() < 1e-10); // 3/10 = 0.3
        assert!((probs[2] - 0.5).abs() < 1e-10); // 5/10 = 0.5
        
        // Probabilities should sum to 1
        let sum: f64 = probs.iter().sum();
        assert!((sum - 1.0).abs() < 1e-10);
    }
    
    #[test]
    #[should_panic(expected = "Cannot create empty categorical distribution")]
    fn test_empty_distribution_panics() {
        CategoricalDistribution::new(Vec::<(&str, f64)>::new());
    }
    
    #[test]
    #[should_panic(expected = "All weights must be positive")]
    fn test_negative_weight_panics() {
        CategoricalDistribution::new(vec![
            ("A", 1.0),
            ("B", -1.0),
        ]);
    }
    
    #[test]
    #[should_panic(expected = "All weights must be positive")]
    fn test_zero_weight_panics() {
        CategoricalDistribution::new(vec![
            ("A", 1.0),
            ("B", 0.0),
        ]);
    }
}