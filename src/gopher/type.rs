use std::fmt;

/// Gopher types are defined according to RFC 1436.
#[allow(missing_docs)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Type {
    Text,       // 0 | cyan
    Menu,       // 1 | blue
    CSOEntity,  // 2 | unsupported
    Error,      // 3 | red
    Binhex,     // 4 | download
    DOSFile,    // 5 | download
    UUEncoded,  // 6 | download
    Search,     // 7 | white
    Telnet,     // 8 | gray underline
    Binary,     // 9 | download
    Mirror,     // + | unsupported
    GIF,        // g | download
    Telnet3270, // T | unsupported
    HTML,       // h | green
    Image,      // I | download
    PNG,        // p | download
    Info,       // i | yellow
    Sound,      // s | green underline
    Document,   // d | download
    Video,      // ; | green underline
    Xml,        // X | cyan
    Executable, // x | red, nonstandard in any way (I made it up)
    Command,   // c | download
    Mailbox,    // M | unsupported
}

impl Type {
    /// Is this an info line?
    pub fn is_info(self) -> bool {
        self == Type::Info
    }

    /// Text document?
    pub fn is_text(self) -> bool {
        matches!(self, Type::Text | Type::Xml)
    }

    /// HTML link?
    pub fn is_html(self) -> bool {
        self == Type::HTML
    }

    /// Telnet link?
    pub fn is_telnet(self) -> bool {
        self == Type::Telnet
    }

    /// Is this a link, ie something we can navigate to or open?
    pub fn is_link(self) -> bool {
        !self.is_info()
    }

    /// Is this something we can download?
    pub fn is_download(self) -> bool {
        matches!(
            self,
            Type::Binhex
                | Type::DOSFile
                | Type::UUEncoded
                | Type::Binary
                | Type::GIF
                | Type::Image
                | Type::PNG
                | Type::Sound
                | Type::Video
                | Type::Command
                | Type::Document
                | Type::Executable
        )
    }

    /// Check if media to open in player
    pub fn is_media(self) -> bool {
        matches!(self, Type::Sound | Type::Video)
    }

    /// Is this a type phetch supports?
    pub fn is_supported(self) -> bool {
        !matches!(
            self,
            Type::CSOEntity | Type::Mirror | Type::Telnet3270 | Type::Mailbox
        )
    }

    /// Gopher Item Type to RFC char.
    pub fn to_char(self) -> char {
        match self {
            Type::Text => '0',
            Type::Menu => '1',
            Type::CSOEntity => '2',
            Type::Error => '3',
            Type::Binhex => '4',
            Type::DOSFile => '5',
            Type::UUEncoded => '6',
            Type::Search => '7',
            Type::Telnet => '8',
            Type::Binary => '9',
            Type::Mirror => '+',
            Type::GIF => 'g',
            Type::Telnet3270 => 'T',
            Type::HTML => 'h',
            Type::Image => 'I',
            Type::PNG => 'p',
            Type::Info => 'i',
            Type::Sound => 's',
            Type::Document => 'd',
            Type::Video => ';',
            Type::Command => 'c',
            Type::Xml => 'X',
            Type::Executable => 'x',
            Type::Mailbox => 'M',
        }
    }

    /// Create a Gopher Item Type from its RFC char code.
    pub fn from(c: char) -> Option<Type> {
        Some(match c {
            '0' => Type::Text,
            '1' => Type::Menu,
            '2' => Type::CSOEntity,
            '3' => Type::Error,
            '4' => Type::Binhex,
            '5' => Type::DOSFile,
            '6' => Type::UUEncoded,
            '7' => Type::Search,
            '8' => Type::Telnet,
            '9' => Type::Binary,
            '+' => Type::Mirror,
            'g' => Type::GIF,
            'T' => Type::Telnet3270,
            'h' => Type::HTML,
            'I' => Type::Image,
            'p' => Type::PNG,
            'i' => Type::Info,
            's' => Type::Sound,
            'd' => Type::Document,
            ';' => Type::Video,
            'c' => Type::Command,
            'x' => Type::Executable,
            'X' => Type::Xml,
            'M' => Type::Mailbox,
            _ => return None,
        })
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}
