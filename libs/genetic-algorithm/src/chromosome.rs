use std::ops::Index;

#[derive(Debug, Clone)]
pub struct Crhomosome {
    pub genes: Vec<f32>,
}

impl PartialEq for Crhomosome {
    fn eq(&self, other: &Self) -> bool {
        approx::relative_eq!(self.genes.as_slice(), other.genes.as_slice())
    }
}

impl Crhomosome {
    pub fn len(&self) -> usize {
        self.genes.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &f32> {
        self.genes.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut f32> {
        self.genes.iter_mut()
    }
}

// This allow the Chromosomes.genes[index]
impl Index<usize> for Crhomosome {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.genes[index]
    }
}

// This allows .collect()
impl FromIterator<f32> for Crhomosome {
    fn from_iter<T: IntoIterator<Item = f32>>(iter: T) -> Self {
        Self {
            genes: iter.into_iter().collect(),
        }
    }
}

// This allows IntoIterator
impl IntoIterator for Crhomosome {
    type Item = f32;
    type IntoIter = std::vec::IntoIter<f32>;

    fn into_iter(self) -> Self::IntoIter {
        self.genes.into_iter()
    }
}
