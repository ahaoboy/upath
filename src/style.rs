//! Numbering "styles" — the pair of delimiter characters used to bracket the
//! numeric suffix of a duplicate, e.g. `a(1).txt`, `a[1].txt`, `a（1）.txt`.

/// A bracketing style for numeric duplicate suffixes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NumberStyle {
    /// Round parentheses: `a(1).txt` — the default.
    Round,
    /// Square brackets: `a[1].txt`.
    Square,
    /// Curly braces: `a{1}.txt`.
    Curly,
    /// Full-width (ideographic) parentheses: `a（1）.txt`.
    FullWidthRound,
    /// Full-width square brackets: `a［1］.txt`.
    FullWidthSquare,
    /// Lenticular (CJK book) brackets: `a【1】.txt`.
    ChineseBracket,
    /// Corner brackets: `a「1」.txt`.
    CornerBracket,
}

impl NumberStyle {
    /// The opening delimiter.
    pub(crate) const fn open(self) -> &'static str {
        match self {
            NumberStyle::Round => "(",
            NumberStyle::Square => "[",
            NumberStyle::Curly => "{",
            NumberStyle::FullWidthRound => "（",
            NumberStyle::FullWidthSquare => "［",
            NumberStyle::ChineseBracket => "【",
            NumberStyle::CornerBracket => "「",
        }
    }

    /// The closing delimiter.
    pub(crate) const fn close(self) -> &'static str {
        match self {
            NumberStyle::Round => ")",
            NumberStyle::Square => "]",
            NumberStyle::Curly => "}",
            NumberStyle::FullWidthRound => "）",
            NumberStyle::FullWidthSquare => "］",
            NumberStyle::ChineseBracket => "】",
            NumberStyle::CornerBracket => "」",
        }
    }

    /// All styles, in the order they are probed when parsing an existing name.
    pub(crate) const ALL: [NumberStyle; 7] = [
        NumberStyle::Round,
        NumberStyle::Square,
        NumberStyle::Curly,
        NumberStyle::FullWidthRound,
        NumberStyle::FullWidthSquare,
        NumberStyle::ChineseBracket,
        NumberStyle::CornerBracket,
    ];

    /// The default style used when a duplicate has no existing number: `a(1).txt`.
    pub(crate) const DEFAULT: NumberStyle = NumberStyle::Round;
}

/// A version suffix that was detected on an existing name.
///
/// For example in `report[042].txt` the detected value is `style = Square`,
/// `number = 42` and `pad = Some(3)` (so the leading zeros are preserved as
/// `report[043].txt` rather than collapsing to `report[43].txt`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Version {
    /// The bracket style the version was found in.
    pub style: NumberStyle,
    /// The parsed number.
    pub number: u64,
    /// Minimum digit width to preserve, set only when the original number was
    /// zero-padded (e.g. `007`).
    pub pad: Option<usize>,
}

/// If `base` ends in `<open><digits><close>` for any known style, return the
/// name *without* that suffix together with the parsed [`Version`].
///
/// Only a suffix at the very end is treated as a version number, so a name
/// such as `photo(2)` is interpreted as a duplicate of `photo` (and will be
/// continued as `photo(3)`), while `notes_2024` is left untouched.
pub(crate) fn detect_version(base: &str) -> Option<(String, Version)> {
    for style in NumberStyle::ALL {
        let (open, close) = (style.open(), style.close());
        let Some(prefix) = base.strip_suffix(close) else {
            continue;
        };
        let Some(idx) = prefix.rfind(open) else {
            continue;
        };
        let digits = &prefix[idx + open.len()..];
        if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
            continue;
        }
        let number = digits.parse::<u64>().ok()?;
        let core = prefix[..idx].to_string();
        let pad = if digits.len() > 1 && digits.starts_with('0') {
            Some(digits.len())
        } else {
            None
        };
        return Some((core, Version { style, number, pad }));
    }
    None
}

/// Format a version into the text `open + digits + close`, applying
/// zero-padding only when `pad` requests it.
pub(crate) fn render(style: NumberStyle, number: u64, pad: Option<usize>) -> String {
    let digits = match pad {
        Some(width) if width > 1 => {
            let mut s = number.to_string();
            while s.len() < width {
                s.insert(0, '0');
            }
            s
        }
        _ => number.to_string(),
    };
    format!("{}{}{}", style.open(), digits, style.close())
}
