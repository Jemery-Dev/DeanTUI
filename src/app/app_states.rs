use strum::Display;
#[derive(Debug, Copy, Clone, Default, Display)]
pub enum AppState{
    #[default]
    #[strum(to_string = "Main menu")]
    Normal,

    #[strum(to_string = "Creating a new tablature")]
    CreatingNewTab,

    #[strum(to_string = "Editing a tablature")]
    EditingTab,

    #[strum(to_string) = "Playing tablature"]
    PlayingTab,
}