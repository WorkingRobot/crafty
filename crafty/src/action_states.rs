#[derive(Default, Clone, Debug)]
pub struct ActionStates {
    pub touch_combo_step: u8,
    pub careful_observation_count: u8,
    pub used_heart_and_soul: bool,
    pub has_observed: bool,
}
