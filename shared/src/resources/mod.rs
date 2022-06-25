/// Stores configuration on the map of a single game
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct MapConfig {
    /// The number of tiles in width the map is
    pub size_width: u16,
    /// The number of tiles in height the map is
    pub size_height: u16,
}
