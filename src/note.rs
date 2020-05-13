use crate::error::Error;
use alloc::string::String;
use core::convert::TryFrom;
use libm::powf;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[repr(u8)]
pub enum Note {
    CMinus1,
    CSharpMinus1,
    DMinus1,
    DSharpMinus1,
    EMinus1,
    FMinus1,
    FSharpMinus1,
    GMinus1,
    GSharpMinus1,
    AMinus1,
    ASharpMinus1,
    BMinus1,
    C0,
    CSharp0,
    D0,
    DSharp0,
    E0,
    F0,
    FSharp0,
    G0,
    GSharp0,
    A0,
    ASharp0,
    B0,
    C1,
    CSharp1,
    D1,
    DSharp1,
    E1,
    F1,
    FSharp1,
    G1,
    GSharp1,
    A1,
    ASharp1,
    B1,
    C2,
    CSharp2,
    D2,
    DSharp2,
    E2,
    F2,
    FSharp2,
    G2,
    GSharp2,
    A2,
    ASharp2,
    B2,
    C3,
    CSharp3,
    D3,
    DSharp3,
    E3,
    F3,
    FSharp3,
    G3,
    GSharp3,
    A3,
    ASharp3,
    B3,
    C4,
    CSharp4,
    D4,
    DSharp4,
    E4,
    F4,
    FSharp4,
    G4,
    GSharp4,
    A4,
    ASharp4,
    B4,
    C5,
    CSharp5,
    D5,
    DSharp5,
    E5,
    F5,
    FSharp5,
    G5,
    GSharp5,
    A5,
    ASharp5,
    B5,
    C6,
    CSharp6,
    D6,
    DSharp6,
    E6,
    F6,
    FSharp6,
    G6,
    GSharp6,
    A6,
    ASharp6,
    B6,
    C7,
    CSharp7,
    D7,
    DSharp7,
    E7,
    F7,
    FSharp7,
    G7,
    GSharp7,
    A7,
    ASharp7,
    B7,
    C8,
    CSharp8,
    D8,
    DSharp8,
    E8,
    F8,
    FSharp8,
    G8,
    GSharp8,
    A8,
    ASharp8,
    B8,
    C9,
    CSharp9,
    D9,
    DSharp9,
    E9,
    F9,
    FSharp9,
    G9,
}

impl Note {
    /// Get the frequency in Hertz of a note.
    ///
    /// # Examples
    /// ```rust
    /// use whatthechord::note::Note;
    ///
    /// // Lowest MIDI note (C-1) is 8.176Hz
    /// let lowest_freq = Note::CMinus1.frequency();
    /// assert_eq!(lowest_freq, 8.175798f32);
    ///
    /// // Middle C (C4) is 261.623Hz
    /// let c4_freq = Note::C4.frequency();
    /// assert_eq!(c4_freq, 261.62555f32);
    ///
    /// // Concert pitch (A4) is 440.000Hz
    /// let a4_freq = Note::A4.frequency();
    /// assert_eq!(a4_freq, 440f32);
    ///
    /// // C8 is 4186.009Hz
    /// let c8_freq = Note::C8.frequency();
    /// assert_eq!(c8_freq, 4186.009f32);
    ///
    /// // Highest MIDI note is 12543.855Hz
    /// let highest_freq = Note::G9.frequency();
    /// assert_eq!(highest_freq, 12543.855f32);
    /// ```
    pub fn frequency(self) -> f32 {
        let midi_key_number = f32::from(self.midi_key_number());
        let relative_to_concert_pitch = midi_key_number - 69f32;
        let octaved = relative_to_concert_pitch / 12f32;

        440f32 * powf(2f32, octaved)
    }

    /// Get the musician-friendly name of a note.
    ///
    /// # Examples
    /// ```rust
    /// use whatthechord::note::{Note, FlatOrSharp::*};
    ///
    /// // Natural C1 is "C1"
    /// let c1_name = Note::C1.name(Sharp);
    /// assert_eq!(c1_name, String::from("C1"));
    ///
    /// // Sharp F1 is "F#1"
    /// let f_sharp1_name = Note::FSharp1.name(Sharp);
    /// assert_eq!(f_sharp1_name, String::from("F#1"));
    ///
    /// // Sharp F1 is also "Gb1"
    /// let g_flat1_name = Note::FSharp1.name(Flat);
    /// assert_eq!(g_flat1_name, String::from("Gb1"));
    ///
    /// // Sharp C9 is "C#9"
    /// let c_sharp9_name = Note::CSharp9.name(Sharp);
    /// assert_eq!(c_sharp9_name, String::from("C#9"));
    ///
    /// // Sharp C9 is also "Db9"
    /// let c_sharp9_name = Note::CSharp9.name(Sharp);
    /// assert_eq!(c_sharp9_name, String::from("C#9"));
    /// ```
    pub fn name(self, accidental: FlatOrSharp) -> String {
        use FlatOrSharp::*;
        let (transpose_half_tones, accidental) = match (self.is_sharp(), accidental) {
            (true, Flat) => (1i8, "b"),
            (true, Sharp) => (0i8, "#"),
            (false, _) => (0i8, ""),
        };
        let tone = self.tone_name(transpose_half_tones);
        let octave = self.octave();

        format!("{}{}{}", tone, accidental, octave)
    }

    /// Get the tone name (one of "A", "B", "C", "D", "E", "F" or "G") of a note.
    ///
    /// # Examples
    /// ```rust
    /// use whatthechord::note::Note;
    ///
    /// // Natural C1 tone name is 'C'
    /// let c1_tone_name = Note::C1.tone_name(0);
    /// assert_eq!(c1_tone_name, 'C');
    ///
    /// // Sharp C1 tone name is 'C'
    /// let c_sharp1_tone_name = Note::CSharp1.tone_name(0);
    /// assert_eq!(c_sharp1_tone_name, 'C');
    ///
    /// // Sharp C1 tone name becomes 'D' if we think of it as a flat note instead of sharp note.
    /// let c_sharp1_tone_name = Note::CSharp1.tone_name(1);
    /// assert_eq!(c_sharp1_tone_name, 'D');
    /// ```
    pub fn tone_name(self, transpose_half_tones: i8) -> char {
        // Unwrap is OK because Note has no more than 128 items
        let midi_key_number = i8::try_from(self.midi_key_number()).unwrap();
        let relative_to_a = (midi_key_number + transpose_half_tones + 3) % 12;
        let char_offset = if relative_to_a < 7 && relative_to_a % 2 == 0 {
            relative_to_a / 2
        } else {
            relative_to_a / 2 + 1
        };

        // Unwrap is OK because `char_offset` is always < 7
        u8::try_from(char_offset)
            .map(|char_offset| char::from(65 + char_offset))
            .unwrap()
    }

    pub fn octave(self) -> u8 {
        let midi_key_number = self.midi_key_number();

        midi_key_number / 12 - 1
    }

    pub fn is_sharp(self) -> bool {
        let midi_key_number = self.midi_key_number();
        let position_in_octave = midi_key_number % 12;

        position_in_octave == 1
            || position_in_octave == 3
            || position_in_octave == 6
            || position_in_octave == 8
            || position_in_octave == 10
    }

    pub fn is_flat(self) -> bool {
        self.is_sharp()
    }

    /// Get the MIDI key number (0-127) of a note.
    ///
    /// # Examples
    /// ```rust
    /// use whatthechord::note::Note;
    ///
    /// // MIDI goes more than 1 octave lower than the regular piano keyboard
    /// let lowest = Note::CMinus1.midi_key_number();
    /// assert_eq!(lowest, 0);
    ///
    /// // C4, also known as "middle C", should be `60`
    /// let middle_c = Note::C4.midi_key_number();
    /// assert_eq!(middle_c, 60);
    ///
    /// // A4, also known as "concert pitch", should be `69`
    /// let middle_c = Note::A4.midi_key_number();
    /// assert_eq!(middle_c, 69);
    ///
    /// // The highest MIDI note is G9, which goes more than one octave beyond the range of a piano
    /// let highest = Note::G9.midi_key_number();
    /// assert_eq!(highest, 127)
    /// ```
    pub fn midi_key_number(self) -> u8 {
        self as u8
    }

    /// Get position (1-61) of a note in a 61-keys organ, or None if the note is not in the keyboard.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use whatthechord::{error::Error::*, note::Note};
    ///
    /// // C2 is the first note in a 61-keys organ keyboard
    /// let organ_c2 = Note::C2.organ_key_number();
    /// assert_eq!(organ_c2, Ok(1));
    ///
    /// // C1 does not exist in a 61-keys organ keyboard
    /// let organ_c1 = Note::C1.organ_key_number();
    /// assert_eq!(organ_c1, Err(OutOfInstrumentRange));
    ///
    /// // C7 is the highest note in a 61-keys organ keyboard
    /// let organ_c2 = Note::C7.organ_key_number();
    /// assert_eq!(organ_c2, Ok(61));
    ///
    /// // C8 does not exist in a 61-keys organ keyboard
    /// let organ_c1 = Note::C8.organ_key_number();
    /// assert_eq!(organ_c1, Err(OutOfInstrumentRange));
    /// ```
    ///
    pub fn organ_key_number(self) -> Result<u8, Error> {
        self.midi_key_number()
            .checked_sub(35)
            .ok_or(Error::OutOfInstrumentRange)
            .and_then(|number| {
                if number <= 61 {
                    Ok(number)
                } else {
                    Err(Error::OutOfInstrumentRange)
                }
            })
    }

    /// Creates a new note that is transposed a number of half tones up or down with respect to
    /// this note.
    ///
    /// # Examples
    /// ```rust
    /// use whatthechord::{error::Error::*, note::Note};
    ///
    /// // Natural C1 becomes sharp C1 when transposed half a tone up
    /// let c1_plus_1 = Note::C1.transposed(1).unwrap();
    /// assert_eq!(c1_plus_1, Note::CSharp1);
    ///
    /// // Sharp C1 becomes natural D1 when transposed half a tone up
    /// let c_sharp1_plus_1 = Note::CSharp1.transposed(1).unwrap();
    /// assert_eq!(c_sharp1_plus_1, Note::D1);
    ///
    /// // Natural C1 becomes natural B0 when transposed half a tone down
    /// let c1_minus_1 = Note::C1.transposed(-1).unwrap();
    /// assert_eq!(c1_minus_1, Note::B0);
    ///
    /// // Natural B0 becomes natural C1 when transposed half a tone up
    /// let b0_plus_1 = Note::B0.transposed(1).unwrap();
    /// assert_eq!(b0_plus_1, Note::C1);
    ///
    /// // Natural C1 becomes natural C2 when transposed a whole octave up
    /// let c1_plus_octave = Note::C1.transposed(12).unwrap();
    /// assert_eq!(c1_plus_octave, Note::C2);
    ///
    /// // Sharp C1 becomes sharp C2 when transposed a whole octave up
    /// let c_sharp1_plus_octave = Note::CSharp1.transposed(12).unwrap();
    /// assert_eq!(c_sharp1_plus_octave, Note::CSharp2);
    ///
    /// // You cannot "underflow" the MIDI keyboard when transposing
    /// let concert_a_minus_127 = Note::A4.transposed(-127).unwrap_err();
    /// assert_eq!(concert_a_minus_127, OutOfMIDIRange);
    ///
    /// // You cannot "overflow" the MIDI keyboard when transposing
    /// let concert_a_plus_127 = Note::A4.transposed(127).unwrap_err();
    /// assert_eq!(concert_a_plus_127, OutOfMIDIRange);
    /// ```
    pub fn transposed(self, half_tones: i8) -> Result<Self, Error> {
        let base_midi_key_number =
            i8::try_from(self.midi_key_number()).map_err(|_| Error::OutOfMIDIRange)?;

        let transposed_midi_key_number = base_midi_key_number
            .checked_add(half_tones)
            .ok_or(Error::OutOfMIDIRange)?;

        u8::try_from(transposed_midi_key_number)
            .map(Self::from)
            .map_err(|_| Error::OutOfMIDIRange)
    }

    /// Get position (1-88) of a note in an 88-keys piano, or None if the note is not in the
    /// keyboard.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use whatthechord::{error::Error::*, note::Note};
    ///
    /// // A0 is the first note in an 88-keys piano keyboard
    /// let piano_a0 = Note::A0.piano_key_number();
    /// assert_eq!(piano_a0, Ok(1));
    ///
    /// // C0 does not exist in an 88-keys piano keyboard
    /// let piano_c0 = Note::C0.piano_key_number();
    /// assert_eq!(piano_c0, Err(OutOfInstrumentRange));
    ///
    /// // C8 is the highest note in an 88-keys piano keyboard
    /// let piano_c8 = Note::C8.piano_key_number();
    /// assert_eq!(piano_c8, Ok(88));
    ///
    /// // C9 does not exist in an 88-keys piano keyboard
    /// let piano_c9 = Note::C9.piano_key_number();
    /// assert_eq!(piano_c9, Err(OutOfInstrumentRange));
    /// ```
    ///
    pub fn piano_key_number(self) -> Result<u8, Error> {
        self.midi_key_number()
            .checked_sub(20)
            .ok_or(Error::OutOfInstrumentRange)
            .and_then(|number| {
                if number <= 88 {
                    Ok(number)
                } else {
                    Err(Error::OutOfInstrumentRange)
                }
            })
    }
}

/// Support for creating a `Note` item from its MIDI key number as `u8`.
impl From<u8> for Note {
    fn from(value: u8) -> Self {
        unsafe { core::mem::transmute(value) }
    }
}

/// Flag for telling whether a note with accidentals should be called flat ("b") or sharp ("#").
pub enum FlatOrSharp {
    /// Flat notes take the name of the natural tone above.
    Flat,
    /// Sharp notes take the name of the natural tone below.
    Sharp,
}
