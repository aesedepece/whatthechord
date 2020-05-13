use alloc::string::String;
use core::fmt::{Display, Formatter};

/// Different qualities of dyads.
#[derive(Debug, Eq, PartialEq)]
pub enum DyadQuality {
    Augmented(u8),
    Diminished(u8),
    Indeterminate,
    Major(u8),
    Minor(u8),
    Perfect(u8),
}

/// Text representations of dyad qualities.
impl Display for DyadQuality {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        use DyadQuality::*;

        let name = match self {
            Augmented(x) => format!("aug{}", x),
            Diminished(x) => format!("sus{}", x),
            Indeterminate => String::from("(ind)"),
            Major(x) => format!("{}", x),
            Minor(x) => format!("m{}", x),
            Perfect(x) => format!("P{}", x),
        };

        write!(f, "{}", name)
    }
}

/// Different qualities of triads.
#[derive(Debug, Eq, PartialEq)]
pub enum TriadQuality {
    Augmented,
    Diminished,
    Indeterminate,
    Major,
    Minor,
    Suspended(u8),
}

/// Text representations of triad qualities.
impl Display for TriadQuality {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        use TriadQuality::*;

        let name = match self {
            Augmented => String::from("aug"),
            Diminished => String::from("dim"),
            Indeterminate => String::from("ind"),
            Major => String::new(),
            Minor => String::from("m"),
            Suspended(x) => format!("sus{}", x),
        };

        write!(f, "{}", name)
    }
}

/// Different types of tetrads.
#[derive(Debug, Eq, PartialEq)]
pub enum TetradQuality {
    Indeterminate,
    SeventhDiminished,       // Tertian
    SeventhDominant,         // Tertian
    SeventhDominantFlatFive, // Non-tertian
    SeventhMajor,            // Tertian
    SeventhMajorFlatFive,    // Non-tertian
    SeventhMinor,            // Tertian
    SeventhMinorMajor,       // Tertian
    SeventhAugmented,        // Non-tertian | Also: SeventhAugmentedFifth, SeventhSharpFive
    SeventhDiminishedMajor,  // Non-Tertian
    SeventhHalfDiminished,   // Tertian | Also: SeventhMinorFlatFive
    SeventhAugmentedMajor,   // Tertian | Also: SeventhMajorSharpFive
}

/// Text representations of tetrad qualities.
impl Display for TetradQuality {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        use TetradQuality::*;

        let name = match self {
            Indeterminate => String::from("ind"),
            SeventhDiminished => String::from("dim7"),
            SeventhDominant => String::from("7"),
            SeventhDominantFlatFive => String::from("7b5"),
            SeventhMajor => String::from("M7"),
            SeventhMajorFlatFive => String::from("M7b5"),
            SeventhMinor => String::from("m7"),
            SeventhMinorMajor => String::from("mM7"),
            SeventhAugmented => String::from("aug7"),
            SeventhDiminishedMajor => String::from("mM7b5"),
            SeventhHalfDiminished => String::from("m7b5"),
            SeventhAugmentedMajor => String::from("M7#5"),
        };

        write!(f, "{}", name)
    }
}
