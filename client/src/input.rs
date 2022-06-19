use leafwing_input_manager::Actionlike;

// This is the list of "things in the game I want to be able to do based on input"
#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Action {
    Pan,
    Select,
    Zoom,
}
