#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]
#![allow(dead_code)]

#[doc = r" Error types."]
pub mod error {
    #[doc = r" Error from a `TryFrom` or `FromStr` implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
#[doc = "`AccentContent`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": ["]
#[doc = "    \"string\","]
#[doc = "    \"null\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct AccentContent(pub ::std::option::Option<::std::string::String>);
impl ::std::ops::Deref for AccentContent {
    type Target = ::std::option::Option<::std::string::String>;
    fn deref(&self) -> &::std::option::Option<::std::string::String> {
        &self.0
    }
}
impl ::std::convert::From<AccentContent> for ::std::option::Option<::std::string::String> {
    fn from(value: AccentContent) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::option::Option<::std::string::String>> for AccentContent {
    fn from(value: ::std::option::Option<::std::string::String>) -> Self {
        Self(value)
    }
}
#[doc = "`AppearanceContent`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"light\","]
#[doc = "    \"dark\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AppearanceContent {
    #[serde(rename = "light")]
    Light,
    #[serde(rename = "dark")]
    Dark,
}
impl ::std::fmt::Display for AppearanceContent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Light => f.write_str("light"),
            Self::Dark => f.write_str("dark"),
        }
    }
}
impl ::std::str::FromStr for AppearanceContent {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "light" => Ok(Self::Light),
            "dark" => Ok(Self::Dark),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AppearanceContent {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for AppearanceContent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AppearanceContent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`FontStyleContent`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"normal\","]
#[doc = "    \"italic\","]
#[doc = "    \"oblique\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum FontStyleContent {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "italic")]
    Italic,
    #[serde(rename = "oblique")]
    Oblique,
}
impl ::std::fmt::Display for FontStyleContent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Normal => f.write_str("normal"),
            Self::Italic => f.write_str("italic"),
            Self::Oblique => f.write_str("oblique"),
        }
    }
}
impl ::std::str::FromStr for FontStyleContent {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "normal" => Ok(Self::Normal),
            "italic" => Ok(Self::Italic),
            "oblique" => Ok(Self::Oblique),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for FontStyleContent {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for FontStyleContent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for FontStyleContent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`HighlightStyleContent`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"background_color\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"color\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"font_style\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/FontStyleContent\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"font_weight\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"enum\": ["]
#[doc = "            100,"]
#[doc = "            200,"]
#[doc = "            300,"]
#[doc = "            400,"]
#[doc = "            500,"]
#[doc = "            600,"]
#[doc = "            700,"]
#[doc = "            800,"]
#[doc = "            900"]
#[doc = "          ]"]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct HighlightStyleContent {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub background_color: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub color: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub font_style: ::std::option::Option<FontStyleContent>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub font_weight: ::std::option::Option<HighlightStyleContentFontWeight>,
}
impl ::std::default::Default for HighlightStyleContent {
    fn default() -> Self {
        Self {
            background_color: Default::default(),
            color: Default::default(),
            font_style: Default::default(),
            font_weight: Default::default(),
        }
    }
}
impl HighlightStyleContent {
    pub fn builder() -> builder::HighlightStyleContent {
        Default::default()
    }
}
#[doc = "`HighlightStyleContentFontWeight`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"enum\": ["]
#[doc = "    100,"]
#[doc = "    200,"]
#[doc = "    300,"]
#[doc = "    400,"]
#[doc = "    500,"]
#[doc = "    600,"]
#[doc = "    700,"]
#[doc = "    800,"]
#[doc = "    900"]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Serialize, Clone, Debug)]
#[serde(transparent)]
pub struct HighlightStyleContentFontWeight(f64);
impl ::std::ops::Deref for HighlightStyleContentFontWeight {
    type Target = f64;
    fn deref(&self) -> &f64 {
        &self.0
    }
}
impl ::std::convert::From<HighlightStyleContentFontWeight> for f64 {
    fn from(value: HighlightStyleContentFontWeight) -> Self {
        value.0
    }
}
impl ::std::convert::TryFrom<f64> for HighlightStyleContentFontWeight {
    type Error = self::error::ConversionError;
    fn try_from(value: f64) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![
            100_f64, 200_f64, 300_f64, 400_f64, 500_f64, 600_f64, 700_f64, 800_f64, 900_f64,
        ]
        .contains(&value)
        {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for HighlightStyleContentFontWeight {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<f64>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "`PlayerColorContent`"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"background\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"cursor\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"selection\": {"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct PlayerColorContent {
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub background: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub cursor: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub selection: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for PlayerColorContent {
    fn default() -> Self {
        Self {
            background: Default::default(),
            cursor: Default::default(),
            selection: Default::default(),
        }
    }
}
impl PlayerColorContent {
    pub fn builder() -> builder::PlayerColorContent {
        Default::default()
    }
}
#[doc = "The content of a serialized theme."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The content of a serialized theme.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"appearance\","]
#[doc = "    \"name\","]
#[doc = "    \"style\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"appearance\": {"]
#[doc = "      \"$ref\": \"#/definitions/AppearanceContent\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"style\": {"]
#[doc = "      \"$ref\": \"#/definitions/ThemeStyleContent\""]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThemeContent {
    pub appearance: AppearanceContent,
    pub name: ::std::string::String,
    pub style: ThemeStyleContent,
}
impl ThemeContent {
    pub fn builder() -> builder::ThemeContent {
        Default::default()
    }
}
#[doc = "The content of a serialized theme family."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"title\": \"ThemeFamilyContent\","]
#[doc = "  \"description\": \"The content of a serialized theme family.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"required\": ["]
#[doc = "    \"author\","]
#[doc = "    \"name\","]
#[doc = "    \"themes\""]
#[doc = "  ],"]
#[doc = "  \"properties\": {"]
#[doc = "    \"author\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"name\": {"]
#[doc = "      \"type\": \"string\""]
#[doc = "    },"]
#[doc = "    \"themes\": {"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/ThemeContent\""]
#[doc = "      }"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThemeFamilyContent {
    pub author: ::std::string::String,
    pub name: ::std::string::String,
    pub themes: ::std::vec::Vec<ThemeContent>,
}
impl ThemeFamilyContent {
    pub fn builder() -> builder::ThemeFamilyContent {
        Default::default()
    }
}
#[doc = "The content of a serialized theme."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The content of a serialized theme.\","]
#[doc = "  \"type\": \"object\","]
#[doc = "  \"properties\": {"]
#[doc = "    \"accents\": {"]
#[doc = "      \"default\": [],"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/AccentContent\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"background\": {"]
#[doc = "      \"description\": \"Background Color. Used for the app background and blank panels or windows.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"background.appearance\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"anyOf\": ["]
#[doc = "        {"]
#[doc = "          \"$ref\": \"#/definitions/WindowBackgroundContent\""]
#[doc = "        },"]
#[doc = "        {"]
#[doc = "          \"type\": \"null\""]
#[doc = "        }"]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"border\": {"]
#[doc = "      \"description\": \"Border color. Used for most borders, is usually a high contrast color.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"border.disabled\": {"]
#[doc = "      \"description\": \"Border color. Used for disabled elements, like a disabled input or button.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"border.focused\": {"]
#[doc = "      \"description\": \"Border color. Used for focused elements, like keyboard focused list item.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"border.selected\": {"]
#[doc = "      \"description\": \"Border color. Used for selected elements, like an active search filter or selected checkbox.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"border.transparent\": {"]
#[doc = "      \"description\": \"Border color. Used for transparent borders. Used for placeholder borders when an element gains a border on state change.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"border.variant\": {"]
#[doc = "      \"description\": \"Border color. Used for deemphasized borders, like a visual divider between two sections\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"conflict\": {"]
#[doc = "      \"description\": \"Indicates some kind of conflict, like a file changed on disk while it was open, or merge conflicts in a Git repository.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"conflict.background\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"conflict.border\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"created\": {"]
#[doc = "      \"description\": \"Indicates something new, like a new file added to a Git repository.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"created.background\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"created.border\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"deleted\": {"]
#[doc = "      \"description\": \"Indicates that something no longer exists, like a deleted file.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"deleted.background\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"deleted.border\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"drop_target.background\": {"]
#[doc = "      \"description\": \"Background Color. Used for the area that shows where a dragged element will be dropped.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"editor.active_line.background\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"editor.active_line_number\": {"]
#[doc = "      \"description\": \"Text Color. Used for the text of the line number in the editor gutter when the line is highlighted.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"editor.active_wrap_guide\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"editor.background\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"editor.document_highlight.bracket_background\": {"]
#[doc = "      \"description\": \"Highlighted brackets background color.\\n\\nMatching brackets in the cursor scope are highlighted with this background color.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"editor.document_highlight.read_background\": {"]
#[doc = "      \"description\": \"Read-access of a symbol, like reading a variable.\\n\\nA document highlight is a range inside a text document which deserves special attention. Usually a document highlight is visualized by changing the background color of its range.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"editor.document_highlight.write_background\": {"]
#[doc = "      \"description\": \"Read-access of a symbol, like reading a variable.\\n\\nA document highlight is a range inside a text document which deserves special attention. Usually a document highlight is visualized by changing the background color of its range.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"editor.foreground\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"editor.gutter.background\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"editor.highlighted_line.background\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"editor.indent_guide\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"editor.indent_guide_active\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"editor.invisible\": {"]
#[doc = "      \"description\": \"Text Color. Used to mark invisible characters in the editor.\\n\\nExample: spaces, tabs, carriage returns, etc.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"editor.line_number\": {"]
#[doc = "      \"description\": \"Text Color. Used for the text of the line number in the editor gutter.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"editor.subheader.background\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"editor.wrap_guide\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"element.active\": {"]
#[doc = "      \"description\": \"Background Color. Used for the active state of an element that should have a different background than the surface it's on.\\n\\nActive states are triggered by the mouse button being pressed down on an element, or the Return button or other activator being pressd.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"element.background\": {"]
#[doc = "      \"description\": \"Background Color. Used for the background of an element that should have a different background than the surface it's on.\\n\\nElements might include: Buttons, Inputs, Checkboxes, Radio Buttons...\\n\\nFor an element that should have the same background as the surface it's on, use `ghost_element_background`.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"element.disabled\": {"]
#[doc = "      \"description\": \"Background Color. Used for the disabled state of an element that should have a different background than the surface it's on.\\n\\nDisabled states are shown when a user cannot interact with an element, like a disabled button or input.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"element.hover\": {"]
#[doc = "      \"description\": \"Background Color. Used for the hover state of an element that should have a different background than the surface it's on.\\n\\nHover states are triggered by the mouse entering an element, or a finger touching an element on a touch screen.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"element.selected\": {"]
#[doc = "      \"description\": \"Background Color. Used for the selected state of an element that should have a different background than the surface it's on.\\n\\nSelected states are triggered by the element being selected (or \\\"activated\\\") by the user.\\n\\nThis could include a selected checkbox, a toggleable button that is toggled on, etc.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"elevated_surface.background\": {"]
#[doc = "      \"description\": \"Background color. Used for elevated surfaces, like a context menu, popup, or dialog.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"error\": {"]
#[doc = "      \"description\": \"Indicates a system error, a failed operation or a diagnostic error.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"error.background\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"error.border\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"ghost_element.active\": {"]
#[doc = "      \"description\": \"Background Color. Used for the active state of a ghost element that should have the same background as the surface it's on.\\n\\nActive states are triggered by the mouse button being pressed down on an element, or the Return button or other activator being pressd.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"ghost_element.background\": {"]
#[doc = "      \"description\": \"Used for the background of a ghost element that should have the same background as the surface it's on.\\n\\nElements might include: Buttons, Inputs, Checkboxes, Radio Buttons...\\n\\nFor an element that should have a different background than the surface it's on, use `element_background`.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"ghost_element.disabled\": {"]
#[doc = "      \"description\": \"Background Color. Used for the disabled state of a ghost element that should have the same background as the surface it's on.\\n\\nDisabled states are shown when a user cannot interact with an element, like a disabled button or input.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"ghost_element.hover\": {"]
#[doc = "      \"description\": \"Background Color. Used for the hover state of a ghost element that should have the same background as the surface it's on.\\n\\nHover states are triggered by the mouse entering an element, or a finger touching an element on a touch screen.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"ghost_element.selected\": {"]
#[doc = "      \"description\": \"Background Color. Used for the selected state of a ghost element that should have the same background as the surface it's on.\\n\\nSelected states are triggered by the element being selected (or \\\"activated\\\") by the user.\\n\\nThis could include a selected checkbox, a toggleable button that is toggled on, etc.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"hidden\": {"]
#[doc = "      \"description\": \"Represents a hidden status, such as a file being hidden in a file tree.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"hidden.background\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"hidden.border\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"hint\": {"]
#[doc = "      \"description\": \"Indicates a hint or some kind of additional information.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"hint.background\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"hint.border\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"icon\": {"]
#[doc = "      \"description\": \"Fill Color. Used for the default fill color of an icon.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"icon.accent\": {"]
#[doc = "      \"description\": \"Fill Color. Used for the accent fill color of an icon.\\n\\nThis might be used to show when a toggleable icon button is selected.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"icon.disabled\": {"]
#[doc = "      \"description\": \"Fill Color. Used for the disabled fill color of an icon.\\n\\nDisabled states are shown when a user cannot interact with an element, like a icon button.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"icon.muted\": {"]
#[doc = "      \"description\": \"Fill Color. Used for the muted or deemphasized fill color of an icon.\\n\\nThis might be used to show an icon in an inactive pane, or to demphasize a series of icons to give them less visual weight.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"icon.placeholder\": {"]
#[doc = "      \"description\": \"Fill Color. Used for the placeholder fill color of an icon.\\n\\nThis might be used to show an icon in an input that disappears when the user enters text.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"ignored\": {"]
#[doc = "      \"description\": \"Indicates that something is deliberately ignored, such as a file or operation ignored by Git.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"ignored.background\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"ignored.border\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"info\": {"]
#[doc = "      \"description\": \"Represents informational status updates or messages.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"info.background\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"info.border\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"link_text.hover\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"modified\": {"]
#[doc = "      \"description\": \"Indicates a changed or altered status, like a file that has been edited.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"modified.background\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"modified.border\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"pane.focused_border\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"pane_group.border\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"panel.background\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"panel.focused_border\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"panel.indent_guide\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"panel.indent_guide_active\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"panel.indent_guide_hover\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"players\": {"]
#[doc = "      \"default\": [],"]
#[doc = "      \"type\": \"array\","]
#[doc = "      \"items\": {"]
#[doc = "        \"$ref\": \"#/definitions/PlayerColorContent\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"predictive\": {"]
#[doc = "      \"description\": \"Indicates something that is predicted, like automatic code completion, or generated code.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"predictive.background\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"predictive.border\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"renamed\": {"]
#[doc = "      \"description\": \"Represents a renamed status, such as a file that has been renamed.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"renamed.background\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"renamed.border\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"scrollbar.thumb.background\": {"]
#[doc = "      \"description\": \"The color of the scrollbar thumb.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"scrollbar.thumb.border\": {"]
#[doc = "      \"description\": \"The border color of the scrollbar thumb.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"scrollbar.thumb.hover_background\": {"]
#[doc = "      \"description\": \"The color of the scrollbar thumb when hovered over.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"scrollbar.track.background\": {"]
#[doc = "      \"description\": \"The background color of the scrollbar track.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"scrollbar.track.border\": {"]
#[doc = "      \"description\": \"The border color of the scrollbar track.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"search.match_background\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"status_bar.background\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"success\": {"]
#[doc = "      \"description\": \"Indicates a successful operation or task completion.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"success.background\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"success.border\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"surface.background\": {"]
#[doc = "      \"description\": \"Background Color. Used for grounded surfaces like a panel or tab.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"syntax\": {"]
#[doc = "      \"description\": \"The styles for syntax nodes.\","]
#[doc = "      \"default\": {},"]
#[doc = "      \"type\": \"object\","]
#[doc = "      \"additionalProperties\": {"]
#[doc = "        \"$ref\": \"#/definitions/HighlightStyleContent\""]
#[doc = "      }"]
#[doc = "    },"]
#[doc = "    \"tab.active_background\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"tab.inactive_background\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"tab_bar.background\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"terminal.ansi.background\": {"]
#[doc = "      \"description\": \"Terminal ANSI background color.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"terminal.ansi.black\": {"]
#[doc = "      \"description\": \"Black ANSI terminal color.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"terminal.ansi.blue\": {"]
#[doc = "      \"description\": \"Blue ANSI terminal color.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"terminal.ansi.bright_black\": {"]
#[doc = "      \"description\": \"Bright black ANSI terminal color.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"terminal.ansi.bright_blue\": {"]
#[doc = "      \"description\": \"Bright blue ANSI terminal color.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"terminal.ansi.bright_cyan\": {"]
#[doc = "      \"description\": \"Bright cyan ANSI terminal color.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"terminal.ansi.bright_green\": {"]
#[doc = "      \"description\": \"Bright green ANSI terminal color.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"terminal.ansi.bright_magenta\": {"]
#[doc = "      \"description\": \"Bright magenta ANSI terminal color.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"terminal.ansi.bright_red\": {"]
#[doc = "      \"description\": \"Bright red ANSI terminal color.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"terminal.ansi.bright_white\": {"]
#[doc = "      \"description\": \"Bright white ANSI terminal color.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"terminal.ansi.bright_yellow\": {"]
#[doc = "      \"description\": \"Bright yellow ANSI terminal color.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"terminal.ansi.cyan\": {"]
#[doc = "      \"description\": \"Cyan ANSI terminal color.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"terminal.ansi.dim_black\": {"]
#[doc = "      \"description\": \"Dim black ANSI terminal color.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"terminal.ansi.dim_blue\": {"]
#[doc = "      \"description\": \"Dim blue ANSI terminal color.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"terminal.ansi.dim_cyan\": {"]
#[doc = "      \"description\": \"Dim cyan ANSI terminal color.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"terminal.ansi.dim_green\": {"]
#[doc = "      \"description\": \"Dim green ANSI terminal color.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"terminal.ansi.dim_magenta\": {"]
#[doc = "      \"description\": \"Dim magenta ANSI terminal color.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"terminal.ansi.dim_red\": {"]
#[doc = "      \"description\": \"Dim red ANSI terminal color.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"terminal.ansi.dim_white\": {"]
#[doc = "      \"description\": \"Dim white ANSI terminal color.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"terminal.ansi.dim_yellow\": {"]
#[doc = "      \"description\": \"Dim yellow ANSI terminal color.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"terminal.ansi.green\": {"]
#[doc = "      \"description\": \"Green ANSI terminal color.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"terminal.ansi.magenta\": {"]
#[doc = "      \"description\": \"Magenta ANSI terminal color.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"terminal.ansi.red\": {"]
#[doc = "      \"description\": \"Red ANSI terminal color.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"terminal.ansi.white\": {"]
#[doc = "      \"description\": \"White ANSI terminal color.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"terminal.ansi.yellow\": {"]
#[doc = "      \"description\": \"Yellow ANSI terminal color.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"terminal.background\": {"]
#[doc = "      \"description\": \"Terminal background color.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"terminal.bright_foreground\": {"]
#[doc = "      \"description\": \"Bright terminal foreground color.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"terminal.dim_foreground\": {"]
#[doc = "      \"description\": \"Dim terminal foreground color.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"terminal.foreground\": {"]
#[doc = "      \"description\": \"Terminal foreground color.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"text\": {"]
#[doc = "      \"description\": \"Text Color. Default text color used for most text.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"text.accent\": {"]
#[doc = "      \"description\": \"Text Color. Color used for emphasis or highlighting certain text, like an active filter or a matched character in a search.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"text.disabled\": {"]
#[doc = "      \"description\": \"Text Color. Color used for text denoting disabled elements. Typically, the color is faded or grayed out to emphasize the disabled state.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"text.muted\": {"]
#[doc = "      \"description\": \"Text Color. Color of muted or deemphasized text. It is a subdued version of the standard text color.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"text.placeholder\": {"]
#[doc = "      \"description\": \"Text Color. Color of the placeholder text typically shown in input fields to guide the user to enter valid data.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"title_bar.background\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"title_bar.inactive_background\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"toolbar.background\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"unreachable\": {"]
#[doc = "      \"description\": \"Indicates some kind of unreachable status, like a block of code that can never be reached.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"unreachable.background\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"unreachable.border\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"warning\": {"]
#[doc = "      \"description\": \"Represents a warning status, like an operation that is about to fail.\","]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"warning.background\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    },"]
#[doc = "    \"warning.border\": {"]
#[doc = "      \"default\": null,"]
#[doc = "      \"type\": ["]
#[doc = "        \"string\","]
#[doc = "        \"null\""]
#[doc = "      ]"]
#[doc = "    }"]
#[doc = "  }"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug)]
pub struct ThemeStyleContent {
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub accents: ::std::vec::Vec<AccentContent>,
    #[doc = "Background Color. Used for the app background and blank panels or windows."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub background: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "background.appearance",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub background_appearance: ::std::option::Option<WindowBackgroundContent>,
    #[doc = "Border color. Used for most borders, is usually a high contrast color."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub border: ::std::option::Option<::std::string::String>,
    #[doc = "Border color. Used for disabled elements, like a disabled input or button."]
    #[serde(
        rename = "border.disabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub border_disabled: ::std::option::Option<::std::string::String>,
    #[doc = "Border color. Used for focused elements, like keyboard focused list item."]
    #[serde(
        rename = "border.focused",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub border_focused: ::std::option::Option<::std::string::String>,
    #[doc = "Border color. Used for selected elements, like an active search filter or selected checkbox."]
    #[serde(
        rename = "border.selected",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub border_selected: ::std::option::Option<::std::string::String>,
    #[doc = "Border color. Used for transparent borders. Used for placeholder borders when an element gains a border on state change."]
    #[serde(
        rename = "border.transparent",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub border_transparent: ::std::option::Option<::std::string::String>,
    #[doc = "Border color. Used for deemphasized borders, like a visual divider between two sections"]
    #[serde(
        rename = "border.variant",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub border_variant: ::std::option::Option<::std::string::String>,
    #[doc = "Indicates some kind of conflict, like a file changed on disk while it was open, or merge conflicts in a Git repository."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub conflict: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "conflict.background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub conflict_background: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "conflict.border",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub conflict_border: ::std::option::Option<::std::string::String>,
    #[doc = "Indicates something new, like a new file added to a Git repository."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub created: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "created.background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub created_background: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "created.border",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub created_border: ::std::option::Option<::std::string::String>,
    #[doc = "Indicates that something no longer exists, like a deleted file."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub deleted: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "deleted.background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub deleted_background: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "deleted.border",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub deleted_border: ::std::option::Option<::std::string::String>,
    #[doc = "Background Color. Used for the area that shows where a dragged element will be dropped."]
    #[serde(
        rename = "drop_target.background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub drop_target_background: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "editor.active_line.background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub editor_active_line_background: ::std::option::Option<::std::string::String>,
    #[doc = "Text Color. Used for the text of the line number in the editor gutter when the line is highlighted."]
    #[serde(
        rename = "editor.active_line_number",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub editor_active_line_number: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "editor.active_wrap_guide",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub editor_active_wrap_guide: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "editor.background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub editor_background: ::std::option::Option<::std::string::String>,
    #[doc = "Highlighted brackets background color.\n\nMatching brackets in the cursor scope are highlighted with this background color."]
    #[serde(
        rename = "editor.document_highlight.bracket_background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub editor_document_highlight_bracket_background: ::std::option::Option<::std::string::String>,
    #[doc = "Read-access of a symbol, like reading a variable.\n\nA document highlight is a range inside a text document which deserves special attention. Usually a document highlight is visualized by changing the background color of its range."]
    #[serde(
        rename = "editor.document_highlight.read_background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub editor_document_highlight_read_background: ::std::option::Option<::std::string::String>,
    #[doc = "Read-access of a symbol, like reading a variable.\n\nA document highlight is a range inside a text document which deserves special attention. Usually a document highlight is visualized by changing the background color of its range."]
    #[serde(
        rename = "editor.document_highlight.write_background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub editor_document_highlight_write_background: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "editor.foreground",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub editor_foreground: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "editor.gutter.background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub editor_gutter_background: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "editor.highlighted_line.background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub editor_highlighted_line_background: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "editor.indent_guide",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub editor_indent_guide: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "editor.indent_guide_active",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub editor_indent_guide_active: ::std::option::Option<::std::string::String>,
    #[doc = "Text Color. Used to mark invisible characters in the editor.\n\nExample: spaces, tabs, carriage returns, etc."]
    #[serde(
        rename = "editor.invisible",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub editor_invisible: ::std::option::Option<::std::string::String>,
    #[doc = "Text Color. Used for the text of the line number in the editor gutter."]
    #[serde(
        rename = "editor.line_number",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub editor_line_number: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "editor.subheader.background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub editor_subheader_background: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "editor.wrap_guide",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub editor_wrap_guide: ::std::option::Option<::std::string::String>,
    #[doc = "Background Color. Used for the active state of an element that should have a different background than the surface it's on.\n\nActive states are triggered by the mouse button being pressed down on an element, or the Return button or other activator being pressd."]
    #[serde(
        rename = "element.active",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub element_active: ::std::option::Option<::std::string::String>,
    #[doc = "Background Color. Used for the background of an element that should have a different background than the surface it's on.\n\nElements might include: Buttons, Inputs, Checkboxes, Radio Buttons...\n\nFor an element that should have the same background as the surface it's on, use `ghost_element_background`."]
    #[serde(
        rename = "element.background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub element_background: ::std::option::Option<::std::string::String>,
    #[doc = "Background Color. Used for the disabled state of an element that should have a different background than the surface it's on.\n\nDisabled states are shown when a user cannot interact with an element, like a disabled button or input."]
    #[serde(
        rename = "element.disabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub element_disabled: ::std::option::Option<::std::string::String>,
    #[doc = "Background Color. Used for the hover state of an element that should have a different background than the surface it's on.\n\nHover states are triggered by the mouse entering an element, or a finger touching an element on a touch screen."]
    #[serde(
        rename = "element.hover",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub element_hover: ::std::option::Option<::std::string::String>,
    #[doc = "Background Color. Used for the selected state of an element that should have a different background than the surface it's on.\n\nSelected states are triggered by the element being selected (or \"activated\") by the user.\n\nThis could include a selected checkbox, a toggleable button that is toggled on, etc."]
    #[serde(
        rename = "element.selected",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub element_selected: ::std::option::Option<::std::string::String>,
    #[doc = "Background color. Used for elevated surfaces, like a context menu, popup, or dialog."]
    #[serde(
        rename = "elevated_surface.background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub elevated_surface_background: ::std::option::Option<::std::string::String>,
    #[doc = "Indicates a system error, a failed operation or a diagnostic error."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub error: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "error.background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub error_background: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "error.border",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub error_border: ::std::option::Option<::std::string::String>,
    #[doc = "Background Color. Used for the active state of a ghost element that should have the same background as the surface it's on.\n\nActive states are triggered by the mouse button being pressed down on an element, or the Return button or other activator being pressd."]
    #[serde(
        rename = "ghost_element.active",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ghost_element_active: ::std::option::Option<::std::string::String>,
    #[doc = "Used for the background of a ghost element that should have the same background as the surface it's on.\n\nElements might include: Buttons, Inputs, Checkboxes, Radio Buttons...\n\nFor an element that should have a different background than the surface it's on, use `element_background`."]
    #[serde(
        rename = "ghost_element.background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ghost_element_background: ::std::option::Option<::std::string::String>,
    #[doc = "Background Color. Used for the disabled state of a ghost element that should have the same background as the surface it's on.\n\nDisabled states are shown when a user cannot interact with an element, like a disabled button or input."]
    #[serde(
        rename = "ghost_element.disabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ghost_element_disabled: ::std::option::Option<::std::string::String>,
    #[doc = "Background Color. Used for the hover state of a ghost element that should have the same background as the surface it's on.\n\nHover states are triggered by the mouse entering an element, or a finger touching an element on a touch screen."]
    #[serde(
        rename = "ghost_element.hover",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ghost_element_hover: ::std::option::Option<::std::string::String>,
    #[doc = "Background Color. Used for the selected state of a ghost element that should have the same background as the surface it's on.\n\nSelected states are triggered by the element being selected (or \"activated\") by the user.\n\nThis could include a selected checkbox, a toggleable button that is toggled on, etc."]
    #[serde(
        rename = "ghost_element.selected",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ghost_element_selected: ::std::option::Option<::std::string::String>,
    #[doc = "Represents a hidden status, such as a file being hidden in a file tree."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub hidden: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "hidden.background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub hidden_background: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "hidden.border",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub hidden_border: ::std::option::Option<::std::string::String>,
    #[doc = "Indicates a hint or some kind of additional information."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub hint: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "hint.background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub hint_background: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "hint.border",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub hint_border: ::std::option::Option<::std::string::String>,
    #[doc = "Fill Color. Used for the default fill color of an icon."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub icon: ::std::option::Option<::std::string::String>,
    #[doc = "Fill Color. Used for the accent fill color of an icon.\n\nThis might be used to show when a toggleable icon button is selected."]
    #[serde(
        rename = "icon.accent",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub icon_accent: ::std::option::Option<::std::string::String>,
    #[doc = "Fill Color. Used for the disabled fill color of an icon.\n\nDisabled states are shown when a user cannot interact with an element, like a icon button."]
    #[serde(
        rename = "icon.disabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub icon_disabled: ::std::option::Option<::std::string::String>,
    #[doc = "Fill Color. Used for the muted or deemphasized fill color of an icon.\n\nThis might be used to show an icon in an inactive pane, or to demphasize a series of icons to give them less visual weight."]
    #[serde(
        rename = "icon.muted",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub icon_muted: ::std::option::Option<::std::string::String>,
    #[doc = "Fill Color. Used for the placeholder fill color of an icon.\n\nThis might be used to show an icon in an input that disappears when the user enters text."]
    #[serde(
        rename = "icon.placeholder",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub icon_placeholder: ::std::option::Option<::std::string::String>,
    #[doc = "Indicates that something is deliberately ignored, such as a file or operation ignored by Git."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub ignored: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "ignored.background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ignored_background: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "ignored.border",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub ignored_border: ::std::option::Option<::std::string::String>,
    #[doc = "Represents informational status updates or messages."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub info: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "info.background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub info_background: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "info.border",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub info_border: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "link_text.hover",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub link_text_hover: ::std::option::Option<::std::string::String>,
    #[doc = "Indicates a changed or altered status, like a file that has been edited."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub modified: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "modified.background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub modified_background: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "modified.border",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub modified_border: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "pane.focused_border",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub pane_focused_border: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "pane_group.border",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub pane_group_border: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "panel.background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub panel_background: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "panel.focused_border",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub panel_focused_border: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "panel.indent_guide",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub panel_indent_guide: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "panel.indent_guide_active",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub panel_indent_guide_active: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "panel.indent_guide_hover",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub panel_indent_guide_hover: ::std::option::Option<::std::string::String>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub players: ::std::vec::Vec<PlayerColorContent>,
    #[doc = "Indicates something that is predicted, like automatic code completion, or generated code."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub predictive: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "predictive.background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub predictive_background: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "predictive.border",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub predictive_border: ::std::option::Option<::std::string::String>,
    #[doc = "Represents a renamed status, such as a file that has been renamed."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub renamed: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "renamed.background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub renamed_background: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "renamed.border",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub renamed_border: ::std::option::Option<::std::string::String>,
    #[doc = "The color of the scrollbar thumb."]
    #[serde(
        rename = "scrollbar.thumb.background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub scrollbar_thumb_background: ::std::option::Option<::std::string::String>,
    #[doc = "The border color of the scrollbar thumb."]
    #[serde(
        rename = "scrollbar.thumb.border",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub scrollbar_thumb_border: ::std::option::Option<::std::string::String>,
    #[doc = "The color of the scrollbar thumb when hovered over."]
    #[serde(
        rename = "scrollbar.thumb.hover_background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub scrollbar_thumb_hover_background: ::std::option::Option<::std::string::String>,
    #[doc = "The background color of the scrollbar track."]
    #[serde(
        rename = "scrollbar.track.background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub scrollbar_track_background: ::std::option::Option<::std::string::String>,
    #[doc = "The border color of the scrollbar track."]
    #[serde(
        rename = "scrollbar.track.border",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub scrollbar_track_border: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "search.match_background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub search_match_background: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "status_bar.background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub status_bar_background: ::std::option::Option<::std::string::String>,
    #[doc = "Indicates a successful operation or task completion."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub success: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "success.background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub success_background: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "success.border",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub success_border: ::std::option::Option<::std::string::String>,
    #[doc = "Background Color. Used for grounded surfaces like a panel or tab."]
    #[serde(
        rename = "surface.background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub surface_background: ::std::option::Option<::std::string::String>,
    #[doc = "The styles for syntax nodes."]
    #[serde(
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub syntax: ::std::collections::HashMap<::std::string::String, HighlightStyleContent>,
    #[serde(
        rename = "tab.active_background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub tab_active_background: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "tab_bar.background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub tab_bar_background: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "tab.inactive_background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub tab_inactive_background: ::std::option::Option<::std::string::String>,
    #[doc = "Terminal ANSI background color."]
    #[serde(
        rename = "terminal.ansi.background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub terminal_ansi_background: ::std::option::Option<::std::string::String>,
    #[doc = "Black ANSI terminal color."]
    #[serde(
        rename = "terminal.ansi.black",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub terminal_ansi_black: ::std::option::Option<::std::string::String>,
    #[doc = "Blue ANSI terminal color."]
    #[serde(
        rename = "terminal.ansi.blue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub terminal_ansi_blue: ::std::option::Option<::std::string::String>,
    #[doc = "Bright black ANSI terminal color."]
    #[serde(
        rename = "terminal.ansi.bright_black",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub terminal_ansi_bright_black: ::std::option::Option<::std::string::String>,
    #[doc = "Bright blue ANSI terminal color."]
    #[serde(
        rename = "terminal.ansi.bright_blue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub terminal_ansi_bright_blue: ::std::option::Option<::std::string::String>,
    #[doc = "Bright cyan ANSI terminal color."]
    #[serde(
        rename = "terminal.ansi.bright_cyan",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub terminal_ansi_bright_cyan: ::std::option::Option<::std::string::String>,
    #[doc = "Bright green ANSI terminal color."]
    #[serde(
        rename = "terminal.ansi.bright_green",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub terminal_ansi_bright_green: ::std::option::Option<::std::string::String>,
    #[doc = "Bright magenta ANSI terminal color."]
    #[serde(
        rename = "terminal.ansi.bright_magenta",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub terminal_ansi_bright_magenta: ::std::option::Option<::std::string::String>,
    #[doc = "Bright red ANSI terminal color."]
    #[serde(
        rename = "terminal.ansi.bright_red",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub terminal_ansi_bright_red: ::std::option::Option<::std::string::String>,
    #[doc = "Bright white ANSI terminal color."]
    #[serde(
        rename = "terminal.ansi.bright_white",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub terminal_ansi_bright_white: ::std::option::Option<::std::string::String>,
    #[doc = "Bright yellow ANSI terminal color."]
    #[serde(
        rename = "terminal.ansi.bright_yellow",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub terminal_ansi_bright_yellow: ::std::option::Option<::std::string::String>,
    #[doc = "Cyan ANSI terminal color."]
    #[serde(
        rename = "terminal.ansi.cyan",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub terminal_ansi_cyan: ::std::option::Option<::std::string::String>,
    #[doc = "Dim black ANSI terminal color."]
    #[serde(
        rename = "terminal.ansi.dim_black",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub terminal_ansi_dim_black: ::std::option::Option<::std::string::String>,
    #[doc = "Dim blue ANSI terminal color."]
    #[serde(
        rename = "terminal.ansi.dim_blue",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub terminal_ansi_dim_blue: ::std::option::Option<::std::string::String>,
    #[doc = "Dim cyan ANSI terminal color."]
    #[serde(
        rename = "terminal.ansi.dim_cyan",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub terminal_ansi_dim_cyan: ::std::option::Option<::std::string::String>,
    #[doc = "Dim green ANSI terminal color."]
    #[serde(
        rename = "terminal.ansi.dim_green",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub terminal_ansi_dim_green: ::std::option::Option<::std::string::String>,
    #[doc = "Dim magenta ANSI terminal color."]
    #[serde(
        rename = "terminal.ansi.dim_magenta",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub terminal_ansi_dim_magenta: ::std::option::Option<::std::string::String>,
    #[doc = "Dim red ANSI terminal color."]
    #[serde(
        rename = "terminal.ansi.dim_red",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub terminal_ansi_dim_red: ::std::option::Option<::std::string::String>,
    #[doc = "Dim white ANSI terminal color."]
    #[serde(
        rename = "terminal.ansi.dim_white",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub terminal_ansi_dim_white: ::std::option::Option<::std::string::String>,
    #[doc = "Dim yellow ANSI terminal color."]
    #[serde(
        rename = "terminal.ansi.dim_yellow",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub terminal_ansi_dim_yellow: ::std::option::Option<::std::string::String>,
    #[doc = "Green ANSI terminal color."]
    #[serde(
        rename = "terminal.ansi.green",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub terminal_ansi_green: ::std::option::Option<::std::string::String>,
    #[doc = "Magenta ANSI terminal color."]
    #[serde(
        rename = "terminal.ansi.magenta",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub terminal_ansi_magenta: ::std::option::Option<::std::string::String>,
    #[doc = "Red ANSI terminal color."]
    #[serde(
        rename = "terminal.ansi.red",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub terminal_ansi_red: ::std::option::Option<::std::string::String>,
    #[doc = "White ANSI terminal color."]
    #[serde(
        rename = "terminal.ansi.white",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub terminal_ansi_white: ::std::option::Option<::std::string::String>,
    #[doc = "Yellow ANSI terminal color."]
    #[serde(
        rename = "terminal.ansi.yellow",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub terminal_ansi_yellow: ::std::option::Option<::std::string::String>,
    #[doc = "Terminal background color."]
    #[serde(
        rename = "terminal.background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub terminal_background: ::std::option::Option<::std::string::String>,
    #[doc = "Bright terminal foreground color."]
    #[serde(
        rename = "terminal.bright_foreground",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub terminal_bright_foreground: ::std::option::Option<::std::string::String>,
    #[doc = "Dim terminal foreground color."]
    #[serde(
        rename = "terminal.dim_foreground",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub terminal_dim_foreground: ::std::option::Option<::std::string::String>,
    #[doc = "Terminal foreground color."]
    #[serde(
        rename = "terminal.foreground",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub terminal_foreground: ::std::option::Option<::std::string::String>,
    #[doc = "Text Color. Default text color used for most text."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub text: ::std::option::Option<::std::string::String>,
    #[doc = "Text Color. Color used for emphasis or highlighting certain text, like an active filter or a matched character in a search."]
    #[serde(
        rename = "text.accent",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub text_accent: ::std::option::Option<::std::string::String>,
    #[doc = "Text Color. Color used for text denoting disabled elements. Typically, the color is faded or grayed out to emphasize the disabled state."]
    #[serde(
        rename = "text.disabled",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub text_disabled: ::std::option::Option<::std::string::String>,
    #[doc = "Text Color. Color of muted or deemphasized text. It is a subdued version of the standard text color."]
    #[serde(
        rename = "text.muted",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub text_muted: ::std::option::Option<::std::string::String>,
    #[doc = "Text Color. Color of the placeholder text typically shown in input fields to guide the user to enter valid data."]
    #[serde(
        rename = "text.placeholder",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub text_placeholder: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "title_bar.background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub title_bar_background: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "title_bar.inactive_background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub title_bar_inactive_background: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "toolbar.background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub toolbar_background: ::std::option::Option<::std::string::String>,
    #[doc = "Indicates some kind of unreachable status, like a block of code that can never be reached."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub unreachable: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "unreachable.background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub unreachable_background: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "unreachable.border",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub unreachable_border: ::std::option::Option<::std::string::String>,
    #[doc = "Represents a warning status, like an operation that is about to fail."]
    #[serde(default, skip_serializing_if = "::std::option::Option::is_none")]
    pub warning: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "warning.background",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub warning_background: ::std::option::Option<::std::string::String>,
    #[serde(
        rename = "warning.border",
        default,
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub warning_border: ::std::option::Option<::std::string::String>,
}
impl ::std::default::Default for ThemeStyleContent {
    fn default() -> Self {
        Self {
            accents: Default::default(),
            background: Default::default(),
            background_appearance: Default::default(),
            border: Default::default(),
            border_disabled: Default::default(),
            border_focused: Default::default(),
            border_selected: Default::default(),
            border_transparent: Default::default(),
            border_variant: Default::default(),
            conflict: Default::default(),
            conflict_background: Default::default(),
            conflict_border: Default::default(),
            created: Default::default(),
            created_background: Default::default(),
            created_border: Default::default(),
            deleted: Default::default(),
            deleted_background: Default::default(),
            deleted_border: Default::default(),
            drop_target_background: Default::default(),
            editor_active_line_background: Default::default(),
            editor_active_line_number: Default::default(),
            editor_active_wrap_guide: Default::default(),
            editor_background: Default::default(),
            editor_document_highlight_bracket_background: Default::default(),
            editor_document_highlight_read_background: Default::default(),
            editor_document_highlight_write_background: Default::default(),
            editor_foreground: Default::default(),
            editor_gutter_background: Default::default(),
            editor_highlighted_line_background: Default::default(),
            editor_indent_guide: Default::default(),
            editor_indent_guide_active: Default::default(),
            editor_invisible: Default::default(),
            editor_line_number: Default::default(),
            editor_subheader_background: Default::default(),
            editor_wrap_guide: Default::default(),
            element_active: Default::default(),
            element_background: Default::default(),
            element_disabled: Default::default(),
            element_hover: Default::default(),
            element_selected: Default::default(),
            elevated_surface_background: Default::default(),
            error: Default::default(),
            error_background: Default::default(),
            error_border: Default::default(),
            ghost_element_active: Default::default(),
            ghost_element_background: Default::default(),
            ghost_element_disabled: Default::default(),
            ghost_element_hover: Default::default(),
            ghost_element_selected: Default::default(),
            hidden: Default::default(),
            hidden_background: Default::default(),
            hidden_border: Default::default(),
            hint: Default::default(),
            hint_background: Default::default(),
            hint_border: Default::default(),
            icon: Default::default(),
            icon_accent: Default::default(),
            icon_disabled: Default::default(),
            icon_muted: Default::default(),
            icon_placeholder: Default::default(),
            ignored: Default::default(),
            ignored_background: Default::default(),
            ignored_border: Default::default(),
            info: Default::default(),
            info_background: Default::default(),
            info_border: Default::default(),
            link_text_hover: Default::default(),
            modified: Default::default(),
            modified_background: Default::default(),
            modified_border: Default::default(),
            pane_focused_border: Default::default(),
            pane_group_border: Default::default(),
            panel_background: Default::default(),
            panel_focused_border: Default::default(),
            panel_indent_guide: Default::default(),
            panel_indent_guide_active: Default::default(),
            panel_indent_guide_hover: Default::default(),
            players: Default::default(),
            predictive: Default::default(),
            predictive_background: Default::default(),
            predictive_border: Default::default(),
            renamed: Default::default(),
            renamed_background: Default::default(),
            renamed_border: Default::default(),
            scrollbar_thumb_background: Default::default(),
            scrollbar_thumb_border: Default::default(),
            scrollbar_thumb_hover_background: Default::default(),
            scrollbar_track_background: Default::default(),
            scrollbar_track_border: Default::default(),
            search_match_background: Default::default(),
            status_bar_background: Default::default(),
            success: Default::default(),
            success_background: Default::default(),
            success_border: Default::default(),
            surface_background: Default::default(),
            syntax: Default::default(),
            tab_active_background: Default::default(),
            tab_bar_background: Default::default(),
            tab_inactive_background: Default::default(),
            terminal_ansi_background: Default::default(),
            terminal_ansi_black: Default::default(),
            terminal_ansi_blue: Default::default(),
            terminal_ansi_bright_black: Default::default(),
            terminal_ansi_bright_blue: Default::default(),
            terminal_ansi_bright_cyan: Default::default(),
            terminal_ansi_bright_green: Default::default(),
            terminal_ansi_bright_magenta: Default::default(),
            terminal_ansi_bright_red: Default::default(),
            terminal_ansi_bright_white: Default::default(),
            terminal_ansi_bright_yellow: Default::default(),
            terminal_ansi_cyan: Default::default(),
            terminal_ansi_dim_black: Default::default(),
            terminal_ansi_dim_blue: Default::default(),
            terminal_ansi_dim_cyan: Default::default(),
            terminal_ansi_dim_green: Default::default(),
            terminal_ansi_dim_magenta: Default::default(),
            terminal_ansi_dim_red: Default::default(),
            terminal_ansi_dim_white: Default::default(),
            terminal_ansi_dim_yellow: Default::default(),
            terminal_ansi_green: Default::default(),
            terminal_ansi_magenta: Default::default(),
            terminal_ansi_red: Default::default(),
            terminal_ansi_white: Default::default(),
            terminal_ansi_yellow: Default::default(),
            terminal_background: Default::default(),
            terminal_bright_foreground: Default::default(),
            terminal_dim_foreground: Default::default(),
            terminal_foreground: Default::default(),
            text: Default::default(),
            text_accent: Default::default(),
            text_disabled: Default::default(),
            text_muted: Default::default(),
            text_placeholder: Default::default(),
            title_bar_background: Default::default(),
            title_bar_inactive_background: Default::default(),
            toolbar_background: Default::default(),
            unreachable: Default::default(),
            unreachable_background: Default::default(),
            unreachable_border: Default::default(),
            warning: Default::default(),
            warning_background: Default::default(),
            warning_border: Default::default(),
        }
    }
}
impl ThemeStyleContent {
    pub fn builder() -> builder::ThemeStyleContent {
        Default::default()
    }
}
#[doc = "The background appearance of the window."]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{"]
#[doc = "  \"description\": \"The background appearance of the window.\","]
#[doc = "  \"type\": \"string\","]
#[doc = "  \"enum\": ["]
#[doc = "    \"opaque\","]
#[doc = "    \"transparent\","]
#[doc = "    \"blurred\""]
#[doc = "  ]"]
#[doc = "}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum WindowBackgroundContent {
    #[serde(rename = "opaque")]
    Opaque,
    #[serde(rename = "transparent")]
    Transparent,
    #[serde(rename = "blurred")]
    Blurred,
}
impl ::std::fmt::Display for WindowBackgroundContent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Opaque => f.write_str("opaque"),
            Self::Transparent => f.write_str("transparent"),
            Self::Blurred => f.write_str("blurred"),
        }
    }
}
impl ::std::str::FromStr for WindowBackgroundContent {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "opaque" => Ok(Self::Opaque),
            "transparent" => Ok(Self::Transparent),
            "blurred" => Ok(Self::Blurred),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WindowBackgroundContent {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<&::std::string::String> for WindowBackgroundContent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: &::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WindowBackgroundContent {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = r" Types for composing complex structures."]
pub mod builder {
    #[derive(Clone, Debug)]
    pub struct HighlightStyleContent {
        background_color: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        color: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        font_style: ::std::result::Result<
            ::std::option::Option<super::FontStyleContent>,
            ::std::string::String,
        >,
        font_weight: ::std::result::Result<
            ::std::option::Option<super::HighlightStyleContentFontWeight>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for HighlightStyleContent {
        fn default() -> Self {
            Self {
                background_color: Ok(Default::default()),
                color: Ok(Default::default()),
                font_style: Ok(Default::default()),
                font_weight: Ok(Default::default()),
            }
        }
    }
    impl HighlightStyleContent {
        pub fn background_color<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.background_color = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for background_color: {e}"));
            self
        }
        pub fn color<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.color = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for color: {e}"));
            self
        }
        pub fn font_style<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::FontStyleContent>>,
            T::Error: ::std::fmt::Display,
        {
            self.font_style = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for font_style: {e}"));
            self
        }
        pub fn font_weight<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::option::Option<super::HighlightStyleContentFontWeight>,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.font_weight = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for font_weight: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<HighlightStyleContent> for super::HighlightStyleContent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: HighlightStyleContent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                background_color: value.background_color?,
                color: value.color?,
                font_style: value.font_style?,
                font_weight: value.font_weight?,
            })
        }
    }
    impl ::std::convert::From<super::HighlightStyleContent> for HighlightStyleContent {
        fn from(value: super::HighlightStyleContent) -> Self {
            Self {
                background_color: Ok(value.background_color),
                color: Ok(value.color),
                font_style: Ok(value.font_style),
                font_weight: Ok(value.font_weight),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct PlayerColorContent {
        background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        cursor: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        selection: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for PlayerColorContent {
        fn default() -> Self {
            Self {
                background: Ok(Default::default()),
                cursor: Ok(Default::default()),
                selection: Ok(Default::default()),
            }
        }
    }
    impl PlayerColorContent {
        pub fn background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.background = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for background: {e}"));
            self
        }
        pub fn cursor<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.cursor = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for cursor: {e}"));
            self
        }
        pub fn selection<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.selection = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for selection: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<PlayerColorContent> for super::PlayerColorContent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: PlayerColorContent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                background: value.background?,
                cursor: value.cursor?,
                selection: value.selection?,
            })
        }
    }
    impl ::std::convert::From<super::PlayerColorContent> for PlayerColorContent {
        fn from(value: super::PlayerColorContent) -> Self {
            Self {
                background: Ok(value.background),
                cursor: Ok(value.cursor),
                selection: Ok(value.selection),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ThemeContent {
        appearance: ::std::result::Result<super::AppearanceContent, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        style: ::std::result::Result<super::ThemeStyleContent, ::std::string::String>,
    }
    impl ::std::default::Default for ThemeContent {
        fn default() -> Self {
            Self {
                appearance: Err("no value supplied for appearance".to_string()),
                name: Err("no value supplied for name".to_string()),
                style: Err("no value supplied for style".to_string()),
            }
        }
    }
    impl ThemeContent {
        pub fn appearance<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::AppearanceContent>,
            T::Error: ::std::fmt::Display,
        {
            self.appearance = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for appearance: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn style<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<super::ThemeStyleContent>,
            T::Error: ::std::fmt::Display,
        {
            self.style = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for style: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ThemeContent> for super::ThemeContent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ThemeContent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                appearance: value.appearance?,
                name: value.name?,
                style: value.style?,
            })
        }
    }
    impl ::std::convert::From<super::ThemeContent> for ThemeContent {
        fn from(value: super::ThemeContent) -> Self {
            Self {
                appearance: Ok(value.appearance),
                name: Ok(value.name),
                style: Ok(value.style),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ThemeFamilyContent {
        author: ::std::result::Result<::std::string::String, ::std::string::String>,
        name: ::std::result::Result<::std::string::String, ::std::string::String>,
        themes: ::std::result::Result<::std::vec::Vec<super::ThemeContent>, ::std::string::String>,
    }
    impl ::std::default::Default for ThemeFamilyContent {
        fn default() -> Self {
            Self {
                author: Err("no value supplied for author".to_string()),
                name: Err("no value supplied for name".to_string()),
                themes: Err("no value supplied for themes".to_string()),
            }
        }
    }
    impl ThemeFamilyContent {
        pub fn author<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.author = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for author: {e}"));
            self
        }
        pub fn name<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::string::String>,
            T::Error: ::std::fmt::Display,
        {
            self.name = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for name: {e}"));
            self
        }
        pub fn themes<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::ThemeContent>>,
            T::Error: ::std::fmt::Display,
        {
            self.themes = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for themes: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ThemeFamilyContent> for super::ThemeFamilyContent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ThemeFamilyContent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                author: value.author?,
                name: value.name?,
                themes: value.themes?,
            })
        }
    }
    impl ::std::convert::From<super::ThemeFamilyContent> for ThemeFamilyContent {
        fn from(value: super::ThemeFamilyContent) -> Self {
            Self {
                author: Ok(value.author),
                name: Ok(value.name),
                themes: Ok(value.themes),
            }
        }
    }
    #[derive(Clone, Debug)]
    pub struct ThemeStyleContent {
        accents:
            ::std::result::Result<::std::vec::Vec<super::AccentContent>, ::std::string::String>,
        background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        background_appearance: ::std::result::Result<
            ::std::option::Option<super::WindowBackgroundContent>,
            ::std::string::String,
        >,
        border: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        border_disabled: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        border_focused: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        border_selected: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        border_transparent: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        border_variant: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        conflict: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        conflict_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        conflict_border: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        created: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        created_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        created_border: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        deleted: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        deleted_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        deleted_border: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        drop_target_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        editor_active_line_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        editor_active_line_number: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        editor_active_wrap_guide: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        editor_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        editor_document_highlight_bracket_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        editor_document_highlight_read_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        editor_document_highlight_write_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        editor_foreground: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        editor_gutter_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        editor_highlighted_line_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        editor_indent_guide: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        editor_indent_guide_active: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        editor_invisible: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        editor_line_number: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        editor_subheader_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        editor_wrap_guide: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        element_active: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        element_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        element_disabled: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        element_hover: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        element_selected: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        elevated_surface_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        error: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        error_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        error_border: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        ghost_element_active: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        ghost_element_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        ghost_element_disabled: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        ghost_element_hover: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        ghost_element_selected: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        hidden: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        hidden_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        hidden_border: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        hint: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        hint_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        hint_border: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        icon: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        icon_accent: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        icon_disabled: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        icon_muted: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        icon_placeholder: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        ignored: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        ignored_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        ignored_border: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        info: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        info_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        info_border: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        link_text_hover: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        modified: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        modified_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        modified_border: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        pane_focused_border: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        pane_group_border: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        panel_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        panel_focused_border: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        panel_indent_guide: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        panel_indent_guide_active: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        panel_indent_guide_hover: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        players: ::std::result::Result<
            ::std::vec::Vec<super::PlayerColorContent>,
            ::std::string::String,
        >,
        predictive: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        predictive_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        predictive_border: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        renamed: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        renamed_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        renamed_border: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        scrollbar_thumb_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        scrollbar_thumb_border: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        scrollbar_thumb_hover_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        scrollbar_track_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        scrollbar_track_border: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        search_match_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        status_bar_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        success: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        success_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        success_border: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        surface_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        syntax: ::std::result::Result<
            ::std::collections::HashMap<::std::string::String, super::HighlightStyleContent>,
            ::std::string::String,
        >,
        tab_active_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        tab_bar_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        tab_inactive_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        terminal_ansi_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        terminal_ansi_black: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        terminal_ansi_blue: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        terminal_ansi_bright_black: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        terminal_ansi_bright_blue: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        terminal_ansi_bright_cyan: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        terminal_ansi_bright_green: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        terminal_ansi_bright_magenta: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        terminal_ansi_bright_red: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        terminal_ansi_bright_white: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        terminal_ansi_bright_yellow: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        terminal_ansi_cyan: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        terminal_ansi_dim_black: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        terminal_ansi_dim_blue: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        terminal_ansi_dim_cyan: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        terminal_ansi_dim_green: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        terminal_ansi_dim_magenta: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        terminal_ansi_dim_red: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        terminal_ansi_dim_white: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        terminal_ansi_dim_yellow: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        terminal_ansi_green: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        terminal_ansi_magenta: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        terminal_ansi_red: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        terminal_ansi_white: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        terminal_ansi_yellow: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        terminal_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        terminal_bright_foreground: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        terminal_dim_foreground: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        terminal_foreground: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        text: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        text_accent: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        text_disabled: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        text_muted: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        text_placeholder: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        title_bar_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        title_bar_inactive_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        toolbar_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        unreachable: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        unreachable_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        unreachable_border: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        warning: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        warning_background: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
        warning_border: ::std::result::Result<
            ::std::option::Option<::std::string::String>,
            ::std::string::String,
        >,
    }
    impl ::std::default::Default for ThemeStyleContent {
        fn default() -> Self {
            Self {
                accents: Ok(Default::default()),
                background: Ok(Default::default()),
                background_appearance: Ok(Default::default()),
                border: Ok(Default::default()),
                border_disabled: Ok(Default::default()),
                border_focused: Ok(Default::default()),
                border_selected: Ok(Default::default()),
                border_transparent: Ok(Default::default()),
                border_variant: Ok(Default::default()),
                conflict: Ok(Default::default()),
                conflict_background: Ok(Default::default()),
                conflict_border: Ok(Default::default()),
                created: Ok(Default::default()),
                created_background: Ok(Default::default()),
                created_border: Ok(Default::default()),
                deleted: Ok(Default::default()),
                deleted_background: Ok(Default::default()),
                deleted_border: Ok(Default::default()),
                drop_target_background: Ok(Default::default()),
                editor_active_line_background: Ok(Default::default()),
                editor_active_line_number: Ok(Default::default()),
                editor_active_wrap_guide: Ok(Default::default()),
                editor_background: Ok(Default::default()),
                editor_document_highlight_bracket_background: Ok(Default::default()),
                editor_document_highlight_read_background: Ok(Default::default()),
                editor_document_highlight_write_background: Ok(Default::default()),
                editor_foreground: Ok(Default::default()),
                editor_gutter_background: Ok(Default::default()),
                editor_highlighted_line_background: Ok(Default::default()),
                editor_indent_guide: Ok(Default::default()),
                editor_indent_guide_active: Ok(Default::default()),
                editor_invisible: Ok(Default::default()),
                editor_line_number: Ok(Default::default()),
                editor_subheader_background: Ok(Default::default()),
                editor_wrap_guide: Ok(Default::default()),
                element_active: Ok(Default::default()),
                element_background: Ok(Default::default()),
                element_disabled: Ok(Default::default()),
                element_hover: Ok(Default::default()),
                element_selected: Ok(Default::default()),
                elevated_surface_background: Ok(Default::default()),
                error: Ok(Default::default()),
                error_background: Ok(Default::default()),
                error_border: Ok(Default::default()),
                ghost_element_active: Ok(Default::default()),
                ghost_element_background: Ok(Default::default()),
                ghost_element_disabled: Ok(Default::default()),
                ghost_element_hover: Ok(Default::default()),
                ghost_element_selected: Ok(Default::default()),
                hidden: Ok(Default::default()),
                hidden_background: Ok(Default::default()),
                hidden_border: Ok(Default::default()),
                hint: Ok(Default::default()),
                hint_background: Ok(Default::default()),
                hint_border: Ok(Default::default()),
                icon: Ok(Default::default()),
                icon_accent: Ok(Default::default()),
                icon_disabled: Ok(Default::default()),
                icon_muted: Ok(Default::default()),
                icon_placeholder: Ok(Default::default()),
                ignored: Ok(Default::default()),
                ignored_background: Ok(Default::default()),
                ignored_border: Ok(Default::default()),
                info: Ok(Default::default()),
                info_background: Ok(Default::default()),
                info_border: Ok(Default::default()),
                link_text_hover: Ok(Default::default()),
                modified: Ok(Default::default()),
                modified_background: Ok(Default::default()),
                modified_border: Ok(Default::default()),
                pane_focused_border: Ok(Default::default()),
                pane_group_border: Ok(Default::default()),
                panel_background: Ok(Default::default()),
                panel_focused_border: Ok(Default::default()),
                panel_indent_guide: Ok(Default::default()),
                panel_indent_guide_active: Ok(Default::default()),
                panel_indent_guide_hover: Ok(Default::default()),
                players: Ok(Default::default()),
                predictive: Ok(Default::default()),
                predictive_background: Ok(Default::default()),
                predictive_border: Ok(Default::default()),
                renamed: Ok(Default::default()),
                renamed_background: Ok(Default::default()),
                renamed_border: Ok(Default::default()),
                scrollbar_thumb_background: Ok(Default::default()),
                scrollbar_thumb_border: Ok(Default::default()),
                scrollbar_thumb_hover_background: Ok(Default::default()),
                scrollbar_track_background: Ok(Default::default()),
                scrollbar_track_border: Ok(Default::default()),
                search_match_background: Ok(Default::default()),
                status_bar_background: Ok(Default::default()),
                success: Ok(Default::default()),
                success_background: Ok(Default::default()),
                success_border: Ok(Default::default()),
                surface_background: Ok(Default::default()),
                syntax: Ok(Default::default()),
                tab_active_background: Ok(Default::default()),
                tab_bar_background: Ok(Default::default()),
                tab_inactive_background: Ok(Default::default()),
                terminal_ansi_background: Ok(Default::default()),
                terminal_ansi_black: Ok(Default::default()),
                terminal_ansi_blue: Ok(Default::default()),
                terminal_ansi_bright_black: Ok(Default::default()),
                terminal_ansi_bright_blue: Ok(Default::default()),
                terminal_ansi_bright_cyan: Ok(Default::default()),
                terminal_ansi_bright_green: Ok(Default::default()),
                terminal_ansi_bright_magenta: Ok(Default::default()),
                terminal_ansi_bright_red: Ok(Default::default()),
                terminal_ansi_bright_white: Ok(Default::default()),
                terminal_ansi_bright_yellow: Ok(Default::default()),
                terminal_ansi_cyan: Ok(Default::default()),
                terminal_ansi_dim_black: Ok(Default::default()),
                terminal_ansi_dim_blue: Ok(Default::default()),
                terminal_ansi_dim_cyan: Ok(Default::default()),
                terminal_ansi_dim_green: Ok(Default::default()),
                terminal_ansi_dim_magenta: Ok(Default::default()),
                terminal_ansi_dim_red: Ok(Default::default()),
                terminal_ansi_dim_white: Ok(Default::default()),
                terminal_ansi_dim_yellow: Ok(Default::default()),
                terminal_ansi_green: Ok(Default::default()),
                terminal_ansi_magenta: Ok(Default::default()),
                terminal_ansi_red: Ok(Default::default()),
                terminal_ansi_white: Ok(Default::default()),
                terminal_ansi_yellow: Ok(Default::default()),
                terminal_background: Ok(Default::default()),
                terminal_bright_foreground: Ok(Default::default()),
                terminal_dim_foreground: Ok(Default::default()),
                terminal_foreground: Ok(Default::default()),
                text: Ok(Default::default()),
                text_accent: Ok(Default::default()),
                text_disabled: Ok(Default::default()),
                text_muted: Ok(Default::default()),
                text_placeholder: Ok(Default::default()),
                title_bar_background: Ok(Default::default()),
                title_bar_inactive_background: Ok(Default::default()),
                toolbar_background: Ok(Default::default()),
                unreachable: Ok(Default::default()),
                unreachable_background: Ok(Default::default()),
                unreachable_border: Ok(Default::default()),
                warning: Ok(Default::default()),
                warning_background: Ok(Default::default()),
                warning_border: Ok(Default::default()),
            }
        }
    }
    impl ThemeStyleContent {
        pub fn accents<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::AccentContent>>,
            T::Error: ::std::fmt::Display,
        {
            self.accents = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for accents: {e}"));
            self
        }
        pub fn background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.background = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for background: {e}"));
            self
        }
        pub fn background_appearance<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<super::WindowBackgroundContent>>,
            T::Error: ::std::fmt::Display,
        {
            self.background_appearance = value.try_into().map_err(|e| {
                format!("error converting supplied value for background_appearance: {e}")
            });
            self
        }
        pub fn border<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.border = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for border: {e}"));
            self
        }
        pub fn border_disabled<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.border_disabled = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for border_disabled: {e}"));
            self
        }
        pub fn border_focused<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.border_focused = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for border_focused: {e}"));
            self
        }
        pub fn border_selected<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.border_selected = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for border_selected: {e}"));
            self
        }
        pub fn border_transparent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.border_transparent = value.try_into().map_err(|e| {
                format!("error converting supplied value for border_transparent: {e}")
            });
            self
        }
        pub fn border_variant<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.border_variant = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for border_variant: {e}"));
            self
        }
        pub fn conflict<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.conflict = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for conflict: {e}"));
            self
        }
        pub fn conflict_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.conflict_background = value.try_into().map_err(|e| {
                format!("error converting supplied value for conflict_background: {e}")
            });
            self
        }
        pub fn conflict_border<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.conflict_border = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for conflict_border: {e}"));
            self
        }
        pub fn created<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.created = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for created: {e}"));
            self
        }
        pub fn created_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.created_background = value.try_into().map_err(|e| {
                format!("error converting supplied value for created_background: {e}")
            });
            self
        }
        pub fn created_border<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.created_border = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for created_border: {e}"));
            self
        }
        pub fn deleted<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.deleted = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for deleted: {e}"));
            self
        }
        pub fn deleted_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.deleted_background = value.try_into().map_err(|e| {
                format!("error converting supplied value for deleted_background: {e}")
            });
            self
        }
        pub fn deleted_border<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.deleted_border = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for deleted_border: {e}"));
            self
        }
        pub fn drop_target_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.drop_target_background = value.try_into().map_err(|e| {
                format!("error converting supplied value for drop_target_background: {e}")
            });
            self
        }
        pub fn editor_active_line_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.editor_active_line_background = value.try_into().map_err(|e| {
                format!("error converting supplied value for editor_active_line_background: {e}")
            });
            self
        }
        pub fn editor_active_line_number<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.editor_active_line_number = value.try_into().map_err(|e| {
                format!("error converting supplied value for editor_active_line_number: {e}")
            });
            self
        }
        pub fn editor_active_wrap_guide<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.editor_active_wrap_guide = value.try_into().map_err(|e| {
                format!("error converting supplied value for editor_active_wrap_guide: {e}")
            });
            self
        }
        pub fn editor_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.editor_background = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for editor_background: {e}"));
            self
        }
        pub fn editor_document_highlight_bracket_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self . editor_document_highlight_bracket_background = value . try_into () . map_err (| e | format ! ("error converting supplied value for editor_document_highlight_bracket_background: {e}")) ;
            self
        }
        pub fn editor_document_highlight_read_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self . editor_document_highlight_read_background = value . try_into () . map_err (| e | format ! ("error converting supplied value for editor_document_highlight_read_background: {e}")) ;
            self
        }
        pub fn editor_document_highlight_write_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self . editor_document_highlight_write_background = value . try_into () . map_err (| e | format ! ("error converting supplied value for editor_document_highlight_write_background: {e}")) ;
            self
        }
        pub fn editor_foreground<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.editor_foreground = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for editor_foreground: {e}"));
            self
        }
        pub fn editor_gutter_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.editor_gutter_background = value.try_into().map_err(|e| {
                format!("error converting supplied value for editor_gutter_background: {e}")
            });
            self
        }
        pub fn editor_highlighted_line_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.editor_highlighted_line_background = value.try_into().map_err(|e| {
                format!(
                    "error converting supplied value for editor_highlighted_line_background: {e}"
                )
            });
            self
        }
        pub fn editor_indent_guide<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.editor_indent_guide = value.try_into().map_err(|e| {
                format!("error converting supplied value for editor_indent_guide: {e}")
            });
            self
        }
        pub fn editor_indent_guide_active<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.editor_indent_guide_active = value.try_into().map_err(|e| {
                format!("error converting supplied value for editor_indent_guide_active: {e}")
            });
            self
        }
        pub fn editor_invisible<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.editor_invisible = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for editor_invisible: {e}"));
            self
        }
        pub fn editor_line_number<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.editor_line_number = value.try_into().map_err(|e| {
                format!("error converting supplied value for editor_line_number: {e}")
            });
            self
        }
        pub fn editor_subheader_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.editor_subheader_background = value.try_into().map_err(|e| {
                format!("error converting supplied value for editor_subheader_background: {e}")
            });
            self
        }
        pub fn editor_wrap_guide<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.editor_wrap_guide = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for editor_wrap_guide: {e}"));
            self
        }
        pub fn element_active<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.element_active = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for element_active: {e}"));
            self
        }
        pub fn element_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.element_background = value.try_into().map_err(|e| {
                format!("error converting supplied value for element_background: {e}")
            });
            self
        }
        pub fn element_disabled<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.element_disabled = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for element_disabled: {e}"));
            self
        }
        pub fn element_hover<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.element_hover = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for element_hover: {e}"));
            self
        }
        pub fn element_selected<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.element_selected = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for element_selected: {e}"));
            self
        }
        pub fn elevated_surface_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.elevated_surface_background = value.try_into().map_err(|e| {
                format!("error converting supplied value for elevated_surface_background: {e}")
            });
            self
        }
        pub fn error<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.error = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for error: {e}"));
            self
        }
        pub fn error_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.error_background = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for error_background: {e}"));
            self
        }
        pub fn error_border<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.error_border = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for error_border: {e}"));
            self
        }
        pub fn ghost_element_active<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.ghost_element_active = value.try_into().map_err(|e| {
                format!("error converting supplied value for ghost_element_active: {e}")
            });
            self
        }
        pub fn ghost_element_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.ghost_element_background = value.try_into().map_err(|e| {
                format!("error converting supplied value for ghost_element_background: {e}")
            });
            self
        }
        pub fn ghost_element_disabled<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.ghost_element_disabled = value.try_into().map_err(|e| {
                format!("error converting supplied value for ghost_element_disabled: {e}")
            });
            self
        }
        pub fn ghost_element_hover<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.ghost_element_hover = value.try_into().map_err(|e| {
                format!("error converting supplied value for ghost_element_hover: {e}")
            });
            self
        }
        pub fn ghost_element_selected<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.ghost_element_selected = value.try_into().map_err(|e| {
                format!("error converting supplied value for ghost_element_selected: {e}")
            });
            self
        }
        pub fn hidden<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.hidden = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hidden: {e}"));
            self
        }
        pub fn hidden_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.hidden_background = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hidden_background: {e}"));
            self
        }
        pub fn hidden_border<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.hidden_border = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hidden_border: {e}"));
            self
        }
        pub fn hint<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.hint = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hint: {e}"));
            self
        }
        pub fn hint_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.hint_background = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hint_background: {e}"));
            self
        }
        pub fn hint_border<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.hint_border = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for hint_border: {e}"));
            self
        }
        pub fn icon<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.icon = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for icon: {e}"));
            self
        }
        pub fn icon_accent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.icon_accent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for icon_accent: {e}"));
            self
        }
        pub fn icon_disabled<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.icon_disabled = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for icon_disabled: {e}"));
            self
        }
        pub fn icon_muted<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.icon_muted = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for icon_muted: {e}"));
            self
        }
        pub fn icon_placeholder<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.icon_placeholder = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for icon_placeholder: {e}"));
            self
        }
        pub fn ignored<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.ignored = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ignored: {e}"));
            self
        }
        pub fn ignored_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.ignored_background = value.try_into().map_err(|e| {
                format!("error converting supplied value for ignored_background: {e}")
            });
            self
        }
        pub fn ignored_border<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.ignored_border = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for ignored_border: {e}"));
            self
        }
        pub fn info<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.info = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for info: {e}"));
            self
        }
        pub fn info_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.info_background = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for info_background: {e}"));
            self
        }
        pub fn info_border<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.info_border = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for info_border: {e}"));
            self
        }
        pub fn link_text_hover<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.link_text_hover = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for link_text_hover: {e}"));
            self
        }
        pub fn modified<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.modified = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for modified: {e}"));
            self
        }
        pub fn modified_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.modified_background = value.try_into().map_err(|e| {
                format!("error converting supplied value for modified_background: {e}")
            });
            self
        }
        pub fn modified_border<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.modified_border = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for modified_border: {e}"));
            self
        }
        pub fn pane_focused_border<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.pane_focused_border = value.try_into().map_err(|e| {
                format!("error converting supplied value for pane_focused_border: {e}")
            });
            self
        }
        pub fn pane_group_border<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.pane_group_border = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for pane_group_border: {e}"));
            self
        }
        pub fn panel_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.panel_background = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for panel_background: {e}"));
            self
        }
        pub fn panel_focused_border<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.panel_focused_border = value.try_into().map_err(|e| {
                format!("error converting supplied value for panel_focused_border: {e}")
            });
            self
        }
        pub fn panel_indent_guide<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.panel_indent_guide = value.try_into().map_err(|e| {
                format!("error converting supplied value for panel_indent_guide: {e}")
            });
            self
        }
        pub fn panel_indent_guide_active<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.panel_indent_guide_active = value.try_into().map_err(|e| {
                format!("error converting supplied value for panel_indent_guide_active: {e}")
            });
            self
        }
        pub fn panel_indent_guide_hover<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.panel_indent_guide_hover = value.try_into().map_err(|e| {
                format!("error converting supplied value for panel_indent_guide_hover: {e}")
            });
            self
        }
        pub fn players<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::vec::Vec<super::PlayerColorContent>>,
            T::Error: ::std::fmt::Display,
        {
            self.players = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for players: {e}"));
            self
        }
        pub fn predictive<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.predictive = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for predictive: {e}"));
            self
        }
        pub fn predictive_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.predictive_background = value.try_into().map_err(|e| {
                format!("error converting supplied value for predictive_background: {e}")
            });
            self
        }
        pub fn predictive_border<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.predictive_border = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for predictive_border: {e}"));
            self
        }
        pub fn renamed<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.renamed = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for renamed: {e}"));
            self
        }
        pub fn renamed_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.renamed_background = value.try_into().map_err(|e| {
                format!("error converting supplied value for renamed_background: {e}")
            });
            self
        }
        pub fn renamed_border<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.renamed_border = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for renamed_border: {e}"));
            self
        }
        pub fn scrollbar_thumb_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.scrollbar_thumb_background = value.try_into().map_err(|e| {
                format!("error converting supplied value for scrollbar_thumb_background: {e}")
            });
            self
        }
        pub fn scrollbar_thumb_border<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.scrollbar_thumb_border = value.try_into().map_err(|e| {
                format!("error converting supplied value for scrollbar_thumb_border: {e}")
            });
            self
        }
        pub fn scrollbar_thumb_hover_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.scrollbar_thumb_hover_background = value.try_into().map_err(|e| {
                format!("error converting supplied value for scrollbar_thumb_hover_background: {e}")
            });
            self
        }
        pub fn scrollbar_track_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.scrollbar_track_background = value.try_into().map_err(|e| {
                format!("error converting supplied value for scrollbar_track_background: {e}")
            });
            self
        }
        pub fn scrollbar_track_border<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.scrollbar_track_border = value.try_into().map_err(|e| {
                format!("error converting supplied value for scrollbar_track_border: {e}")
            });
            self
        }
        pub fn search_match_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.search_match_background = value.try_into().map_err(|e| {
                format!("error converting supplied value for search_match_background: {e}")
            });
            self
        }
        pub fn status_bar_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.status_bar_background = value.try_into().map_err(|e| {
                format!("error converting supplied value for status_bar_background: {e}")
            });
            self
        }
        pub fn success<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.success = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for success: {e}"));
            self
        }
        pub fn success_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.success_background = value.try_into().map_err(|e| {
                format!("error converting supplied value for success_background: {e}")
            });
            self
        }
        pub fn success_border<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.success_border = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for success_border: {e}"));
            self
        }
        pub fn surface_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.surface_background = value.try_into().map_err(|e| {
                format!("error converting supplied value for surface_background: {e}")
            });
            self
        }
        pub fn syntax<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<
                    ::std::collections::HashMap<
                        ::std::string::String,
                        super::HighlightStyleContent,
                    >,
                >,
            T::Error: ::std::fmt::Display,
        {
            self.syntax = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for syntax: {e}"));
            self
        }
        pub fn tab_active_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.tab_active_background = value.try_into().map_err(|e| {
                format!("error converting supplied value for tab_active_background: {e}")
            });
            self
        }
        pub fn tab_bar_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.tab_bar_background = value.try_into().map_err(|e| {
                format!("error converting supplied value for tab_bar_background: {e}")
            });
            self
        }
        pub fn tab_inactive_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.tab_inactive_background = value.try_into().map_err(|e| {
                format!("error converting supplied value for tab_inactive_background: {e}")
            });
            self
        }
        pub fn terminal_ansi_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.terminal_ansi_background = value.try_into().map_err(|e| {
                format!("error converting supplied value for terminal_ansi_background: {e}")
            });
            self
        }
        pub fn terminal_ansi_black<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.terminal_ansi_black = value.try_into().map_err(|e| {
                format!("error converting supplied value for terminal_ansi_black: {e}")
            });
            self
        }
        pub fn terminal_ansi_blue<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.terminal_ansi_blue = value.try_into().map_err(|e| {
                format!("error converting supplied value for terminal_ansi_blue: {e}")
            });
            self
        }
        pub fn terminal_ansi_bright_black<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.terminal_ansi_bright_black = value.try_into().map_err(|e| {
                format!("error converting supplied value for terminal_ansi_bright_black: {e}")
            });
            self
        }
        pub fn terminal_ansi_bright_blue<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.terminal_ansi_bright_blue = value.try_into().map_err(|e| {
                format!("error converting supplied value for terminal_ansi_bright_blue: {e}")
            });
            self
        }
        pub fn terminal_ansi_bright_cyan<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.terminal_ansi_bright_cyan = value.try_into().map_err(|e| {
                format!("error converting supplied value for terminal_ansi_bright_cyan: {e}")
            });
            self
        }
        pub fn terminal_ansi_bright_green<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.terminal_ansi_bright_green = value.try_into().map_err(|e| {
                format!("error converting supplied value for terminal_ansi_bright_green: {e}")
            });
            self
        }
        pub fn terminal_ansi_bright_magenta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.terminal_ansi_bright_magenta = value.try_into().map_err(|e| {
                format!("error converting supplied value for terminal_ansi_bright_magenta: {e}")
            });
            self
        }
        pub fn terminal_ansi_bright_red<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.terminal_ansi_bright_red = value.try_into().map_err(|e| {
                format!("error converting supplied value for terminal_ansi_bright_red: {e}")
            });
            self
        }
        pub fn terminal_ansi_bright_white<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.terminal_ansi_bright_white = value.try_into().map_err(|e| {
                format!("error converting supplied value for terminal_ansi_bright_white: {e}")
            });
            self
        }
        pub fn terminal_ansi_bright_yellow<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.terminal_ansi_bright_yellow = value.try_into().map_err(|e| {
                format!("error converting supplied value for terminal_ansi_bright_yellow: {e}")
            });
            self
        }
        pub fn terminal_ansi_cyan<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.terminal_ansi_cyan = value.try_into().map_err(|e| {
                format!("error converting supplied value for terminal_ansi_cyan: {e}")
            });
            self
        }
        pub fn terminal_ansi_dim_black<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.terminal_ansi_dim_black = value.try_into().map_err(|e| {
                format!("error converting supplied value for terminal_ansi_dim_black: {e}")
            });
            self
        }
        pub fn terminal_ansi_dim_blue<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.terminal_ansi_dim_blue = value.try_into().map_err(|e| {
                format!("error converting supplied value for terminal_ansi_dim_blue: {e}")
            });
            self
        }
        pub fn terminal_ansi_dim_cyan<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.terminal_ansi_dim_cyan = value.try_into().map_err(|e| {
                format!("error converting supplied value for terminal_ansi_dim_cyan: {e}")
            });
            self
        }
        pub fn terminal_ansi_dim_green<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.terminal_ansi_dim_green = value.try_into().map_err(|e| {
                format!("error converting supplied value for terminal_ansi_dim_green: {e}")
            });
            self
        }
        pub fn terminal_ansi_dim_magenta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.terminal_ansi_dim_magenta = value.try_into().map_err(|e| {
                format!("error converting supplied value for terminal_ansi_dim_magenta: {e}")
            });
            self
        }
        pub fn terminal_ansi_dim_red<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.terminal_ansi_dim_red = value.try_into().map_err(|e| {
                format!("error converting supplied value for terminal_ansi_dim_red: {e}")
            });
            self
        }
        pub fn terminal_ansi_dim_white<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.terminal_ansi_dim_white = value.try_into().map_err(|e| {
                format!("error converting supplied value for terminal_ansi_dim_white: {e}")
            });
            self
        }
        pub fn terminal_ansi_dim_yellow<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.terminal_ansi_dim_yellow = value.try_into().map_err(|e| {
                format!("error converting supplied value for terminal_ansi_dim_yellow: {e}")
            });
            self
        }
        pub fn terminal_ansi_green<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.terminal_ansi_green = value.try_into().map_err(|e| {
                format!("error converting supplied value for terminal_ansi_green: {e}")
            });
            self
        }
        pub fn terminal_ansi_magenta<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.terminal_ansi_magenta = value.try_into().map_err(|e| {
                format!("error converting supplied value for terminal_ansi_magenta: {e}")
            });
            self
        }
        pub fn terminal_ansi_red<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.terminal_ansi_red = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for terminal_ansi_red: {e}"));
            self
        }
        pub fn terminal_ansi_white<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.terminal_ansi_white = value.try_into().map_err(|e| {
                format!("error converting supplied value for terminal_ansi_white: {e}")
            });
            self
        }
        pub fn terminal_ansi_yellow<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.terminal_ansi_yellow = value.try_into().map_err(|e| {
                format!("error converting supplied value for terminal_ansi_yellow: {e}")
            });
            self
        }
        pub fn terminal_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.terminal_background = value.try_into().map_err(|e| {
                format!("error converting supplied value for terminal_background: {e}")
            });
            self
        }
        pub fn terminal_bright_foreground<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.terminal_bright_foreground = value.try_into().map_err(|e| {
                format!("error converting supplied value for terminal_bright_foreground: {e}")
            });
            self
        }
        pub fn terminal_dim_foreground<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.terminal_dim_foreground = value.try_into().map_err(|e| {
                format!("error converting supplied value for terminal_dim_foreground: {e}")
            });
            self
        }
        pub fn terminal_foreground<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.terminal_foreground = value.try_into().map_err(|e| {
                format!("error converting supplied value for terminal_foreground: {e}")
            });
            self
        }
        pub fn text<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.text = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text: {e}"));
            self
        }
        pub fn text_accent<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.text_accent = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text_accent: {e}"));
            self
        }
        pub fn text_disabled<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.text_disabled = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text_disabled: {e}"));
            self
        }
        pub fn text_muted<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.text_muted = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text_muted: {e}"));
            self
        }
        pub fn text_placeholder<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.text_placeholder = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for text_placeholder: {e}"));
            self
        }
        pub fn title_bar_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title_bar_background = value.try_into().map_err(|e| {
                format!("error converting supplied value for title_bar_background: {e}")
            });
            self
        }
        pub fn title_bar_inactive_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.title_bar_inactive_background = value.try_into().map_err(|e| {
                format!("error converting supplied value for title_bar_inactive_background: {e}")
            });
            self
        }
        pub fn toolbar_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.toolbar_background = value.try_into().map_err(|e| {
                format!("error converting supplied value for toolbar_background: {e}")
            });
            self
        }
        pub fn unreachable<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.unreachable = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for unreachable: {e}"));
            self
        }
        pub fn unreachable_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.unreachable_background = value.try_into().map_err(|e| {
                format!("error converting supplied value for unreachable_background: {e}")
            });
            self
        }
        pub fn unreachable_border<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.unreachable_border = value.try_into().map_err(|e| {
                format!("error converting supplied value for unreachable_border: {e}")
            });
            self
        }
        pub fn warning<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.warning = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for warning: {e}"));
            self
        }
        pub fn warning_background<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.warning_background = value.try_into().map_err(|e| {
                format!("error converting supplied value for warning_background: {e}")
            });
            self
        }
        pub fn warning_border<T>(mut self, value: T) -> Self
        where
            T: ::std::convert::TryInto<::std::option::Option<::std::string::String>>,
            T::Error: ::std::fmt::Display,
        {
            self.warning_border = value
                .try_into()
                .map_err(|e| format!("error converting supplied value for warning_border: {e}"));
            self
        }
    }
    impl ::std::convert::TryFrom<ThemeStyleContent> for super::ThemeStyleContent {
        type Error = super::error::ConversionError;
        fn try_from(
            value: ThemeStyleContent,
        ) -> ::std::result::Result<Self, super::error::ConversionError> {
            Ok(Self {
                accents: value.accents?,
                background: value.background?,
                background_appearance: value.background_appearance?,
                border: value.border?,
                border_disabled: value.border_disabled?,
                border_focused: value.border_focused?,
                border_selected: value.border_selected?,
                border_transparent: value.border_transparent?,
                border_variant: value.border_variant?,
                conflict: value.conflict?,
                conflict_background: value.conflict_background?,
                conflict_border: value.conflict_border?,
                created: value.created?,
                created_background: value.created_background?,
                created_border: value.created_border?,
                deleted: value.deleted?,
                deleted_background: value.deleted_background?,
                deleted_border: value.deleted_border?,
                drop_target_background: value.drop_target_background?,
                editor_active_line_background: value.editor_active_line_background?,
                editor_active_line_number: value.editor_active_line_number?,
                editor_active_wrap_guide: value.editor_active_wrap_guide?,
                editor_background: value.editor_background?,
                editor_document_highlight_bracket_background: value
                    .editor_document_highlight_bracket_background?,
                editor_document_highlight_read_background: value
                    .editor_document_highlight_read_background?,
                editor_document_highlight_write_background: value
                    .editor_document_highlight_write_background?,
                editor_foreground: value.editor_foreground?,
                editor_gutter_background: value.editor_gutter_background?,
                editor_highlighted_line_background: value.editor_highlighted_line_background?,
                editor_indent_guide: value.editor_indent_guide?,
                editor_indent_guide_active: value.editor_indent_guide_active?,
                editor_invisible: value.editor_invisible?,
                editor_line_number: value.editor_line_number?,
                editor_subheader_background: value.editor_subheader_background?,
                editor_wrap_guide: value.editor_wrap_guide?,
                element_active: value.element_active?,
                element_background: value.element_background?,
                element_disabled: value.element_disabled?,
                element_hover: value.element_hover?,
                element_selected: value.element_selected?,
                elevated_surface_background: value.elevated_surface_background?,
                error: value.error?,
                error_background: value.error_background?,
                error_border: value.error_border?,
                ghost_element_active: value.ghost_element_active?,
                ghost_element_background: value.ghost_element_background?,
                ghost_element_disabled: value.ghost_element_disabled?,
                ghost_element_hover: value.ghost_element_hover?,
                ghost_element_selected: value.ghost_element_selected?,
                hidden: value.hidden?,
                hidden_background: value.hidden_background?,
                hidden_border: value.hidden_border?,
                hint: value.hint?,
                hint_background: value.hint_background?,
                hint_border: value.hint_border?,
                icon: value.icon?,
                icon_accent: value.icon_accent?,
                icon_disabled: value.icon_disabled?,
                icon_muted: value.icon_muted?,
                icon_placeholder: value.icon_placeholder?,
                ignored: value.ignored?,
                ignored_background: value.ignored_background?,
                ignored_border: value.ignored_border?,
                info: value.info?,
                info_background: value.info_background?,
                info_border: value.info_border?,
                link_text_hover: value.link_text_hover?,
                modified: value.modified?,
                modified_background: value.modified_background?,
                modified_border: value.modified_border?,
                pane_focused_border: value.pane_focused_border?,
                pane_group_border: value.pane_group_border?,
                panel_background: value.panel_background?,
                panel_focused_border: value.panel_focused_border?,
                panel_indent_guide: value.panel_indent_guide?,
                panel_indent_guide_active: value.panel_indent_guide_active?,
                panel_indent_guide_hover: value.panel_indent_guide_hover?,
                players: value.players?,
                predictive: value.predictive?,
                predictive_background: value.predictive_background?,
                predictive_border: value.predictive_border?,
                renamed: value.renamed?,
                renamed_background: value.renamed_background?,
                renamed_border: value.renamed_border?,
                scrollbar_thumb_background: value.scrollbar_thumb_background?,
                scrollbar_thumb_border: value.scrollbar_thumb_border?,
                scrollbar_thumb_hover_background: value.scrollbar_thumb_hover_background?,
                scrollbar_track_background: value.scrollbar_track_background?,
                scrollbar_track_border: value.scrollbar_track_border?,
                search_match_background: value.search_match_background?,
                status_bar_background: value.status_bar_background?,
                success: value.success?,
                success_background: value.success_background?,
                success_border: value.success_border?,
                surface_background: value.surface_background?,
                syntax: value.syntax?,
                tab_active_background: value.tab_active_background?,
                tab_bar_background: value.tab_bar_background?,
                tab_inactive_background: value.tab_inactive_background?,
                terminal_ansi_background: value.terminal_ansi_background?,
                terminal_ansi_black: value.terminal_ansi_black?,
                terminal_ansi_blue: value.terminal_ansi_blue?,
                terminal_ansi_bright_black: value.terminal_ansi_bright_black?,
                terminal_ansi_bright_blue: value.terminal_ansi_bright_blue?,
                terminal_ansi_bright_cyan: value.terminal_ansi_bright_cyan?,
                terminal_ansi_bright_green: value.terminal_ansi_bright_green?,
                terminal_ansi_bright_magenta: value.terminal_ansi_bright_magenta?,
                terminal_ansi_bright_red: value.terminal_ansi_bright_red?,
                terminal_ansi_bright_white: value.terminal_ansi_bright_white?,
                terminal_ansi_bright_yellow: value.terminal_ansi_bright_yellow?,
                terminal_ansi_cyan: value.terminal_ansi_cyan?,
                terminal_ansi_dim_black: value.terminal_ansi_dim_black?,
                terminal_ansi_dim_blue: value.terminal_ansi_dim_blue?,
                terminal_ansi_dim_cyan: value.terminal_ansi_dim_cyan?,
                terminal_ansi_dim_green: value.terminal_ansi_dim_green?,
                terminal_ansi_dim_magenta: value.terminal_ansi_dim_magenta?,
                terminal_ansi_dim_red: value.terminal_ansi_dim_red?,
                terminal_ansi_dim_white: value.terminal_ansi_dim_white?,
                terminal_ansi_dim_yellow: value.terminal_ansi_dim_yellow?,
                terminal_ansi_green: value.terminal_ansi_green?,
                terminal_ansi_magenta: value.terminal_ansi_magenta?,
                terminal_ansi_red: value.terminal_ansi_red?,
                terminal_ansi_white: value.terminal_ansi_white?,
                terminal_ansi_yellow: value.terminal_ansi_yellow?,
                terminal_background: value.terminal_background?,
                terminal_bright_foreground: value.terminal_bright_foreground?,
                terminal_dim_foreground: value.terminal_dim_foreground?,
                terminal_foreground: value.terminal_foreground?,
                text: value.text?,
                text_accent: value.text_accent?,
                text_disabled: value.text_disabled?,
                text_muted: value.text_muted?,
                text_placeholder: value.text_placeholder?,
                title_bar_background: value.title_bar_background?,
                title_bar_inactive_background: value.title_bar_inactive_background?,
                toolbar_background: value.toolbar_background?,
                unreachable: value.unreachable?,
                unreachable_background: value.unreachable_background?,
                unreachable_border: value.unreachable_border?,
                warning: value.warning?,
                warning_background: value.warning_background?,
                warning_border: value.warning_border?,
            })
        }
    }
    impl ::std::convert::From<super::ThemeStyleContent> for ThemeStyleContent {
        fn from(value: super::ThemeStyleContent) -> Self {
            Self {
                accents: Ok(value.accents),
                background: Ok(value.background),
                background_appearance: Ok(value.background_appearance),
                border: Ok(value.border),
                border_disabled: Ok(value.border_disabled),
                border_focused: Ok(value.border_focused),
                border_selected: Ok(value.border_selected),
                border_transparent: Ok(value.border_transparent),
                border_variant: Ok(value.border_variant),
                conflict: Ok(value.conflict),
                conflict_background: Ok(value.conflict_background),
                conflict_border: Ok(value.conflict_border),
                created: Ok(value.created),
                created_background: Ok(value.created_background),
                created_border: Ok(value.created_border),
                deleted: Ok(value.deleted),
                deleted_background: Ok(value.deleted_background),
                deleted_border: Ok(value.deleted_border),
                drop_target_background: Ok(value.drop_target_background),
                editor_active_line_background: Ok(value.editor_active_line_background),
                editor_active_line_number: Ok(value.editor_active_line_number),
                editor_active_wrap_guide: Ok(value.editor_active_wrap_guide),
                editor_background: Ok(value.editor_background),
                editor_document_highlight_bracket_background: Ok(
                    value.editor_document_highlight_bracket_background
                ),
                editor_document_highlight_read_background: Ok(
                    value.editor_document_highlight_read_background
                ),
                editor_document_highlight_write_background: Ok(
                    value.editor_document_highlight_write_background
                ),
                editor_foreground: Ok(value.editor_foreground),
                editor_gutter_background: Ok(value.editor_gutter_background),
                editor_highlighted_line_background: Ok(value.editor_highlighted_line_background),
                editor_indent_guide: Ok(value.editor_indent_guide),
                editor_indent_guide_active: Ok(value.editor_indent_guide_active),
                editor_invisible: Ok(value.editor_invisible),
                editor_line_number: Ok(value.editor_line_number),
                editor_subheader_background: Ok(value.editor_subheader_background),
                editor_wrap_guide: Ok(value.editor_wrap_guide),
                element_active: Ok(value.element_active),
                element_background: Ok(value.element_background),
                element_disabled: Ok(value.element_disabled),
                element_hover: Ok(value.element_hover),
                element_selected: Ok(value.element_selected),
                elevated_surface_background: Ok(value.elevated_surface_background),
                error: Ok(value.error),
                error_background: Ok(value.error_background),
                error_border: Ok(value.error_border),
                ghost_element_active: Ok(value.ghost_element_active),
                ghost_element_background: Ok(value.ghost_element_background),
                ghost_element_disabled: Ok(value.ghost_element_disabled),
                ghost_element_hover: Ok(value.ghost_element_hover),
                ghost_element_selected: Ok(value.ghost_element_selected),
                hidden: Ok(value.hidden),
                hidden_background: Ok(value.hidden_background),
                hidden_border: Ok(value.hidden_border),
                hint: Ok(value.hint),
                hint_background: Ok(value.hint_background),
                hint_border: Ok(value.hint_border),
                icon: Ok(value.icon),
                icon_accent: Ok(value.icon_accent),
                icon_disabled: Ok(value.icon_disabled),
                icon_muted: Ok(value.icon_muted),
                icon_placeholder: Ok(value.icon_placeholder),
                ignored: Ok(value.ignored),
                ignored_background: Ok(value.ignored_background),
                ignored_border: Ok(value.ignored_border),
                info: Ok(value.info),
                info_background: Ok(value.info_background),
                info_border: Ok(value.info_border),
                link_text_hover: Ok(value.link_text_hover),
                modified: Ok(value.modified),
                modified_background: Ok(value.modified_background),
                modified_border: Ok(value.modified_border),
                pane_focused_border: Ok(value.pane_focused_border),
                pane_group_border: Ok(value.pane_group_border),
                panel_background: Ok(value.panel_background),
                panel_focused_border: Ok(value.panel_focused_border),
                panel_indent_guide: Ok(value.panel_indent_guide),
                panel_indent_guide_active: Ok(value.panel_indent_guide_active),
                panel_indent_guide_hover: Ok(value.panel_indent_guide_hover),
                players: Ok(value.players),
                predictive: Ok(value.predictive),
                predictive_background: Ok(value.predictive_background),
                predictive_border: Ok(value.predictive_border),
                renamed: Ok(value.renamed),
                renamed_background: Ok(value.renamed_background),
                renamed_border: Ok(value.renamed_border),
                scrollbar_thumb_background: Ok(value.scrollbar_thumb_background),
                scrollbar_thumb_border: Ok(value.scrollbar_thumb_border),
                scrollbar_thumb_hover_background: Ok(value.scrollbar_thumb_hover_background),
                scrollbar_track_background: Ok(value.scrollbar_track_background),
                scrollbar_track_border: Ok(value.scrollbar_track_border),
                search_match_background: Ok(value.search_match_background),
                status_bar_background: Ok(value.status_bar_background),
                success: Ok(value.success),
                success_background: Ok(value.success_background),
                success_border: Ok(value.success_border),
                surface_background: Ok(value.surface_background),
                syntax: Ok(value.syntax),
                tab_active_background: Ok(value.tab_active_background),
                tab_bar_background: Ok(value.tab_bar_background),
                tab_inactive_background: Ok(value.tab_inactive_background),
                terminal_ansi_background: Ok(value.terminal_ansi_background),
                terminal_ansi_black: Ok(value.terminal_ansi_black),
                terminal_ansi_blue: Ok(value.terminal_ansi_blue),
                terminal_ansi_bright_black: Ok(value.terminal_ansi_bright_black),
                terminal_ansi_bright_blue: Ok(value.terminal_ansi_bright_blue),
                terminal_ansi_bright_cyan: Ok(value.terminal_ansi_bright_cyan),
                terminal_ansi_bright_green: Ok(value.terminal_ansi_bright_green),
                terminal_ansi_bright_magenta: Ok(value.terminal_ansi_bright_magenta),
                terminal_ansi_bright_red: Ok(value.terminal_ansi_bright_red),
                terminal_ansi_bright_white: Ok(value.terminal_ansi_bright_white),
                terminal_ansi_bright_yellow: Ok(value.terminal_ansi_bright_yellow),
                terminal_ansi_cyan: Ok(value.terminal_ansi_cyan),
                terminal_ansi_dim_black: Ok(value.terminal_ansi_dim_black),
                terminal_ansi_dim_blue: Ok(value.terminal_ansi_dim_blue),
                terminal_ansi_dim_cyan: Ok(value.terminal_ansi_dim_cyan),
                terminal_ansi_dim_green: Ok(value.terminal_ansi_dim_green),
                terminal_ansi_dim_magenta: Ok(value.terminal_ansi_dim_magenta),
                terminal_ansi_dim_red: Ok(value.terminal_ansi_dim_red),
                terminal_ansi_dim_white: Ok(value.terminal_ansi_dim_white),
                terminal_ansi_dim_yellow: Ok(value.terminal_ansi_dim_yellow),
                terminal_ansi_green: Ok(value.terminal_ansi_green),
                terminal_ansi_magenta: Ok(value.terminal_ansi_magenta),
                terminal_ansi_red: Ok(value.terminal_ansi_red),
                terminal_ansi_white: Ok(value.terminal_ansi_white),
                terminal_ansi_yellow: Ok(value.terminal_ansi_yellow),
                terminal_background: Ok(value.terminal_background),
                terminal_bright_foreground: Ok(value.terminal_bright_foreground),
                terminal_dim_foreground: Ok(value.terminal_dim_foreground),
                terminal_foreground: Ok(value.terminal_foreground),
                text: Ok(value.text),
                text_accent: Ok(value.text_accent),
                text_disabled: Ok(value.text_disabled),
                text_muted: Ok(value.text_muted),
                text_placeholder: Ok(value.text_placeholder),
                title_bar_background: Ok(value.title_bar_background),
                title_bar_inactive_background: Ok(value.title_bar_inactive_background),
                toolbar_background: Ok(value.toolbar_background),
                unreachable: Ok(value.unreachable),
                unreachable_background: Ok(value.unreachable_background),
                unreachable_border: Ok(value.unreachable_border),
                warning: Ok(value.warning),
                warning_background: Ok(value.warning_background),
                warning_border: Ok(value.warning_border),
            }
        }
    }
}

pub type ZedThemeSpec = ThemeFamilyContent;
