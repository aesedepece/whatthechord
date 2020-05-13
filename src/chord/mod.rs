use crate::prelude::*;
use alloc::collections::BTreeSet;
use alloc::string::String;
use alloc::vec::Vec;

/// Separate functions for extracting information about intervals and different chord sizes.
pub mod guess;
/// Definitions for each chord type's own qualities (major, minor, augmented, etc).
pub mod qualities;

/// From Wikipedia: A set of notes that are heard as if sounding simultaneously.
#[derive(Debug, Eq, PartialEq)]
pub struct Chord {
    intervals: Vec<u8>,
    chord_type: ChordType,
    notes: Vec<Note>,
    root: Option<Note>,
    additions: Option<Vec<Note>>,
}

/// Convenient methods for working with musical chords.
impl Chord {
    /// Build a chord from a set of notes.
    ///
    /// # Examples
    /// ```rust
    /// use whatthechord::prelude::{*, Note::*};
    ///
    /// // A chord can be constructed from any set of notes
    /// let c_major_notes = [C1, E1, G1];
    /// let c_major_chord = Chord::from_notes(&c_major_notes);
    /// ```
    pub fn from_notes(notes: &[Note]) -> Self {
        // Go `&[Note]` -> `BTreeSet<&Note>` -> `Vec<Note>` to ensure note uniqueness.
        let notes = notes
            .iter()
            .collect::<BTreeSet<&Note>>()
            .iter()
            .cloned()
            .cloned()
            .collect::<Vec<Note>>();

        // Compute note intervals
        let intervals = guess::intervals(&notes);

        // Handle each chord size separately
        match &notes.len() {
            // No notes, only silence
            0 => Chord::default(),
            // Single note
            1 => {
                let root = notes.get(0).cloned();

                Chord {
                    intervals,
                    chord_type: ChordType::SingleNote,
                    notes,
                    root,
                    additions: None,
                }
            }
            2 => guess::dyad(notes, intervals),
            // Triad
            3 => guess::triad(&notes, &intervals),
            // Tetrad
            4 => guess::tetrad(&notes, &intervals),
            // Anything else not looking like a proper chord that is worth naming
            _ => Chord {
                intervals,
                chord_type: ChordType::Unknown,
                notes,
                root: None,
                additions: None,
            },
        }
    }

    /// Retrieve the intervals in a chord.
    ///
    /// # Examples
    /// ```rust
    /// use whatthechord::prelude::{*, Note::*};
    ///
    /// // There is no interval in silence
    /// let silence_interval = Chord::default().intervals();
    /// assert_eq!(silence_interval, vec![]);
    ///
    /// // There is no interval in a single-note chord
    /// let c1_interval = Chord::from_notes(&[C1]).intervals();
    /// assert_eq!(c1_interval, vec![]);
    ///
    /// // The intervals for a major chord should be [4, 7]
    /// let c_major_notes = [C1, E1, G1];
    /// let c_major_interval = Chord::from_notes(&c_major_notes).intervals();
    /// assert_eq!(c_major_interval, vec![4u8, 3u8]);
    ///
    /// ```
    pub fn intervals(&self) -> Vec<u8> {
        self.intervals.clone()
    }

    /// Tells whether the chord is actually a silence (has no notes in it)
    ///
    /// # Examples
    /// ```rust
    /// use whatthechord::prelude::{*, Note::*};
    ///
    /// // The default chord is a silence (has no notes in it)
    /// let default_is_silence = Chord::default().is_silence();
    /// assert_eq!(default_is_silence, true);
    ///
    /// // C1 is not a silence
    /// let c1_is_silence = Chord::from_notes(&[C1]).is_silence();
    /// assert_eq!(c1_is_silence, false);
    ///
    /// // C Major is not silence
    /// let c_major_notes = [C1, E1, G1];
    /// let c_major_is_silence = Chord::from_notes(&c_major_notes).is_silence();
    /// assert_eq!(c_major_is_silence, false);
    /// ```
    pub fn is_silence(&self) -> bool {
        self.notes.is_empty()
    }

    /// Tells whether the chord is actually not an harmony but a single note.
    ///
    /// # Examples
    /// ```rust
    /// use whatthechord::prelude::{*, Note::*};
    ///
    /// // The default chord is a silence (has no notes in it), so it is not a single note
    /// let default_is_single_note = Chord::default().is_single_note();
    /// assert_eq!(default_is_single_note, false);
    ///
    /// // C1 is a single note
    /// let c1_is_single_note = Chord::from_notes(&[C1]).is_single_note();
    /// assert_eq!(c1_is_single_note, true);
    ///
    /// // C Major is not a single note but a triad
    /// let c_major_notes = [C1, E1, G1];
    /// let c_major_is_single_note = Chord::from_notes(&c_major_notes).is_single_note();
    /// assert_eq!(c_major_is_single_note, false);
    /// ```
    pub fn is_single_note(&self) -> bool {
        self.notes.len() == 1
    }

    /// Tells whether the chord is a triad.
    ///
    /// # Examples
    /// ```rust
    /// use whatthechord::prelude::{*, Note::*};
    ///
    /// // The default chord is a silence (has no notes in it), so it is not a triad
    /// let default_is_triad = Chord::default().is_triad();
    /// assert_eq!(default_is_triad, false);
    ///
    /// // C1 is a single note, so it is not a triad
    /// let c1_is_triad = Chord::from_notes(&[C1]).is_triad();
    /// assert_eq!(c1_is_triad, false);
    ///
    /// // Cmaj is a triad
    /// let c_maj_notes = [C1, E1, G1];
    /// let c_maj_is_triad = Chord::from_notes(&c_maj_notes).is_triad();
    /// assert_eq!(c_maj_is_triad, true);
    ///
    /// // Cmaj/E is a triad
    /// let c_maj_e_notes = [E1, G1, C2];
    /// let c_maj_e_is_triad = Chord::from_notes(&c_maj_e_notes).is_triad();
    /// assert_eq!(c_maj_e_is_triad, true);
    ///
    /// // Cmaj/G is a triad
    /// let c_maj_g_notes = [G1, C2, E2];
    /// let c_maj_g_is_triad = Chord::from_notes(&c_maj_g_notes).is_triad();
    /// assert_eq!(c_maj_g_is_triad, true);
    ///
    /// // Cm is a triad
    /// let c_m_notes = [C1, DSharp1, G1];
    /// let c_m_is_triad = Chord::from_notes(&c_m_notes).is_triad();
    /// assert_eq!(c_m_is_triad, true);
    ///
    /// // Cm/D# is a triad
    /// let c_m_d_sharp_notes = [DSharp1, G1, C2];
    /// let c_m_d_sharp_is_triad = Chord::from_notes(&c_m_d_sharp_notes).is_triad();
    /// assert_eq!(c_m_d_sharp_is_triad, true);
    ///
    /// // Cm/G is a triad
    /// let c_m_g_notes = [G1, C2, DSharp2];
    /// let c_m_g_is_triad = Chord::from_notes(&c_m_g_notes).is_triad();
    /// assert_eq!(c_m_g_is_triad, true);
    ///
    /// // Cdim is a triad
    /// let c_dim_notes = [C1, DSharp1, FSharp1];
    /// let c_dim_is_triad = Chord::from_notes(&c_dim_notes).is_triad();
    /// assert_eq!(c_m_is_triad, true);
    ///
    /// // Cdim/D# is a triad
    /// let c_dim_d_sharp_notes = [DSharp1, FSharp1, C2];
    /// let c_dim_d_sharp_is_triad = Chord::from_notes(&c_dim_d_sharp_notes).is_triad();
    /// assert_eq!(c_dim_d_sharp_is_triad, true);
    ///
    /// // Cdim/F# is a triad
    /// let c_dim_f_sharp_notes = [FSharp1, C2, DSharp2];
    /// let c_dim_f_sharp_is_triad = Chord::from_notes(&c_dim_f_sharp_notes).is_triad();
    /// assert_eq!(c_dim_f_sharp_is_triad, true);
    /// ```
    pub fn is_triad(&self) -> bool {
        self.notes.len() == 3
    }

    /// Get the musician-friendly name of a chord.
    ///
    /// # Examples
    /// ```rust
    /// use whatthechord::prelude::{*, Note::*};
    ///
    /// let chord = Chord::from_notes(&[C1, E1, G1]);
    /// assert_eq!(chord.name(FlatOrSharp::Sharp).unwrap(), "C");
    ///
    /// let chord = Chord::from_notes(&[CSharp1, F1, GSharp1]);
    /// assert_eq!(chord.name(FlatOrSharp::Sharp).unwrap(), "C#");
    ///
    /// let chord = Chord::from_notes(&[CSharp1, F1, GSharp1]);
    /// assert_eq!(chord.name(FlatOrSharp::Flat).unwrap(), "Db");
    ///
    /// let chord = Chord::from_notes(&[C1, DSharp1, G1]);
    /// assert_eq!(chord.name(FlatOrSharp::Sharp).unwrap(), "Cm");
    ///
    /// let chord = Chord::from_notes(&[CSharp1, E1, GSharp1]);
    /// assert_eq!(chord.name(FlatOrSharp::Sharp).unwrap(), "C#m");
    ///
    /// let chord = Chord::from_notes(&[CSharp1, E1, GSharp1]);
    /// assert_eq!(chord.name(FlatOrSharp::Flat).unwrap(), "Dbm");
    /// ```
    pub fn name(&self, accidental: FlatOrSharp) -> Option<String> {
        let root = String::from(
            self.root?
                .name(accidental)
                .trim_end_matches(char::is_numeric),
        );
        let quality = match &self.chord_type {
            ChordType::Triad(quality) => format!("{}", quality),
            _ => String::new(),
        };

        Some(format!("{}{}", root, quality))
    }
}

/// A default, empty chord with no notes, aka "silence"
impl Default for Chord {
    fn default() -> Self {
        Self {
            intervals: vec![],
            chord_type: ChordType::Silence,
            notes: vec![],
            root: None,
            additions: None,
        }
    }
}

/// Different types of chords.
#[derive(Debug, Eq, PartialEq)]
pub enum ChordType {
    Complex(Vec<Chord>),
    Dyad(DyadQuality),
    Silence,
    SingleNote,
    Tetrad(TetradQuality),
    Triad(TriadQuality),
    Unknown,
}

#[cfg(test)]
mod tests {
    use crate::prelude::{ChordType::*, Note::*, *};
    use alloc::vec::Vec;

    #[test]
    fn test_major_triad_root_position() {
        let notes = [C1, E1, G1];
        let chord = Chord::from_notes(&notes);
        let expected = Chord {
            intervals: vec![4, 3],
            chord_type: Triad(TriadQuality::Major),
            notes: Vec::from(notes.as_ref()),
            root: Some(C1),
            additions: None,
        };

        assert_eq!(chord, expected);
    }

    #[test]
    fn test_major_triad_first_inversion() {
        let notes = [E1, G1, C2];
        let chord = Chord::from_notes(&notes);
        let expected = Chord {
            intervals: vec![3, 5],
            chord_type: Triad(TriadQuality::Major),
            notes: Vec::from(notes.as_ref()),
            root: Some(C2),
            additions: None,
        };

        assert_eq!(chord, expected);
    }

    #[test]
    fn test_major_triad_second_inversion() {
        let notes = [G1, C2, E2];
        let chord = Chord::from_notes(&notes);
        let expected = Chord {
            intervals: vec![5, 4],
            chord_type: Triad(TriadQuality::Major),
            notes: Vec::from(notes.as_ref()),
            root: Some(C2),
            additions: None,
        };

        assert_eq!(chord, expected);
    }

    #[test]
    fn test_minor_triad_root_position() {
        let notes = [C1, DSharp1, G1];
        let chord = Chord::from_notes(&notes);
        let expected = Chord {
            intervals: vec![3, 4],
            chord_type: Triad(TriadQuality::Minor),
            notes: Vec::from(notes.as_ref()),
            root: Some(C1),
            additions: None,
        };

        assert_eq!(chord, expected);
    }

    #[test]
    fn test_minor_triad_first_inversion() {
        let notes = [DSharp1, G1, C2];
        let chord = Chord::from_notes(&notes);
        let expected = Chord {
            intervals: vec![4, 5],
            chord_type: Triad(TriadQuality::Minor),
            notes: Vec::from(notes.as_ref()),
            root: Some(C2),
            additions: None,
        };

        assert_eq!(chord, expected);
    }

    #[test]
    fn test_minor_triad_second_inversion() {
        let notes = [G1, C2, DSharp2];
        let chord = Chord::from_notes(&notes);
        let expected = Chord {
            intervals: vec![5, 3],
            chord_type: Triad(TriadQuality::Minor),
            notes: Vec::from(notes.as_ref()),
            root: Some(C2),
            additions: None,
        };

        assert_eq!(chord, expected);
    }

    #[test]
    fn test_diminished_triad_root_position() {
        let notes = [C1, DSharp1, FSharp1];
        let chord = Chord::from_notes(&notes);
        let expected = Chord {
            intervals: vec![3, 3],
            chord_type: Triad(TriadQuality::Diminished),
            notes: Vec::from(notes.as_ref()),
            root: Some(C1),
            additions: None,
        };

        assert_eq!(chord, expected);
    }

    #[test]
    fn test_diminished_triad_first_inversion() {
        let notes = [DSharp1, FSharp1, C2];
        let chord = Chord::from_notes(&notes);
        let expected = Chord {
            intervals: vec![3, 6],
            chord_type: Triad(TriadQuality::Diminished),
            notes: Vec::from(notes.as_ref()),
            root: Some(C2),
            additions: None,
        };

        assert_eq!(chord, expected);
    }

    #[test]
    fn test_diminished_triad_second_inversion() {
        let notes = [FSharp1, C2, DSharp2];
        let chord = Chord::from_notes(&notes);
        let expected = Chord {
            intervals: vec![6, 3],
            chord_type: Triad(TriadQuality::Diminished),
            notes: Vec::from(notes.as_ref()),
            root: Some(C2),
            additions: None,
        };

        assert_eq!(chord, expected);
    }

    #[test]
    fn test_augmented_triad_root_position() {
        let notes = [C1, E1, GSharp1];
        let chord = Chord::from_notes(&notes);
        let expected = Chord {
            intervals: vec![4, 4],
            chord_type: Triad(TriadQuality::Augmented),
            notes: Vec::from(notes.as_ref()),
            root: Some(C1),
            additions: None,
        };

        assert_eq!(chord, expected);
    }

    #[test]
    fn test_suspended_2_triad_root_position() {
        let notes = [C1, D1, G1];
        let chord = Chord::from_notes(&notes);
        let expected = Chord {
            intervals: vec![2, 5],
            chord_type: Triad(TriadQuality::Suspended(2)),
            notes: Vec::from(notes.as_ref()),
            root: Some(C1),
            additions: None,
        };

        assert_eq!(chord, expected);
    }

    #[test]
    fn test_suspended_4_triad_root_position() {
        let notes = [C1, F1, G1];
        let chord = Chord::from_notes(&notes);
        let expected = Chord {
            intervals: vec![5, 2],
            chord_type: Triad(TriadQuality::Suspended(4)),
            notes: Vec::from(notes.as_ref()),
            root: Some(C1),
            additions: None,
        };

        assert_eq!(chord, expected);
    }

    #[test]
    fn test_suspended_4_triad_second_inversion() {
        let notes = [G1, C2, F2];
        let chord = Chord::from_notes(&notes);
        let expected = Chord {
            intervals: vec![5, 5],
            chord_type: Triad(TriadQuality::Suspended(4)),
            notes: Vec::from(notes.as_ref()),
            root: Some(C2),
            additions: None,
        };

        assert_eq!(chord, expected);
    }

    #[test]
    fn test_major_seven_tetrad_root_position() {
        let notes = [C1, E1, G1, B1];
        let chord = Chord::from_notes(&notes);
        let expected = Chord {
            intervals: vec![4, 3, 4],
            chord_type: Tetrad(TetradQuality::SeventhMajor),
            notes: Vec::from(notes.as_ref()),
            root: Some(C1),
            additions: None,
        };

        assert_eq!(chord, expected);
    }

    #[test]
    fn test_major_seven_tetrad_first_inversion() {
        let notes = [E1, G1, B1, C2];
        let chord = Chord::from_notes(&notes);
        let expected = Chord {
            intervals: vec![3, 4, 1],
            chord_type: Tetrad(TetradQuality::SeventhMajor),
            notes: Vec::from(notes.as_ref()),
            root: Some(C2),
            additions: None,
        };

        assert_eq!(chord, expected);
    }

    #[test]
    fn test_major_seven_tetrad_second_inversion() {
        let notes = [G1, B1, C2, E2];
        let chord = Chord::from_notes(&notes);
        let expected = Chord {
            intervals: vec![4, 1, 4],
            chord_type: Tetrad(TetradQuality::SeventhMajor),
            notes: Vec::from(notes.as_ref()),
            root: Some(C2),
            additions: None,
        };

        assert_eq!(chord, expected);
    }

    #[test]
    fn test_major_seven_tetrad_third_inversion() {
        let notes = [B1, C2, E2, G2];
        let chord = Chord::from_notes(&notes);
        let expected = Chord {
            intervals: vec![1, 4, 3],
            chord_type: Tetrad(TetradQuality::SeventhMajor),
            notes: Vec::from(notes.as_ref()),
            root: Some(C2),
            additions: None,
        };

        assert_eq!(chord, expected);
    }

    #[test]
    fn test_major_triad_with_subtone() {
        let notes = [C3, C4, E4, G4];
        let chord = Chord::from_notes(&notes);
        let expected = Chord {
            intervals: vec![4, 3],
            chord_type: Triad(TriadQuality::Major),
            notes: Vec::from(notes.as_ref()),
            root: Some(C4),
            additions: Some(vec![C3]),
        };

        assert_eq!(chord, expected);
    }

    #[test]
    fn test_major_triad_with_overtone() {
        let notes = [C4, E4, G4, C5];
        let chord = Chord::from_notes(&notes);
        let expected = Chord {
            intervals: vec![4, 3],
            chord_type: Triad(TriadQuality::Major),
            notes: Vec::from(notes.as_ref()),
            root: Some(C4),
            additions: Some(vec![C5]),
        };

        assert_eq!(chord, expected);
    }

    #[test]
    fn test_major_triad_with_major_second() {
        let notes = [C4, D4, E4, G4];
        let chord = Chord::from_notes(&notes);
        let expected = Chord {
            intervals: vec![4, 3],
            chord_type: Triad(TriadQuality::Major),
            notes: Vec::from(notes.as_ref()),
            root: Some(C4),
            additions: Some(vec![D4]),
        };

        assert_eq!(chord, expected);
    }

    #[test]
    fn test_major_triad_with_minor_second() {
        let notes = [C4, CSharp4, E4, G4];
        let chord = Chord::from_notes(&notes);
        let expected = Chord {
            intervals: vec![4, 3],
            chord_type: Triad(TriadQuality::Major),
            notes: Vec::from(notes.as_ref()),
            root: Some(C4),
            additions: Some(vec![CSharp4]),
        };

        assert_eq!(chord, expected);
    }

    #[test]
    fn test_major_triad_with_perfect_fourth() {
        let notes = [C4, E4, F4, G4];
        let chord = Chord::from_notes(&notes);
        let expected = Chord {
            intervals: vec![4, 3],
            chord_type: Triad(TriadQuality::Major),
            notes: Vec::from(notes.as_ref()),
            root: Some(C4),
            additions: Some(vec![F4]),
        };

        assert_eq!(chord, expected);
    }

    #[test]
    fn test_major_triad_with_augmented_fourth() {
        let notes = [C4, E4, FSharp4, G4];
        let chord = Chord::from_notes(&notes);
        let expected = Chord {
            intervals: vec![4, 3],
            chord_type: Triad(TriadQuality::Major),
            notes: Vec::from(notes.as_ref()),
            root: Some(C4),
            additions: Some(vec![FSharp4]),
        };

        assert_eq!(chord, expected);
    }
}
