use enumflags2::BitFlags;
use rand::{rngs::SmallRng, Rng};

use crate::Action;

#[derive(Debug, Default, Clone)]
pub struct ActionSet(BitFlags<Action>);

impl ActionSet {
    pub fn set(&mut self, action: Action) {
        self.0.insert(action)
    }

    pub fn unset(&mut self, action: Action) {
        self.0.remove(action)
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_vec(actions: &Vec<Action>) -> Self {
        let mut instance = Self::new();

        for action in actions {
            instance.set(*action);
        }

        instance
    }

    pub fn contains(&self, action: Action) -> bool {
        self.0.contains(action)
    }

    /// Iterates through Actions in the set and keeps or removes them based on
    /// the closure `f` provided.
    ///
    /// Similar to Vec's retain method.
    pub fn keep<F>(&mut self, mut f: F)
    where
        F: FnMut(&Action) -> bool,
    {
        for action in self.0 {
            if !f(&action) {
                self.unset(action);
            }
        }
    }

    /// Returns a random Action from the set
    pub fn sample(&self, rng: &mut SmallRng) -> Action {
        let n = rng.gen_range(0..self.len());
        self.0.iter().nth(n as usize).unwrap()
    }

    /// Removes and returns a random Action from the set
    pub fn pick(&mut self, rng: &mut SmallRng) -> Action {
        let ret = self.sample(rng);
        self.unset(ret);
        ret
    }

    pub fn len(&self) -> u32 {
        self.0.len() as u32
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn to_vec(&self) -> Vec<Action> {
        let mut actions = vec![];

        for action in Action::ACTIONS {
            if self.contains(*action) {
                actions.push(*action);
            }
        }

        actions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use Action::*;

    #[test]
    fn set_and_unset_works() {
        let mut set = ActionSet::new();

        set.set(BasicTouch);
        set.set(BasicSynthesis);
        assert_eq!(set.len(), 2);

        set.unset(BasicTouch);
        set.unset(BasicSynthesis);
        assert!(set.is_empty());
    }

    #[test]
    fn keep_works() {
        let mut set = ActionSet::new();
        set.set(BasicTouch);
        set.set(BasicSynthesis);
        set.set(GreatStrides);
        set.set(MuscleMemory);

        set.keep(|action| *action != BasicTouch && *action != GreatStrides);
        assert_eq!(set.len(), 2);
        assert!(set.contains(BasicSynthesis));
        assert!(set.contains(MuscleMemory));
    }

    #[test]
    fn random_index_works() {
        let mut set = ActionSet::new();
        set.set(BasicTouch);
        set.set(BasicSynthesis);
        set.set(GreatStrides);
        set.set(TrainedFinesse);

        let mut counts = vec![0; Action::ACTIONS.len()];
        let mut rng = SmallRng::seed_from_u64(1);
        for _ in 0..100 {
            let random_action = set.sample(&mut rng);

            assert!(
                [BasicTouch, BasicSynthesis, GreatStrides, TrainedFinesse].contains(&random_action)
            );

            counts[(random_action as u32).ilog2() as usize] += 1;
        }

        assert!(counts[(BasicTouch as u32).ilog2() as usize] > 0);
        assert!(counts[(BasicSynthesis as u32).ilog2() as usize] > 0);
        assert!(counts[(GreatStrides as u32).ilog2() as usize] > 0);
        assert!(counts[(TrainedFinesse as u32).ilog2() as usize] > 0);
    }
}
