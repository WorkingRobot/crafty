use crate::simulator::Simulator;
use enum_indexing::EnumIndexing;

use super::{
    action::ActionTrait, advanced_touch::AdvancedTouch, basic_synthesis::BasicSynthesis,
    basic_touch::BasicTouch, byregots_blessing::ByregotsBlessing,
    careful_observation::CarefulObservation, careful_synthesis::CarefulSynthesis,
    delicate_synthesis::DelicateSynthesis, final_appraisal::FinalAppraisal,
    focused_synthesis::FocusedSynthesis, focused_touch::FocusedTouch, great_strides::GreatStrides,
    groundwork::Groundwork, hasty_touch::HastyTouch, heart_and_soul::HeartAndSoul,
    innovation::Innovation, intensive_synthesis::IntensiveSynthesis, manipulation::Manipulation,
    masters_mend::MastersMend, muscle_memory::MuscleMemory, observe::Observe,
    precise_touch::PreciseTouch, preparatory_touch::PreparatoryTouch,
    prudent_synthesis::PrudentSynthesis, prudent_touch::PrudentTouch,
    rapid_synthesis::RapidSynthesis, reflect::Reflect, standard_touch::StandardTouch,
    trained_eye::TrainedEye, trained_finesse::TrainedFinesse,
    tricks_of_the_trade::TricksOfTheTrade, veneration::Veneration, waste_not::WasteNot,
    waste_not2::WasteNot2,
};

macro_rules! nested {
    (
        $(pub)? enum $enum_name:ident {
            $($variant:ident),+
        }

        impl $trait_name:ident {
            $($(pub)? fn $func_name:ident($($func_arg:ident: $func_arg_type:ty),*)$( -> $func_ret_type:ty)?;)+
        }
    ) => {
        #[derive(PartialEq, Copy, Clone, Debug, EnumIndexing)]
        pub enum $enum_name {
            $($variant),*
        }

        impl $enum_name {
            pub const VALUES: &'static [$enum_name] = &[
                $($enum_name::$variant,)*
            ];

            nested!{@0 $enum_name ($(($func_name @ self @ (self, $($func_arg: $func_arg_type),*) @ ($($func_arg)*) @ $($func_ret_type)?);)*) @ ($($variant),*)}
        }
    };

    (@0 $enum_name:ident ($(($($func_data:tt)+);)+) @ $tuple:tt) => {
        $(nested!{@1 $enum_name ($($func_data)+) @ $tuple})*
    };

    (@1 $enum_name:ident ($func_name:ident @ $self_arg:ident @ $func_arg:tt @ $func_param:tt @ $($func_ret_type:ty)?) @ ($($variant:ident),*)) => {
        pub fn $func_name$func_arg$( -> $func_ret_type)? {
            match $self_arg {
                $($enum_name::$variant => $variant::$func_name$func_param),*
            }
        }
    };
}

nested! {
    pub enum Action {
        AdvancedTouch,
        BasicSynthesis,
        BasicTouch,
        ByregotsBlessing,
        CarefulObservation,
        CarefulSynthesis,
        DelicateSynthesis,
        FinalAppraisal,
        FocusedSynthesis,
        FocusedTouch,
        GreatStrides,
        Groundwork,
        HastyTouch,
        HeartAndSoul,
        Innovation,
        IntensiveSynthesis,
        Manipulation,
        MastersMend,
        MuscleMemory,
        Observe,
        PreciseTouch,
        PreparatoryTouch,
        PrudentSynthesis,
        PrudentTouch,
        RapidSynthesis,
        Reflect,
        StandardTouch,
        TrainedEye,
        TrainedFinesse,
        TricksOfTheTrade,
        Veneration,
        WasteNot,
        WasteNot2
    }

    impl ActionTrait {
        pub fn level() -> u8;
        pub fn increases_progress() -> bool;
        pub fn increases_quality() -> bool;
        pub fn durability_cost() -> i8;
        pub fn increases_step_count() -> bool;
        pub fn cp_cost(sim: &impl Simulator) -> u32;
        pub fn efficiency(sim: &impl Simulator) -> f32;
        pub fn success_rate(sim: &impl Simulator) -> f32;
        pub fn can_use(sim: &impl Simulator) -> bool;
        pub fn use_action(sim: &mut impl Simulator);
        pub fn use_success(sim: &mut impl Simulator);
    }
}
