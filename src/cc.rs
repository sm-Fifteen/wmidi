use crate::byte::U7;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ControlFunction {
    /// Continuous Controller Data (MSB)
    BankSelect = 0,
    ModulationWheel = 1,
    BreathController = 2,
    Undefined3 = 3,
    FootController = 4,
    PortamentoTime = 5,
    /// Data entry is 6, 38, 96, 97, 98, 99, 100 and 101
    DataEntryMSB = 6,
    ChannelVolume = 7,
    Balance = 8,
    Undefined9 = 9,
    Pan = 10,
    ExpressionController = 11,
    EffectControl1 = 12,
    EffectControl2 = 13,
    Undefined14 = 14,
    Undefined15 = 15,
    GeneralPurposeController1 = 16,
    GeneralPurposeController2 = 17,
    GeneralPurposeController3 = 18,
    GeneralPurposeController4 = 19,
    Undefined20 = 20,
    Undefined21 = 21,
    Undefined22 = 22,
    Undefined23 = 23,
    Undefined24 = 24,
    Undefined25 = 25,
    Undefined26 = 26,
    Undefined27 = 27,
    Undefined28 = 28,
    Undefined29 = 29,
    Undefined30 = 30,
    Undefined31 = 31,
    /// Continuous Controller Data (LSB)
    BankSelectLSB = 32,
    ModulationWheelLSB = 33,
    BreathControllerLSB = 34,
    Undefined3LSB = 35,
    FootControllerLSB = 36,
    PortamentoTimeLSB = 37,
    DataEntryLSB = 38,
    ChannelVolumeLSB = 39,
    BalanceLSB = 40,
    Undefined9LSB = 41,
    PanLSB = 42,
    ExpressionControllerLSB = 43,
    EffectControl1LSB = 44,
    EffectControl2LSB = 45,
    Undefined14LSB = 46,
    Undefined15LSB = 47,
    GeneralPurposeController1LSB = 48,
    GeneralPurposeController2LSB = 49,
    GeneralPurposeController3LSB = 50,
    GeneralPurposeController4LSB = 51,
    Undefined20LSB = 52,
    Undefined21LSB = 53,
    Undefined22LSB = 54,
    Undefined23LSB = 55,
    Undefined24LSB = 56,
    Undefined25LSB = 57,
    Undefined26LSB = 58,
    Undefined27LSB = 59,
    Undefined28LSB = 60,
    Undefined29LSB = 61,
    Undefined30LSB = 62,
    Undefined31LSB = 63,

    // Additional single-byte controllers
    DamperPedal = 64, // sustain
    PortamentoOnOff = 65,
    Sostenuto = 66,
    SoftPedal = 67,
    LegatoFootswitch = 68,  // vv = 00-3F:Normal, 40-7F=Legatto
    Hold2 = 69,             // Hold1 would be 64
    SoundController1 = 70,  // default: Sound Variation
    SoundController2 = 71,  // default: Timbre/Harmonic Intensity
    SoundController3 = 72,  // default: Release Time
    SoundController4 = 73,  // default: Attack Time
    SoundController5 = 74,  // default: Brightness
    SoundController6 = 75,  // no default
    SoundController7 = 76,  // no default
    SoundController8 = 77,  // no default
    SoundController9 = 78,  // no default
    SoundController10 = 79, // no default
    GeneralPurposeController5 = 80,
    GeneralPurposeController6 = 81,
    GeneralPurposeController7 = 82,
    GeneralPurposeController8 = 83,
    PortamentoControl = 84,
    Undefined85 = 85,
    Undefined86 = 86,
    Undefined87 = 87,
    Undefined88 = 88,
    Undefined89 = 89,
    Undefined90 = 90,
    Effects1Depth = 91, // formerly External Effects Depth
    Effects2Depth = 92, // formerly Tremolo Depth
    Effects3Depth = 93, // formerly Chorus Depth
    Effects4Depth = 94, // formerly Celeste (Detune) Depth
    Effects5Depth = 95, // formerly Phaser Depth

    // Increment/Decrement and Parameter numbers
    DataIncrement = 96,                   // See Data Entry (6 & 38)
    DataDecrement = 97,                   // See Data Entry (6 & 38)
    NonRegisteredParameterNumberLSB = 98, // See Data Entry (6 & 38)
    NonRegisteredParameterNumberMSB = 99, // See Data Entry (6 & 38)
    RegisteredParameterNumberLSB = 100,   // See Data Entry (6 & 38)
    RegisteredParameterNumberMSB = 101,   // See Data Entry (6 & 38)

    // Undefined single-byte controllers
    Undefined102 = 102,
    Undefined103 = 103,
    Undefined104 = 104,
    Undefined105 = 105,
    Undefined106 = 106,
    Undefined107 = 107,
    Undefined108 = 108,
    Undefined109 = 109,
    Undefined110 = 110,
    Undefined111 = 111,
    Undefined112 = 112,
    Undefined113 = 113,
    Undefined114 = 114,
    Undefined115 = 115,
    Undefined116 = 116,
    Undefined117 = 117,
    Undefined118 = 118,
    Undefined119 = 119,

    /// CHANNEL MODE MESSAGES
    /// Channel must always be the instrument's Basic channel
    /// Affect the entire instrument
    AllSoundOff = 120, // Mute all sound playing (and maybe lights)
    ResetAllControllers = 121,
    LocalControl = 122, // Whether the instrument should react to its own input (0 = off, 127 = on)
    AllNotesOff = 123, // Mute all notes from MIDI-in (not Local Control), notes will keep playing if DamperPedal is on
    /// All the following also double as AllNotesOff
    /// Recognize sound from all channels
    OmniModeOn = 124,
    /// Only recognize sound from basic channel
    OmniModeOff = 125,
    /// (Poly off) One note per channel (val is how many channels to use, 0 means auto)
    MonoOperation = 126,
    /// (Mono off) One note per key per channel
    PolyOperation = 127,
}

impl From<U7> for ControlFunction {
    fn from(data: U7) -> ControlFunction {
        unsafe { std::mem::transmute(data) }
    }
}

impl From<ControlFunction> for U7 {
    fn from(control_function: ControlFunction) -> U7 {
        let data = control_function as u8;
        unsafe { U7::from_unchecked(data) }
    }
}

impl From<ControlFunction> for u8 {
    fn from(control_function: ControlFunction) -> u8 {
        control_function as u8
    }
}