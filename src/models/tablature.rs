use serde::{Deserialize, Serialize};
use crate::models::time_signature::TimeSignature;
use crate::models::note::NoteName;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tablature {
    name: String, // The name of the tablature
    strings: Vec<GuitarString> // List of all strings of the tab
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuitarString { // Struct for guitar strings
    name: NoteName,
    max_fret_position: u16, // Max number for the frest (Usually around 21~24)
    measures: Vec<Measure>
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Measure {
    bpm: u16, //Beats per minute
    time_signature: TimeSignature, // Time signature for the measure
    notes: Vec<Note> // All notes for the measure
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    name: NoteName,
    fret_position: u16, // Position on the string
    duration: u16 // Duration (Number of beats for the note (Whole 4000, half 2000, quarter 1000, eighth 500)
}


