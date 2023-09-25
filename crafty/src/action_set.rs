use enumflags2::{BitFlag, BitFlags};
use rand::{rngs::SmallRng, Rng};

use crate::Action;

pub type ActionSet = BitFlags<Action>;

pub trait BitFlagExt<T: BitFlag> {
    fn keep<F: Fn(T) -> bool>(&mut self, f: F);

    fn sample(&self, rng: &mut SmallRng) -> T;

    fn pick(&mut self, rng: &mut SmallRng) -> T;
}

impl<T: BitFlag> BitFlagExt<T> for BitFlags<T> {
    fn keep<F: Fn(T) -> bool>(&mut self, f: F) {
        for item in self.iter() {
            if !f(item) {
                self.remove(item);
            }
        }
    }

    fn sample(&self, rng: &mut SmallRng) -> T {
        self.iter().nth(rng.gen_range(0..self.len())).unwrap()
    }

    fn pick(&mut self, rng: &mut SmallRng) -> T {
        let ret = self.sample(rng);
        self.remove(ret);
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use Action::*;

    #[test]
    fn set_and_unset_works() {
        let mut set = ActionSet::default();

        set.insert(BasicTouch);
        set.insert(BasicSynthesis);
        assert_eq!(set.len(), 2);

        set.remove(BasicTouch);
        set.remove(BasicSynthesis);
        assert!(set.is_empty());
    }

    #[test]
    fn keep_works() {
        let mut set = ActionSet::default();
        set.insert(BasicTouch);
        set.insert(BasicSynthesis);
        set.insert(GreatStrides);
        set.insert(MuscleMemory);

        set.keep(|action| action != BasicTouch && action != GreatStrides);
        assert_eq!(set.len(), 2);
        assert!(set.contains(BasicSynthesis));
        assert!(set.contains(MuscleMemory));
    }

    #[test]
    fn random_index_works() {
        let mut set = ActionSet::default();
        set.insert(BasicTouch);
        set.insert(BasicSynthesis);
        set.insert(GreatStrides);
        set.insert(TrainedFinesse);

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
