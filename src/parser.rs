use std::fs;
use std::path::PathBuf;

mod parser {

    pub struct Beatmap {
        version: String,
        general: General,
        editor: Editor,
        metadata: Metadata,
        difficulty: Difficulty,
    }

    pub struct General {
        AudioFilename: PathBuf,
        AudioLeadIn: i32,
        PreviewTime: i32,
        Countdown: i32,
        SampleSet: String,
        StackLeniency: f64,
        Mode: i8,
    }

    pub struct Editor {
        DistanceSpacing: f64,
        BeatDivisor: i32,
        GridSize: i32,
    }

    pub struct Metadata {
        Title: String,
        TitleUnicode: String,
        Artist: String,
        ArtistUnicode: String,
        Creator: String,
        Version: String,
        Source: String,
        Tags: Vec<String>,
        BeatmapID: i32,
        BeatmapSetID: i32,
    }

    pub struct Difficulty {
        HPDrainRate: f64,
        CircleSize: f64,
        OverallDifficulty: f64,
        ApproachRate: f64,
        SliderMultiplier: f64,
        SliderTickRate: f64,
    }

    pub struct Events {
        Background: PathBuf,
    }
}
