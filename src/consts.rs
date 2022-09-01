
pub const APP_STATE_STAGE: &str = "app_stage_state";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    Menu,
    Game,
}
