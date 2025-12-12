//! Deterministic randomness for reproducible chaos.
//!
//! ABSURDTTY uses seeded random number generation to ensure that
//! the same seed + same input = same absurd output. This enables:
//! - Screenshots that can be reproduced
//! - Demos that work twice
//! - Polite chaos for shared environments
//!
//! # Example
//!
//! ```
//! use absurd_core::seed::Chaos;
//!
//! // With explicit seed - deterministic across runs
//! let mut chaos1 = Chaos::seeded(42);
//! let mut chaos2 = Chaos::seeded(42);
//! // Same seed, same sequence
//! assert_eq!(chaos1.pick(&["a", "b", "c"]), chaos2.pick(&["a", "b", "c"]));
//!
//! // Without seed - truly random
//! let mut chaos = Chaos::unseeded();
//! // Results vary each run
//! ```

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

/// The source of all deterministic chaos.
///
/// Wraps a ChaCha8 RNG for cryptographically-irrelevant but
/// reproducible randomness. Named `Chaos` because `Random` would
/// be too optimistic.
pub struct Chaos {
    rng: ChaCha8Rng,
    seed: Option<u64>,
}

impl Chaos {
    /// Creates a new Chaos instance with a specific seed.
    ///
    /// Same seed = same sequence of chaos. This is the polite option.
    pub fn seeded(seed: u64) -> Self {
        Self {
            rng: ChaCha8Rng::seed_from_u64(seed),
            seed: Some(seed),
        }
    }

    /// Creates a new Chaos instance without a seed.
    ///
    /// Each run produces different results. This is the honest option.
    pub fn unseeded() -> Self {
        Self {
            rng: ChaCha8Rng::from_entropy(),
            seed: None,
        }
    }

    /// Creates Chaos from an optional seed.
    ///
    /// Convenience method for CLI flag handling.
    pub fn from_optional_seed(seed: Option<u64>) -> Self {
        match seed {
            Some(s) => Self::seeded(s),
            None => Self::unseeded(),
        }
    }

    /// Returns the seed if one was set, None if entropy-based.
    pub fn seed(&self) -> Option<u64> {
        self.seed
    }

    /// Returns true if this Chaos is deterministic (seeded).
    pub fn is_deterministic(&self) -> bool {
        self.seed.is_some()
    }

    /// Pick a random element from a slice.
    ///
    /// Returns None if the slice is empty (even chaos cannot
    /// choose from nothing).
    pub fn pick<'a, T>(&mut self, items: &'a [T]) -> Option<&'a T> {
        if items.is_empty() {
            None
        } else {
            let index = self.rng.gen_range(0..items.len());
            Some(&items[index])
        }
    }

    /// Pick a random element, panicking if empty.
    ///
    /// Use when emptiness would be a programming error.
    pub fn pick_unwrap<'a, T>(&mut self, items: &'a [T]) -> &'a T {
        self.pick(items).expect("cannot pick from empty slice")
    }

    /// Generate a random boolean with given probability of true.
    ///
    /// `probability` should be between 0.0 and 1.0.
    pub fn chance(&mut self, probability: f64) -> bool {
        self.rng.gen_bool(probability.clamp(0.0, 1.0))
    }

    /// Generate a random integer in the given range.
    pub fn range(&mut self, min: i64, max: i64) -> i64 {
        self.rng.gen_range(min..=max)
    }

    /// Generate a random float between 0.0 and 1.0.
    pub fn float(&mut self) -> f64 {
        self.rng.gen()
    }

    /// Shuffle a slice in place.
    pub fn shuffle<T>(&mut self, items: &mut [T]) {
        use rand::seq::SliceRandom;
        items.shuffle(&mut self.rng);
    }

    /// Generate a case ID in ABSURDTTY format.
    ///
    /// Format: `AB-YYYYMMDD-NNN` where NNN is a random 3-digit number.
    pub fn case_id(&mut self, date: &str) -> String {
        let num: u16 = self.rng.gen_range(1..=999);
        format!("AB-{}-{:03}", date, num)
    }

}

impl Default for Chaos {
    fn default() -> Self {
        Self::unseeded()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_is_deterministic() {
        let mut c1 = Chaos::seeded(42);
        let mut c2 = Chaos::seeded(42);

        for _ in 0..100 {
            assert_eq!(c1.range(0, 1000), c2.range(0, 1000));
        }
    }

    #[test]
    fn different_seeds_differ() {
        let mut c1 = Chaos::seeded(42);
        let mut c2 = Chaos::seeded(43);

        // Collect sequences
        let seq1: Vec<_> = (0..10).map(|_| c1.range(0, 1000)).collect();
        let seq2: Vec<_> = (0..10).map(|_| c2.range(0, 1000)).collect();

        assert_ne!(seq1, seq2);
    }

    #[test]
    fn pick_from_empty_returns_none() {
        let mut chaos = Chaos::seeded(42);
        let empty: Vec<i32> = vec![];
        assert!(chaos.pick(&empty).is_none());
    }

    #[test]
    fn pick_from_single_returns_that_element() {
        let mut chaos = Chaos::seeded(42);
        let single = vec!["only"];
        assert_eq!(chaos.pick(&single), Some(&"only"));
    }

    #[test]
    fn chance_respects_probability() {
        let mut chaos = Chaos::seeded(42);

        // 0% should always be false
        for _ in 0..100 {
            assert!(!chaos.chance(0.0));
        }

        // 100% should always be true
        for _ in 0..100 {
            assert!(chaos.chance(1.0));
        }
    }

    #[test]
    fn case_id_format() {
        let mut chaos = Chaos::seeded(42);
        let id = chaos.case_id("20251212");
        assert!(id.starts_with("AB-20251212-"));
        assert_eq!(id.len(), 15); // AB-YYYYMMDD-NNN
    }

    #[test]
    fn shuffle_is_deterministic_when_seeded() {
        let mut c1 = Chaos::seeded(42);
        let mut c2 = Chaos::seeded(42);

        let mut v1 = vec![1, 2, 3, 4, 5];
        let mut v2 = vec![1, 2, 3, 4, 5];

        c1.shuffle(&mut v1);
        c2.shuffle(&mut v2);

        assert_eq!(v1, v2);
    }
}

