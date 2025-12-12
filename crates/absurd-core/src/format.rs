//! Output formatting for ABSURDTTY's bureaucratic aesthetic.
//!
//! This module provides utilities for creating the distinctive
//! visual style of ABSURDTTY output: boxes, stamps, tables, and
//! other terminal decorations that make everything feel like
//! official paperwork from an institution that doesn't exist.
//!
//! # Design Principles
//!
//! - **Consistent box-drawing**: All boxes use the same Unicode characters
//! - **Stamp authenticity**: Stamps look official but mean nothing
//! - **Width-awareness**: Output respects terminal width when possible
//! - **No color by default**: Aesthetic through structure, not ANSI codes

use std::fmt::Write;

/// Box-drawing characters for consistent visual style.
pub mod chars {
    // Single-line box drawing
    pub const BOX_TL: char = '┌';
    pub const BOX_TR: char = '┐';
    pub const BOX_BL: char = '└';
    pub const BOX_BR: char = '┘';
    pub const BOX_H: char = '─';
    pub const BOX_V: char = '│';

    // Double-line box drawing
    pub const DBOX_TL: char = '╔';
    pub const DBOX_TR: char = '╗';
    pub const DBOX_BL: char = '╚';
    pub const DBOX_BR: char = '╝';
    pub const DBOX_H: char = '═';
    pub const DBOX_V: char = '║';

    // Connectors
    pub const BOX_T_DOWN: char = '┬';
    pub const BOX_T_UP: char = '┴';
    pub const BOX_T_RIGHT: char = '├';
    pub const BOX_T_LEFT: char = '┤';
    pub const BOX_CROSS: char = '┼';

    // Misc
    pub const BULLET: char = '•';
    pub const ARROW_RIGHT: char = '→';
    pub const ARROW_LEFT: char = '←';
    pub const CHECK: char = '✓';
    pub const CROSS: char = '✗';
    pub const ELLIPSIS: char = '…';
}

/// Style of box to draw.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BoxStyle {
    /// Single-line border: ┌─┐
    #[default]
    Single,
    /// Double-line border: ╔═╗
    Double,
}

impl BoxStyle {
    fn chars(&self) -> (char, char, char, char, char, char) {
        match self {
            BoxStyle::Single => (
                chars::BOX_TL,
                chars::BOX_TR,
                chars::BOX_BL,
                chars::BOX_BR,
                chars::BOX_H,
                chars::BOX_V,
            ),
            BoxStyle::Double => (
                chars::DBOX_TL,
                chars::DBOX_TR,
                chars::DBOX_BL,
                chars::DBOX_BR,
                chars::DBOX_H,
                chars::DBOX_V,
            ),
        }
    }
}

/// A builder for creating boxed text output.
///
/// # Example
///
/// ```
/// use absurd_core::format::BoxBuilder;
///
/// let output = BoxBuilder::new()
///     .title("FORM 27-B")
///     .line("Filed by: [REDACTED]")
///     .line("Status: PENDING")
///     .build();
///
/// println!("{}", output);
/// // ┌─────────────────────┐
/// // │      FORM 27-B      │
/// // ├─────────────────────┤
/// // │ Filed by: [REDACTED]│
/// // │ Status: PENDING     │
/// // └─────────────────────┘
/// ```
#[derive(Debug, Clone)]
pub struct BoxBuilder {
    title: Option<String>,
    lines: Vec<String>,
    style: BoxStyle,
    min_width: usize,
    padding: usize,
}

impl Default for BoxBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl BoxBuilder {
    /// Create a new BoxBuilder with default settings.
    pub fn new() -> Self {
        Self {
            title: None,
            lines: Vec::new(),
            style: BoxStyle::Single,
            min_width: 20,
            padding: 1,
        }
    }

    /// Set the box title (centered at top).
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Add a line of content.
    pub fn line(mut self, content: impl Into<String>) -> Self {
        self.lines.push(content.into());
        self
    }

    /// Add multiple lines of content.
    pub fn lines<I, S>(mut self, lines: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.lines.extend(lines.into_iter().map(Into::into));
        self
    }

    /// Set the box style.
    pub fn style(mut self, style: BoxStyle) -> Self {
        self.style = style;
        self
    }

    /// Set minimum width (content area, not including borders).
    pub fn min_width(mut self, width: usize) -> Self {
        self.min_width = width;
        self
    }

    /// Set horizontal padding inside the box.
    pub fn padding(mut self, padding: usize) -> Self {
        self.padding = padding;
        self
    }

    /// Build the boxed output as a String.
    pub fn build(&self) -> String {
        let (tl, tr, bl, br, h, v) = self.style.chars();

        // Calculate content width
        let title_len = self.title.as_ref().map(|t| t.chars().count()).unwrap_or(0);
        let max_line_len = self
            .lines
            .iter()
            .map(|l| l.chars().count())
            .max()
            .unwrap_or(0);
        let content_width = self
            .min_width
            .max(title_len + 4) // Title needs some breathing room
            .max(max_line_len + self.padding * 2);

        let mut output = String::new();

        // Top border
        write!(output, "{}", tl).unwrap();
        for _ in 0..content_width {
            write!(output, "{}", h).unwrap();
        }
        writeln!(output, "{}", tr).unwrap();

        // Title (if present)
        if let Some(ref title) = self.title {
            let title_chars: usize = title.chars().count();
            let left_pad = (content_width - title_chars) / 2;
            let right_pad = content_width - title_chars - left_pad;

            write!(output, "{}", v).unwrap();
            for _ in 0..left_pad {
                write!(output, " ").unwrap();
            }
            write!(output, "{}", title).unwrap();
            for _ in 0..right_pad {
                write!(output, " ").unwrap();
            }
            writeln!(output, "{}", v).unwrap();

            // Separator after title
            write!(output, "{}", chars::BOX_T_RIGHT).unwrap();
            for _ in 0..content_width {
                write!(output, "{}", h).unwrap();
            }
            writeln!(output, "{}", chars::BOX_T_LEFT).unwrap();
        }

        // Content lines
        for line in &self.lines {
            let line_chars = line.chars().count();
            let right_pad = content_width - self.padding - line_chars;

            write!(output, "{}", v).unwrap();
            for _ in 0..self.padding {
                write!(output, " ").unwrap();
            }
            write!(output, "{}", line).unwrap();
            for _ in 0..right_pad {
                write!(output, " ").unwrap();
            }
            writeln!(output, "{}", v).unwrap();
        }

        // Bottom border
        write!(output, "{}", bl).unwrap();
        for _ in 0..content_width {
            write!(output, "{}", h).unwrap();
        }
        writeln!(output, "{}", br).unwrap();

        output
    }
}

/// Pre-defined stamps for official-looking output.
///
/// Stamps are small boxed labels that appear at the bottom
/// of forms and documents to give them bureaucratic weight.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stamp {
    /// NULL BUREAU - FORM RECEIVED BUT NOT READ
    NullBureau,
    /// ABSURDTTY - CERTIFIED: INCONCLUSIVE
    Certified,
    /// FILED - NO ACTION REQUIRED
    Filed,
    /// PENDING - INDEFINITELY
    Pending,
    /// APPROVED - MEANING UNCLEAR
    Approved,
    /// DENIED - APPEAL UNAVAILABLE
    Denied,
    /// REDACTED - BY REQUEST
    Redacted,
    /// VOID - RETROACTIVELY
    Void,
}

impl Stamp {
    /// Get the stamp text content.
    pub fn text(&self) -> &'static str {
        match self {
            Stamp::NullBureau => "NULL BUREAU - FORM RECEIVED BUT NOT READ",
            Stamp::Certified => "ABSURDTTY - CERTIFIED: INCONCLUSIVE",
            Stamp::Filed => "FILED - NO ACTION REQUIRED",
            Stamp::Pending => "PENDING - INDEFINITELY",
            Stamp::Approved => "APPROVED - MEANING UNCLEAR",
            Stamp::Denied => "DENIED - APPEAL UNAVAILABLE",
            Stamp::Redacted => "REDACTED - BY REQUEST",
            Stamp::Void => "VOID - RETROACTIVELY",
        }
    }

    /// Render the stamp as a boxed string.
    pub fn render(&self) -> String {
        let text = self.text();
        let width = text.chars().count() + 2;

        let mut output = String::new();

        // Top border
        write!(output, "[").unwrap();
        for _ in 0..width {
            write!(output, "=").unwrap();
        }
        writeln!(output, "]").unwrap();

        // Content
        writeln!(output, "[ {} ]", text).unwrap();

        // Bottom border
        write!(output, "[").unwrap();
        for _ in 0..width {
            write!(output, "=").unwrap();
        }
        writeln!(output, "]").unwrap();

        output
    }

    /// Render the stamp as a single-line bracketed string.
    pub fn inline(&self) -> String {
        format!("[STAMP: {}]", self.text())
    }
}

/// A simple key-value table formatter.
///
/// # Example
///
/// ```
/// use absurd_core::format::Table;
///
/// let output = Table::new()
///     .row("CASE ID", "AB-20251212-001")
///     .row("MOOD", "exhausted")
///     .row("STATUS", "operational but barely")
///     .build();
/// ```
#[derive(Debug, Clone, Default)]
pub struct Table {
    rows: Vec<(String, String)>,
    separator: String,
    key_width: Option<usize>,
}

impl Table {
    /// Create a new empty table.
    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
            separator: ": ".to_string(),
            key_width: None,
        }
    }

    /// Add a key-value row.
    pub fn row(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.rows.push((key.into(), value.into()));
        self
    }

    /// Set the separator between key and value (default: ": ").
    pub fn separator(mut self, sep: impl Into<String>) -> Self {
        self.separator = sep.into();
        self
    }

    /// Set fixed key column width (otherwise auto-calculated).
    pub fn key_width(mut self, width: usize) -> Self {
        self.key_width = Some(width);
        self
    }

    /// Build the table as a string.
    pub fn build(&self) -> String {
        if self.rows.is_empty() {
            return String::new();
        }

        let key_width = self.key_width.unwrap_or_else(|| {
            self.rows
                .iter()
                .map(|(k, _)| k.chars().count())
                .max()
                .unwrap_or(0)
        });

        let mut output = String::new();
        for (key, value) in &self.rows {
            let key_chars = key.chars().count();
            let padding = key_width.saturating_sub(key_chars);

            write!(output, "{}", key).unwrap();
            for _ in 0..padding {
                write!(output, " ").unwrap();
            }
            writeln!(output, "{}{}", self.separator, value).unwrap();
        }

        output
    }
}

/// Helper for creating indented multi-line output.
pub struct Indenter {
    prefix: String,
    lines: Vec<String>,
}

impl Indenter {
    /// Create a new indenter with the given prefix.
    pub fn new(prefix: impl Into<String>) -> Self {
        Self {
            prefix: prefix.into(),
            lines: Vec::new(),
        }
    }

    /// Create an indenter with spaces.
    pub fn spaces(count: usize) -> Self {
        Self::new(" ".repeat(count))
    }

    /// Add a line.
    pub fn line(mut self, content: impl Into<String>) -> Self {
        self.lines.push(content.into());
        self
    }

    /// Add multiple lines.
    pub fn lines<I, S>(mut self, lines: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.lines.extend(lines.into_iter().map(Into::into));
        self
    }

    /// Build the indented output.
    pub fn build(&self) -> String {
        self.lines
            .iter()
            .map(|l| format!("{}{}\n", self.prefix, l))
            .collect()
    }
}

/// Truncate a string to a maximum width, adding ellipsis if needed.
pub fn truncate(s: &str, max_width: usize) -> String {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() <= max_width {
        s.to_string()
    } else if max_width <= 1 {
        chars::ELLIPSIS.to_string()
    } else {
        let truncated: String = chars[..max_width - 1].iter().collect();
        format!("{}{}", truncated, chars::ELLIPSIS)
    }
}

/// Center a string within a given width.
pub fn center(s: &str, width: usize) -> String {
    let char_count = s.chars().count();
    if char_count >= width {
        return s.to_string();
    }

    let left_pad = (width - char_count) / 2;
    let right_pad = width - char_count - left_pad;

    format!("{}{}{}", " ".repeat(left_pad), s, " ".repeat(right_pad))
}

/// Right-align a string within a given width.
pub fn right_align(s: &str, width: usize) -> String {
    let char_count = s.chars().count();
    if char_count >= width {
        return s.to_string();
    }

    format!("{}{}", " ".repeat(width - char_count), s)
}

/// Create a horizontal rule of the given width.
pub fn hrule(width: usize) -> String {
    chars::BOX_H.to_string().repeat(width)
}

/// Create a double horizontal rule of the given width.
pub fn hrule_double(width: usize) -> String {
    chars::DBOX_H.to_string().repeat(width)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn box_builder_basic() {
        let output = BoxBuilder::new()
            .title("TEST")
            .line("Hello")
            .line("World")
            .build();

        assert!(output.contains("TEST"));
        assert!(output.contains("Hello"));
        assert!(output.contains("World"));
        assert!(output.contains("┌"));
        assert!(output.contains("└"));
    }

    #[test]
    fn box_builder_double_style() {
        let output = BoxBuilder::new()
            .style(BoxStyle::Double)
            .title("DOUBLE")
            .build();

        assert!(output.contains("╔"));
        assert!(output.contains("╝"));
    }

    #[test]
    fn stamp_render() {
        let stamp = Stamp::NullBureau.render();
        assert!(stamp.contains("NULL BUREAU"));
        assert!(stamp.contains("[="));
    }

    #[test]
    fn stamp_inline() {
        let stamp = Stamp::Filed.inline();
        assert_eq!(stamp, "[STAMP: FILED - NO ACTION REQUIRED]");
    }

    #[test]
    fn table_basic() {
        let table = Table::new()
            .row("KEY1", "value1")
            .row("LONGER_KEY", "value2")
            .build();

        assert!(table.contains("KEY1"));
        assert!(table.contains("LONGER_KEY"));
        assert!(table.contains(": "));
    }

    #[test]
    fn truncate_short_string() {
        assert_eq!(truncate("hello", 10), "hello");
    }

    #[test]
    fn truncate_long_string() {
        let result = truncate("hello world", 8);
        assert_eq!(result.chars().count(), 8);
        assert!(result.ends_with('…'));
    }

    #[test]
    fn center_string() {
        let centered = center("hi", 6);
        assert_eq!(centered, "  hi  ");
    }

    #[test]
    fn right_align_string() {
        let aligned = right_align("hi", 6);
        assert_eq!(aligned, "    hi");
    }
}

