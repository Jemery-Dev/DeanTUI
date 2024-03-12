use strum::Display;
#[derive(Debug, Copy, Clone, Default, Display)]
pub enum AppState{
    #[default]
    #[strum(to_string = "Start of app")]
    Normal,

    #[strum(to_string = "Main Menu")]
    Main,

    #[strum(to_string = "Making new project")]
    NewProject,

    #[strum(to_string = "Creating a new tablature")]
    CreatingNewTab,

    #[strum(to_string = "Editing a tablature")]
    EditingTab,

    #[strum(to_string = "Playing tablature")]
    PlayingTab,
}