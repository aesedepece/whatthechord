use crate::prelude::*;
use alloc::collections::BTreeSet;
use alloc::vec::Vec;

/// Try to find notes in a chord that don't belong to the intervals known for any of the
/// recognized chord qualities.
///
/// # Examples
/// ```rust
/// use whatthechord::prelude::*;
/// use whatthechord::chord::guess;
///
/// let notes = [Note::C1, Note::C2, Note::E2, Note::G2];
/// let chord = Chord::from_notes(&notes);
/// let additions = guess::additions(&notes, &chord.intervals());
/// assert_eq!(additions, vec![Note::C1]);
///
/// let notes = [Note::C0, Note::C2, Note::E2, Note::G2];
/// let chord = Chord::from_notes(&notes);
/// let additions = guess::additions(&notes, &chord.intervals());
/// assert_eq!(additions, vec![Note::C0]);
///
/// let notes = [Note::C1, Note::E1, Note::G1, Note::C2];
/// let chord = Chord::from_notes(&notes);
/// let additions = guess::additions(&notes, &chord.intervals());
/// assert_eq!(additions, vec![Note::C2]);
///
/// let notes = [Note::C1, Note::E1, Note::G1, Note::C3];
/// let chord = Chord::from_notes(&notes);
/// let additions = guess::additions(&notes, &chord.intervals());
/// assert_eq!(additions, vec![Note::C3]);
///
/// let notes = [Note::C1, Note::E1, Note::F1, Note::G1];
/// let chord = Chord::from_notes(&notes);
/// let additions = guess::additions(&notes, &chord.intervals());
/// assert_eq!(additions, vec![Note::F1]);
///
/// let notes = [Note::C1, Note::E1, Note::G1];
/// let chord = Chord::from_notes(&notes);
/// let additions = guess::additions(&notes, &chord.intervals());
/// assert_eq!(additions, vec![]);
/// ```
pub fn additions(notes: &[Note], intervals: &[u8]) -> Vec<Note> {
    let mut additions;

    let first_note_number = notes[0].midi_key_number();

    let raw_notes = notes
        .iter()
        .skip(1)
        .map(|x| x.midi_key_number())
        .collect::<BTreeSet<u8>>();

    let chord_notes = intervals
        .iter()
        .scan(first_note_number, |prev, interval| {
            *prev += *interval;

            Some(*prev)
        })
        .collect::<BTreeSet<u8>>();

    additions = raw_notes
        .difference(&chord_notes)
        .map(|note_number| Note::from(*note_number))
        .collect::<Vec<Note>>();

    if additions.len() == notes.len() - 1 {
        additions = vec![notes[0]];
        additions.append(&mut guess::additions(&notes[1..], intervals));
    }

    additions
}

/// Compute the intervals of a set of notes, relative to the bass note (the note in the set
/// having the lowest pitch).
///
/// # Examples
/// ```rust
/// use whatthechord::prelude::*;
/// use whatthechord::chord::guess::intervals;
///
/// let notes = [Note::C1, Note::E1, Note::G1];
/// let intervals = intervals(&notes);
/// assert_eq!(intervals, vec![4, 3]);
/// ```
pub fn intervals(notes: &[Note]) -> Vec<u8> {
    let mut intervals = Vec::new();
    let mut notes = notes.iter();

    // Compute intervals only if there is at least one note in the chord
    if let Some(bass_note) = notes.next() {
        // Get the first tone and insert a `0` interval representing the bass note
        let mut prev_key_number = bass_note.midi_key_number();
        // For all other notes, compare each tones to the previous one, and insert intervals
        for note in notes {
            let cur_key_number = note.midi_key_number();
            intervals.push(cur_key_number - prev_key_number);
            prev_key_number = cur_key_number;
        }
    }

    intervals
}

/// Extract information about a dyad (a set of two notes).
pub(crate) fn dyad(notes: Vec<Note>, intervals: Vec<u8>) -> Chord {
    use DyadQuality::*;

    let dyad_type = match intervals[1] {
        0 => Perfect(0),      // P1  d2
        1 => Augmented(1),    // A1  m2
        2 => Major(2),        // M2  d3
        3 => Minor(3),        // m3  A2
        4 => Major(3),        // M3  d4
        5 => Perfect(4),      // P4  A3
        6 => Augmented(4),    // A4  d5
        7 => Perfect(5),      // P5  d6
        8 => Minor(6),        // m6  A5
        9 => Major(6),        // M6  d7
        10 => Minor(7),       // m7  A6
        11 => Major(7),       // M7  d8
        12 => Diminished(9),  // d9
        13 => Minor(9),       // m9  A8
        14 => Major(9),       // M9  d10
        15 => Minor(10),      // m10 A9
        16 => Major(10),      // M10 d11
        17 => Perfect(11),    // P11 A10
        18 => Diminished(12), // d12 A11
        19 => Perfect(12),    // P12 d13
        20 => Minor(13),      // m13 A12
        21 => Major(13),      // M13 d14
        22 => Minor(14),      // m14 A13
        23 => Major(14),      // M14 d15
        24 => Perfect(15),    // P15 A14
        25 => Augmented(15),  // A15
        _ => Indeterminate,
    };

    let root = notes.get(0).cloned();

    Chord {
        intervals,
        chord_type: ChordType::Dyad(dyad_type),
        notes,
        root,
        additions: None,
    }
}

/// Extract information about a triad (a set of three notes).
pub(crate) fn triad(notes: &[Note], intervals: &[u8]) -> Chord {
    use super::TriadQuality::*;

    // Interval between the topmost note and the first inversion of the root
    let complementary_interval = 12 - intervals[0] - intervals[1];

    // In each iteration of the loop, we try to match the intervals against different inversions
    // of the intervals associated to each triad quality.
    let mut root_guess = 0;
    let quality = loop {
        // Try one inversion each time, looking for the natural intervals / root position
        let natural_interval = match root_guess {
            0 => (intervals[0], intervals[1]),           // Root position
            1 => (intervals[1], complementary_interval), // 2nd inversion
            2 => (complementary_interval, intervals[0]), // 1st inversion
            _ => break Indeterminate,
        };

        // Find the quality that matches our root position guess
        let quality = match natural_interval {
            (4, 3) => Major,
            (3, 4) => Minor,
            (3, 3) => Diminished,
            (4, 4) => Augmented,
            (5, 2) => Suspended(4),
            (2, 5) => Suspended(2),
            _ => Indeterminate,
        };

        if quality != Indeterminate {
            break quality;
        } else {
            root_guess += 1;
        }
    };

    let root = Some(notes[root_guess]);

    Chord {
        intervals: Vec::from(intervals),
        chord_type: ChordType::Triad(quality),
        notes: Vec::from(notes),
        root,
        additions: None,
    }
}

/// Extract information about a triad (a set of four notes).
pub(crate) fn tetrad(notes: &[Note], intervals: &[u8]) -> Chord {
    use super::TetradQuality::*;

    // Try to identify tetrads that are actually a triad plus additions or a couple of dyads.
    match intervals[0..3] {
        // Additional bass tone
        [bass, third, fifth] if bass % 12 == 0 => {
            let intervals = vec![third, fifth];
            let mut chord = triad(&notes[1..], &intervals);
            chord.additions = Some(guess::additions(&notes, &intervals));
            chord.notes = Vec::from(notes);

            return chord;
        }
        // Additional overtone
        [third, fifth, add] if (third + fifth + add) % 12 == 0 => {
            let intervals = vec![third, fifth];
            let mut chord = triad(&notes[..2], &intervals);
            chord.additions = Some(guess::additions(&notes, &intervals));
            chord.notes = Vec::from(notes);

            return chord;
        }
        // Additional second: an added second breaks the third interval into two seconds.
        [second, third, fifth] if second + third == 3 || second + third == 4 => {
            let intervals = vec![second + third, fifth];
            let mut chord = triad(&notes, &intervals);
            chord.additions = Some(guess::additions(&notes, &intervals));
            chord.notes = Vec::from(notes);

            return chord;
        }
        // Additional fourth: an added fourth breaks the fifth interval into two seconds.
        [third, fourth, fifth] if fifth + fourth == 3 || fifth + fourth == 4 => {
            let intervals = vec![third, fourth + fifth];
            let mut chord = triad(&notes, &intervals);
            chord.additions = Some(guess::additions(&notes, &intervals));
            chord.notes = Vec::from(notes);

            return chord;
        }
        _ => {}
    }

    // Interval between the topmost note and the first inversion of the root
    let complementary_interval = 12 - intervals[0] - intervals[1] - intervals[2];

    // In each iteration of the loop, we try to match the intervals against different inversions
    // of the intervals associated to each tetrad quality.
    let mut root_guess = 0;
    let (quality, root_position) = loop {
        // Try one inversion each time, looking for the natural intervals / root position
        let natural_interval = match root_guess {
            0 => (intervals[0], intervals[1], intervals[2]), // Root position
            1 => (intervals[1], intervals[2], complementary_interval), // 3rd inversion
            2 => (intervals[2], complementary_interval, intervals[0]), // 2nd inversion
            3 => (complementary_interval, intervals[0], intervals[1]), // 1st inversion
            _ => break (Indeterminate, None),
        };

        // Find the quality that matches our root position guess
        let quality = match natural_interval {
            (4, 3, 4) => SeventhMajor,
            (3, 4, 3) => SeventhMinor,
            (4, 3, 3) => SeventhDominant,
            (3, 3, 3) => SeventhDiminished,
            (3, 3, 4) => SeventhHalfDiminished,
            (3, 4, 4) => SeventhMinorMajor,
            (4, 4, 3) => SeventhAugmentedMajor,
            (4, 4, 2) => SeventhAugmented,
            (3, 3, 5) => SeventhDiminishedMajor,
            (4, 2, 4) => SeventhDominantFlatFive,
            (4, 2, 5) => SeventhMajorFlatFive,
            _ => Indeterminate,
        };

        if quality != Indeterminate {
            break (quality, Some(root_guess));
        } else {
            root_guess += 1;
        }
    };

    let root = root_position.map(|position| notes[position]);

    Chord {
        intervals: Vec::from(intervals),
        chord_type: ChordType::Tetrad(quality),
        notes: Vec::from(notes),
        root,
        additions: None,
    }
}
