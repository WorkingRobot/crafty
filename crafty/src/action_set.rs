use enumflags2::_internal::RawBitFlags;
use enumflags2::{BitFlag, BitFlags};
use rand::{rngs::SmallRng, Rng};

use crate::{intrinsics, Action};

pub type ActionSet = BitFlags<Action>;

pub trait BitFlagExt<T: BitFlag>: BitFlagNth<T> {
    fn keep<F: Fn(T) -> bool>(&mut self, f: F);

    fn sample(&self, rng: &mut SmallRng) -> T;

    fn pick(&mut self, rng: &mut SmallRng) -> T;
}

pub trait BitFlagNth<T: BitFlag> {
    fn nth(&self, idx: u32) -> T;
}

impl<T: BitFlag + RawBitFlags<Numeric = u32>> BitFlagNth<T> for BitFlags<T, u32> {
    fn nth(&self, idx: u32) -> T {
        let flag = 1u32 << intrinsics::nth_bit_set_32(self.bits(), idx);
        unsafe { core::mem::transmute_copy(&flag) }
    }
}

impl<T: BitFlag + RawBitFlags<Numeric = u64>> BitFlagNth<T> for BitFlags<T, u64> {
    fn nth(&self, idx: u32) -> T {
        let flag = 1u64 << intrinsics::nth_bit_set_64(self.bits(), idx);
        unsafe { core::mem::transmute_copy(&flag) }
    }
}

impl<T: BitFlag + RawBitFlags<Numeric = u32>> BitFlagExt<T> for BitFlags<T, u32> {
    fn keep<F: Fn(T) -> bool>(&mut self, f: F) {
        for item in self.iter() {
            if !f(item) {
                self.remove(item);
            }
        }
    }

    fn sample(&self, rng: &mut SmallRng) -> T {
        self.nth(rng.gen_range(0..self.len()) as u32)
    }

    fn pick(&mut self, rng: &mut SmallRng) -> T {
        let ret = self.sample(rng);
        self.remove(ret);
        ret
    }
}

impl<T: BitFlag + RawBitFlags<Numeric = u64>> BitFlagExt<T> for BitFlags<T, u64> {
    fn keep<F: Fn(T) -> bool>(&mut self, f: F) {
        for item in self.iter() {
            if !f(item) {
                self.remove(item);
            }
        }
    }

    fn sample(&self, rng: &mut SmallRng) -> T {
        self.nth(rng.gen_range(0..self.len()) as u32)
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
