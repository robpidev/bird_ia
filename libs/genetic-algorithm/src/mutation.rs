use rand::{Rng, RngCore};

use crate::chromosome::Crhomosome;

pub trait MutationMethod {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Crhomosome);
}

pub struct GaussianMutation {
    /// Probability of changing a gene:
    /// - 0.0 -> no genes will be touched
    /// - 1.0 -> all genes will be touched
    chance: f32,

    /// Magnitud of that change:
    /// 0.0 -> touched genes will not be modified
    /// 3.0 -> touched genes will be += or -= by at most 3.0
    coeff: f32,
}

impl GaussianMutation {
    pub fn new(chance: f32, coeff: f32) -> Self {
        assert!(chance >= 0.0 && chance <= 1.0);
        Self { chance, coeff }
    }
}

impl MutationMethod for GaussianMutation {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Crhomosome) {
        for gene in child.iter_mut() {
            let sign = if rng.random_bool(0.5) { 1.0 } else { -1.0 };

            if rng.random_bool(self.chance as f64) {
                *gene += sign * self.coeff * rng.random::<f32>();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    mod gaussian_mutation {

        use rand::SeedableRng;
        use rand_chacha::ChaCha8Rng;

        use crate::mutation::{GaussianMutation, MutationMethod};

        fn actual(chance: f32, coeff: f32) -> Vec<f32> {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let mut child = (1..=5).map(|n| n as f32).into_iter().collect();

            GaussianMutation::new(chance, coeff).mutate(&mut rng, &mut child);
            child.into_iter().collect()
        }

        mod given_zero_chance {
            use approx::assert_relative_eq;

            fn actual(coeff: f32) -> Vec<f32> {
                super::actual(0.0, coeff)
            }

            mod and_zero_coeff {

                use super::*;

                #[test]
                fn does_not_change_the_original_chromosome() {
                    let actual = actual(0.0);
                    let expected: Vec<f32> = (1..=5).map(|n| n as f32).collect();
                    assert_relative_eq!(actual.as_slice(), expected.as_slice());
                }
            }

            mod and_non_zero_coeff {
                use super::*;

                #[test]
                fn does_not_change_the_original_chromosome() {
                    let actual = actual(1.0);
                    let expected: Vec<f32> = (1..=5).map(|n| n as f32).collect();
                    assert_relative_eq!(actual.as_slice(), expected.as_slice());
                }
            }
        }

        mod given_fifty_fifty_chace {
            use approx::assert_relative_eq;

            fn actual(coeff: f32) -> Vec<f32> {
                super::actual(0.55, coeff)
            }

            mod and_zero_coeff {
                use super::*;

                #[test]
                fn does_not_change_the_original_chromosome() {
                    let actual = actual(0.0);
                    let expected: Vec<f32> = (1..=5).map(|n| n as f32).collect();
                    assert_relative_eq!(actual.as_slice(), expected.as_slice());
                }
            }

            mod and_non_zero_coeff {
                use super::*;
                #[test]
                fn slightly_changes_the_original_chromosome() {
                    let actual = actual(2.0);
                    let expected = vec![1.0, 2.8975005, 3.0, 3.361278, 5.0];
                    assert_relative_eq!(actual.as_slice(), expected.as_slice());
                }
            }
        }

        mod given_max_chance {
            use approx::assert_relative_eq;

            fn actual(coeff: f32) -> Vec<f32> {
                super::actual(1.0, coeff)
            }

            mod and_zero_coeff {
                use super::*;
                #[test]
                fn does_not_change_the_original_chromosome() {
                    let actual = actual(0.0);
                    let expected: Vec<f32> = (1..=5).map(|n| n as f32).collect();
                    assert_relative_eq!(actual.as_slice(), expected.as_slice());
                }
            }

            mod and_non_zero_coeff {
                use super::*;
                #[test]
                fn entirely_changes_the_original_chromosome() {
                    let actual = actual(2.0);
                    let expected = vec![-0.8181261, 1.5351684, 3.8975005, 4.19795, 6.4452357];
                    assert_relative_eq!(actual.as_slice(), expected.as_slice());
                }
            }
        }
    }
}
