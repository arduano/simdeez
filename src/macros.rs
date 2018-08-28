//! Utility macros.
#![allow(unused_macros)]
macro_rules! mm_shuffle {
    ($z:expr, $y:expr, $x:expr, $w:expr) => {
        ($z << 6) | ($y << 4) | ($x << 2) | $w
    };
}

macro_rules! constify_imm8 {
    ($imm8:expr, $expand:ident) => {
        #[allow(overflowing_literals)]
        match ($imm8) & 0b1111_1111 {
            x @ 0...254 => $expand!(x),
            _ => $expand!(255),
        }
    };
}

macro_rules! constify_imm6 {
    ($imm8:expr, $expand:ident) => {
        #[allow(overflowing_literals)]
        match ($imm8) & 0b1_1111 {
            x @ 0...30 => $expand!(x),
            _ => $expand!(31),
        }
    };
}

macro_rules! constify_imm4 {
    ($imm8:expr, $expand:ident) => {
        #[allow(overflowing_literals)]
        match ($imm8) & 0b1111 {
            x @ 0...14 => $expand!(x),
            _ => $expand!(15),
        }
    };
}

macro_rules! constify_imm3 {
    ($imm8:expr, $expand:ident) => {
        #[allow(overflowing_literals)]
        match ($imm8) & 0b111 {
            x @ 0...6 => $expand!(x),
            _ => $expand!(7),
        }
    };
}

macro_rules! constify_imm2 {
    ($imm8:expr, $expand:ident) => {
        #[allow(overflowing_literals)]
        match ($imm8) & 0b11 {
            x @ 0...2 => $expand!(x),
            _ => $expand!(3),
        }
    };
}

#[cfg(test)]
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr, $eps:expr) => {{
        let (a, b) = (&$a, &$b);
        assert!(
            (*a - *b).abs() < $eps,
            "assertion failed: `(left !== right)` \
             (left: `{:?}`, right: `{:?}`, expect diff: `{:?}`, real diff: `{:?}`)",
            *a,
            *b,
            $eps,
            (*a - *b).abs()
        );
    }};
}
