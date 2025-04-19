//! Parse user keybindings

use std::str::FromStr;

use iced::keyboard::{Modifiers, key::Key as IcedKey};
use strum::IntoEnumIterator;

/// A sequence of 2 keys. If there are 2 keys like so:
/// - (T, None)
/// - (T, Some(X))
///
/// The 2nd key will never be triggered.
/// We will first search the `HashMap` of keys for the first key.
/// If it does not exist, search for the 2nd key.
#[derive(Debug, Hash, PartialEq, PartialOrd, Ord, Eq)]
pub struct KeySequence(pub (IcedKey, Option<IcedKey>));

/// Modifier keys
#[derive(Debug, Default)]
pub struct KeyMods(iced::keyboard::Modifiers);

impl FromStr for KeyMods {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut mods = iced::keyboard::Modifiers::empty();
        for modifier_str in s.split('+') {
            let modifier = match modifier_str {
                "shift" => Modifiers::SHIFT,
                "ctrl" => Modifiers::CTRL,
                "alt" => Modifiers::ALT,
                "super" | "windows" | "command" => Modifiers::LOGO,
                invalid => return Err(format!("Invalid modifier: {invalid}")),
            };
            if mods.contains(modifier) {
                return Err(format!("Duplicate modifier: {modifier_str}"));
            }
            mods.insert(modifier);
        }

        Ok(Self(mods))
    }
}

impl std::str::FromStr for KeySequence {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_key(key: &str) -> Result<IcedKey, String> {
            Named::from_str(key).map_or_else(
                |_| {
                    if key.len() == 1 {
                        Ok(IcedKey::Character(key.into()))
                    } else if key.len() == 2 {
                        Err(format!(
                            "Invalid key: {key}. Try to place a space in-between the keys: '{} {}'",
                            key.chars().next().expect("len == 2"),
                            key.chars().nth(1).expect("len == 2"),
                        ))
                    } else {
                        Err(format!("Invalid key: {key}"))
                    }
                },
                |key| Ok(IcedKey::Named(key.to_iced())),
            )
        }
        let mut first_key = None;
        for (i, key) in s.split_whitespace().enumerate() {
            if i >= 2 {
                return Err("At the moment, more than 2 keys are not supported".to_string());
            }
            if let Some(first_key) = first_key {
                return Ok(Self((first_key, Some(parse_key(key)?))));
            }
            first_key = Some(parse_key(key)?);
        }

        first_key.map_or_else(
            || Err("Expected at least 1 key".to_string()),
            |first_key| Ok(Self((first_key, None))),
        )
    }
}

/// Since Iced does not implement `FromStr` for `iced::keyboard::Key::Named`, we have to do this
macro_rules! named_keys {
    ( $($Key:ident),* $(,)?) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, strum::EnumString, strum::EnumIter, strum::IntoStaticStr)]
        #[strum(serialize_all = "kebab-case")]
        #[expect(
            clippy::upper_case_acronyms,
            reason = "do not deviate from Iced's name"
        )]
        pub enum Named {
            $(
                #[doc = concat!("The ", stringify!($Key), "key")]
                $Key
            ),*
        }
        impl Named {
            /// Convert this key to an Iced instance
            pub const fn to_iced(self) -> iced::keyboard::key::Named {
                match self {
                    $(Self::$Key => iced::keyboard::key::Named::$Key),*
                }
            }
        }
    };
}

named_keys! {
    Alt,
    AltGraph,
    CapsLock,
    Control,
    Fn,
    FnLock,
    NumLock,
    ScrollLock,
    Shift,
    Symbol,
    SymbolLock,
    Meta,
    Hyper,
    Super,
    Enter,
    Tab,
    Space,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    ArrowUp,
    End,
    Home,
    PageDown,
    PageUp,
    Backspace,
    Clear,
    Copy,
    CrSel,
    Cut,
    Delete,
    EraseEof,
    ExSel,
    Insert,
    Paste,
    Redo,
    Undo,
    Accept,
    Again,
    Attn,
    Cancel,
    ContextMenu,
    Escape,
    Execute,
    Find,
    Help,
    Pause,
    Play,
    Props,
    Select,
    ZoomIn,
    ZoomOut,
    BrightnessDown,
    BrightnessUp,
    Eject,
    LogOff,
    Power,
    PowerOff,
    PrintScreen,
    Hibernate,
    Standby,
    WakeUp,
    AllCandidates,
    Alphanumeric,
    CodeInput,
    Compose,
    Convert,
    FinalMode,
    GroupFirst,
    GroupLast,
    GroupNext,
    GroupPrevious,
    ModeChange,
    NextCandidate,
    NonConvert,
    PreviousCandidate,
    Process,
    SingleCandidate,
    HangulMode,
    HanjaMode,
    JunjaMode,
    Eisu,
    Hankaku,
    Hiragana,
    HiraganaKatakana,
    KanaMode,
    KanjiMode,
    Katakana,
    Romaji,
    Zenkaku,
    ZenkakuHankaku,
    Soft1,
    Soft2,
    Soft3,
    Soft4,
    ChannelDown,
    ChannelUp,
    Close,
    MailForward,
    MailReply,
    MailSend,
    MediaClose,
    MediaFastForward,
    MediaPause,
    MediaPlay,
    MediaPlayPause,
    MediaRecord,
    MediaRewind,
    MediaStop,
    MediaTrackNext,
    MediaTrackPrevious,
    New,
    Open,
    Print,
    Save,
    SpellCheck,
    Key11,
    Key12,
    AudioBalanceLeft,
    AudioBalanceRight,
    AudioBassBoostDown,
    AudioBassBoostToggle,
    AudioBassBoostUp,
    AudioFaderFront,
    AudioFaderRear,
    AudioSurroundModeNext,
    AudioTrebleDown,
    AudioTrebleUp,
    AudioVolumeDown,
    AudioVolumeUp,
    AudioVolumeMute,
    MicrophoneToggle,
    MicrophoneVolumeDown,
    MicrophoneVolumeUp,
    MicrophoneVolumeMute,
    SpeechCorrectionList,
    SpeechInputToggle,
    LaunchApplication1,
    LaunchApplication2,
    LaunchCalendar,
    LaunchContacts,
    LaunchMail,
    LaunchMediaPlayer,
    LaunchMusicPlayer,
    LaunchPhone,
    LaunchScreenSaver,
    LaunchSpreadsheet,
    LaunchWebBrowser,
    LaunchWebCam,
    LaunchWordProcessor,
    BrowserBack,
    BrowserFavorites,
    BrowserForward,
    BrowserHome,
    BrowserRefresh,
    BrowserSearch,
    BrowserStop,
    AppSwitch,
    Call,
    Camera,
    CameraFocus,
    EndCall,
    GoBack,
    GoHome,
    HeadsetHook,
    LastNumberRedial,
    Notification,
    MannerMode,
    VoiceDial,
    TV,
    TV3DMode,
    TVAntennaCable,
    TVAudioDescription,
    TVAudioDescriptionMixDown,
    TVAudioDescriptionMixUp,
    TVContentsMenu,
    TVDataService,
    TVInput,
    TVInputComponent1,
    TVInputComponent2,
    TVInputComposite1,
    TVInputComposite2,
    TVInputHDMI1,
    TVInputHDMI2,
    TVInputHDMI3,
    TVInputHDMI4,
    TVInputVGA1,
    TVMediaContext,
    TVNetwork,
    TVNumberEntry,
    TVPower,
    TVRadioService,
    TVSatellite,
    TVSatelliteBS,
    TVSatelliteCS,
    TVSatelliteToggle,
    TVTerrestrialAnalog,
    TVTerrestrialDigital,
    TVTimer,
    AVRInput,
    AVRPower,
    ColorF0Red,
    ColorF1Green,
    ColorF2Yellow,
    ColorF3Blue,
    ColorF4Grey,
    ColorF5Brown,
    ClosedCaptionToggle,
    Dimmer,
    DisplaySwap,
    DVR,
    Exit,
    FavoriteClear0,
    FavoriteClear1,
    FavoriteClear2,
    FavoriteClear3,
    FavoriteRecall0,
    FavoriteRecall1,
    FavoriteRecall2,
    FavoriteRecall3,
    FavoriteStore0,
    FavoriteStore1,
    FavoriteStore2,
    FavoriteStore3,
    Guide,
    GuideNextDay,
    GuidePreviousDay,
    Info,
    InstantReplay,
    Link,
    ListProgram,
    LiveContent,
    Lock,
    MediaApps,
    MediaAudioTrack,
    MediaLast,
    MediaSkipBackward,
    MediaSkipForward,
    MediaStepBackward,
    MediaStepForward,
    MediaTopMenu,
    NavigateIn,
    NavigateNext,
    NavigateOut,
    NavigatePrevious,
    NextFavoriteChannel,
    NextUserProfile,
    OnDemand,
    Pairing,
    PinPDown,
    PinPMove,
    PinPToggle,
    PinPUp,
    PlaySpeedDown,
    PlaySpeedReset,
    PlaySpeedUp,
    RandomToggle,
    RcLowBattery,
    RecordSpeedNext,
    RfBypass,
    ScanChannelsToggle,
    ScreenModeNext,
    Settings,
    SplitScreenToggle,
    STBInput,
    STBPower,
    Subtitle,
    Teletext,
    VideoModeNext,
    Wink,
    ZoomToggle,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    F13,
    F14,
    F15,
    F16,
    F17,
    F18,
    F19,
    F20,
    F21,
    F22,
    F23,
    F24,
    F25,
    F26,
    F27,
    F28,
    F29,
    F30,
    F31,
    F32,
    F33,
    F34,
    F35,
}
