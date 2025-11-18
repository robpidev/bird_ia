use crate::chromosome::Crhomosome;

pub trait Individual {
    fn create(chromosome: Crhomosome) -> Self;
    fn fitness(&self) -> f32;
    fn chromosome(&self) -> &Crhomosome;
}
