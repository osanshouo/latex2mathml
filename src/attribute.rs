use std::fmt;

/// mi mathvariant attribute
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Variant {
    Normal,
    Italic,
    Bold,
    BoldItalic,
    DoubleStruck,
    BoldFraktur,
    Script,
    BoldScript,
    Fraktur,
    SansSerif,
    BoldSansSerif,
    SansSerifItalic,
    SansSerifBoldItalic,
    Monospace,
}

impl fmt::Display for Variant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Variant::Normal              => write!(f, "normal"),
            Variant::Italic              => write!(f, "italic"),
            Variant::Bold                => write!(f, "bold"),
            Variant::BoldItalic          => write!(f, "bold-italic"),
            Variant::DoubleStruck        => write!(f, "double-struck"),
            Variant::BoldFraktur         => write!(f, "bold-fraktur"),
            Variant::Script              => write!(f, "script"),
            Variant::BoldScript          => write!(f, "bold-script"),
            Variant::Fraktur             => write!(f, "fraktur"),
            Variant::SansSerif           => write!(f, "sans-serif"),
            Variant::BoldSansSerif       => write!(f, "bold-sans-serif"),
            Variant::SansSerifItalic     => write!(f, "sans-serif-italic"),
            Variant::SansSerifBoldItalic => write!(f, "sans-serif-bold-italic"),
            Variant::Monospace           => write!(f, "monospace"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Accent {
    True,
    False,
}

impl fmt::Display for Accent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Accent::True => write!(f, "true"),
            Accent::False => write!(f, "false"),
        }
    }
}
