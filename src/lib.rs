use std::fs;

mod parser {

    #[derive(Debug)]
    pub enum GamemodeType {
        Osu = 0,
        Taiko = 1,
        Ctb = 2,
        Mania = 3,
    }

    #[derive(Debug)]
    pub struct Beatmap {
        pub version: i32,
        pub general: GeneralSection,
        pub editor: EditorSection,
        pub metadata: MetadataSection,
        pub difficulty: DifficultySection,
        pub timing_points: Vec<TimingPoint>,
        pub hit_objects: Vec<HitObjectType>,
    }

    #[derive(Debug)]
    pub struct GeneralSection {
        pub audio_filename: String,
        pub audio_lead_in: i32,
        pub epilepsy_warning: bool,
        pub letterbox_in_breaks: bool,
        pub mode: GamemodeType,
        pub preview_time: i32,
        pub sample_set: String,
        pub samples_match_playback_rate: bool,
        pub skin_preference: String,
        pub special_style: bool,
        pub stack_leniency: f64,
        pub use_skin_sprites: bool,
        pub widescreen_storyboard: bool,
    }

    impl Default for GeneralSection {
        fn default() -> Self {
            GeneralSection {
                audio_filename: String::new(),
                audio_lead_in: 0,
                preview_time: -1,
                sample_set: String::new(),
                stack_leniency: 0.0,
                mode: GamemodeType::Osu,
                letterbox_in_breaks: false,
                use_skin_sprites: false,
                skin_preference: String::new(),
                epilepsy_warning: false,
                special_style: false,
                widescreen_storyboard: false,
                samples_match_playback_rate: false,
            }
        }
    }

    #[derive(Debug)]
    pub struct EditorSection {
        pub bookmarks: Vec<i32>,
        pub distance_spacing: f64,
        pub beat_divisor: i32,
        pub grid_size: i32,
        pub timeline_zoom: f64,
    }

    impl Default for EditorSection {
        fn default() -> Self {
            EditorSection {
                bookmarks: Vec::new(),
                distance_spacing: 1.22,
                beat_divisor: 4,
                grid_size: 4,
                timeline_zoom: 1.0,
            }
        }
    }

    #[derive(Debug)]
    pub struct MetadataSection {
        pub title: String,
        pub title_unicode: String,
        pub artist: String,
        pub artist_unicode: String,
        pub creator: String,
        pub version: String,
        pub source: String,
        pub tags: Vec<String>,
        pub beatmap_ID: i32,
        pub beatmap_set_ID: i32,
    }

    impl Default for MetadataSection {
        fn default() -> Self {
            MetadataSection {
                title: String::new(),
                title_unicode: String::new(),
                artist: String::new(),
                artist_unicode: String::new(),
                creator: String::new(),
                version: String::new(),
                source: String::new(),
                tags: Vec::new(),
                beatmap_ID: 0,
                beatmap_set_ID: 0,
            }
        }
    }

    #[derive(Debug)]
    pub struct DifficultySection {
        pub health_drain_rate: f64,
        pub circle_size: f64,
        pub overall_difficulty: f64,
        pub approach_rate: f64,
        pub slider_multiplier: f64,
        pub slider_tick_rate: f64,
    }

    impl Default for DifficultySection {
        fn default() -> Self {
            DifficultySection {
                health_drain_rate: 5.0,
                circle_size: 4.0,
                overall_difficulty: 7.0,
                approach_rate: 9.0,
                slider_multiplier: 1.4,
                slider_tick_rate: 1.0,
            }
        }
    }

    #[derive(Debug)]
    pub struct TimingPoint {
        // offset from the start of the song
        pub time_offset: i32,
        // if inherited 0 = ms per beat
        // 333.33 = 1/333.33 * 1000 * 60 = 180bpm
        // if inherited 1 = negative inverse slider velocity multiplier as %
        // -25 = SV * (-100/-25) = SV * 4
        pub beat_length: f64,
        pub beats_per_measure: i32,
        pub sample_set: i32,
        pub sample_index: i32,
        pub volume: i32,
        pub inherited: bool,
        pub kiai_effects: bool,
    }

    impl Default for TimingPoint {
        fn default() -> Self {
            TimingPoint {
                time_offset: 0,
                beat_length: 300.0,
                beats_per_measure: 4,
                // overrides
                sample_set: 0,
                sample_index: 0,
                volume: 100,
                inherited: false,
                kiai_effects: false,
            }
        }
    }

    #[derive(Debug)]
    pub enum HitObjectType {
        HitCircle(HitCircle),
        Slider(Slider),
        Spinner(Spinner),
        HoldNote(HoldNote),
    }
    #[derive(Debug)]
    pub struct HitCircle {
        pub x: i32,
        pub y: i32,
        pub time: i32,
        pub new_combo: bool,
        pub color_skip: i32,
        pub hit_sound: i32,
    }

    #[derive(Debug)]
    pub enum SliderCurve {
        Bezier,
        Catmull, // used in old maps
        Linear,
        PerfectCircle,
    }

    #[derive(Debug)]
    pub struct Slider {
        pub x: i32,
        pub y: i32,
        pub time: i32,
        pub new_combo: bool,
        pub color_skip: i32,
        pub hitsound: i32,
        pub slider_type: SliderCurve,
        pub curve_points: Vec<(i32, i32)>,
        pub repeat: i32,
        pub pixel_length: f32,
    }

    #[derive(Debug)]
    pub struct Spinner {
        pub x: i32,
        pub y: i32,
        pub new_combo: bool,
        pub color_skip: i32,
        pub time: i32,
        pub hitsound: i32,
        pub end_time: i32,
    }

    #[derive(Default, Debug)]
    pub struct HoldNote {
        pub x: i32,
        pub y: i32,
        pub new_combo: bool,
        pub color_skip: i32,
        pub time: i32,
        pub hitsound: i32,
        pub end_time: i32,
    }

    #[derive(Debug)]
    enum Section {
        General(GeneralSection),
        Editor(EditorSection),
        Metadata(MetadataSection),
        TimingPoints(Vec<TimingPoint>),
        HitObjects(Vec<HitObjectType>),
        Difficulty(DifficultySection),
        Colours,
        Events,
        None,
    }
}
