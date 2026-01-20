//! Language codes for internationalization support.
//!
//! This module provides the `Language` enum which corresponds to wxWidgets' wxLanguage enum.
//! It contains all language codes supported by wxWidgets for use with the translations system.

/// Language codes for internationalization.
///
/// These values correspond to wxWidgets' wxLanguage enum and are used with
/// [`Translations`](crate::translations::Translations) to set the UI language.
///
/// # Example
/// ```rust,no_run
/// use wxdragon::prelude::*;
///
/// let translations = Translations::new();
/// translations.set_language(Language::French);
/// ```
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(non_camel_case_types)]
pub enum Language {
    /// Use system default language
    #[default]
    Default = 0,
    /// Unknown language
    Unknown = 1,
    /// Abkhazian
    Abkhazian = 2,
    /// Afar
    Afar = 3,
    /// Afrikaans
    Afrikaans = 4,
    /// Albanian
    Albanian = 5,
    /// Amharic
    Amharic = 6,
    /// Arabic
    Arabic = 7,
    /// Arabic (Algeria)
    Arabic_Algeria = 8,
    /// Arabic (Bahrain)
    Arabic_Bahrain = 9,
    /// Arabic (Egypt)
    Arabic_Egypt = 10,
    /// Arabic (Iraq)
    Arabic_Iraq = 11,
    /// Arabic (Jordan)
    Arabic_Jordan = 12,
    /// Arabic (Kuwait)
    Arabic_Kuwait = 13,
    /// Arabic (Lebanon)
    Arabic_Lebanon = 14,
    /// Arabic (Libya)
    Arabic_Libya = 15,
    /// Arabic (Morocco)
    Arabic_Morocco = 16,
    /// Arabic (Oman)
    Arabic_Oman = 17,
    /// Arabic (Qatar)
    Arabic_Qatar = 18,
    /// Arabic (Saudi Arabia)
    Arabic_SaudiArabia = 19,
    /// Arabic (Sudan)
    Arabic_Sudan = 20,
    /// Arabic (Syria)
    Arabic_Syria = 21,
    /// Arabic (Tunisia)
    Arabic_Tunisia = 22,
    /// Arabic (UAE)
    Arabic_Uae = 23,
    /// Arabic (Yemen)
    Arabic_Yemen = 24,
    /// Armenian
    Armenian = 25,
    /// Assamese
    Assamese = 26,
    /// Asturian
    Asturian = 27,
    /// Aymara
    Aymara = 28,
    /// Azerbaijani
    Azerbaijani = 29,
    /// Azerbaijani (Cyrillic)
    Azerbaijani_Cyrillic = 30,
    /// Bashkir
    Bashkir = 31,
    /// Basque
    Basque = 32,
    /// Belarusian
    Belarusian = 33,
    /// Bengali
    Bengali = 34,
    /// Bengali (India)
    Bengali_India = 35,
    /// Bhutani
    Bhutani = 36,
    /// Bihari
    Bihari = 37,
    /// Bislama
    Bislama = 38,
    /// Bosnian
    Bosnian = 39,
    /// Breton
    Breton = 40,
    /// Bulgarian
    Bulgarian = 41,
    /// Burmese
    Burmese = 42,
    /// Cambodian
    Cambodian = 43,
    /// Catalan
    Catalan = 44,
    /// Chinese
    Chinese = 45,
    /// Chinese (Simplified)
    Chinese_Simplified = 46,
    /// Chinese (Traditional)
    Chinese_Traditional = 47,
    /// Chinese (Hong Kong)
    Chinese_Hongkong = 48,
    /// Chinese (Macau)
    Chinese_Macau = 49,
    /// Chinese (Singapore)
    Chinese_Singapore = 50,
    /// Chinese (Taiwan)
    Chinese_Taiwan = 51,
    /// Corsican
    Corsican = 52,
    /// Croatian
    Croatian = 53,
    /// Czech
    Czech = 54,
    /// Danish
    Danish = 55,
    /// Dutch
    Dutch = 56,
    /// Dutch (Belgian)
    Dutch_Belgian = 57,
    /// English
    English = 58,
    /// English (UK)
    English_Uk = 59,
    /// English (US)
    English_Us = 60,
    /// English (Australia)
    English_Australia = 61,
    /// English (Belize)
    English_Belize = 62,
    /// English (Botswana)
    English_Botswana = 63,
    /// English (Canada)
    English_Canada = 64,
    /// English (Caribbean)
    English_Caribbean = 65,
    /// English (Denmark)
    English_Denmark = 66,
    /// English (Eire / Ireland)
    English_Eire = 67,
    /// English (Jamaica)
    English_Jamaica = 68,
    /// English (New Zealand)
    English_NewZealand = 69,
    /// English (Philippines)
    English_Philippines = 70,
    /// English (South Africa)
    English_SouthAfrica = 71,
    /// English (Trinidad)
    English_Trinidad = 72,
    /// English (Zimbabwe)
    English_Zimbabwe = 73,
    /// Esperanto
    Esperanto = 74,
    /// Estonian
    Estonian = 75,
    /// Faeroese
    Faeroese = 76,
    /// Farsi
    Farsi = 77,
    /// Fiji
    Fiji = 78,
    /// Finnish
    Finnish = 79,
    /// French
    French = 80,
    /// French (Belgian)
    French_Belgian = 81,
    /// French (Canada)
    French_Canadian = 82,
    /// French (Luxembourg)
    French_Luxembourg = 83,
    /// French (Monaco)
    French_Monaco = 84,
    /// French (Swiss)
    French_Swiss = 85,
    /// Frisian
    Frisian = 86,
    /// Galician
    Galician = 87,
    /// Georgian
    Georgian = 88,
    /// German
    German = 89,
    /// German (Austrian)
    German_Austrian = 90,
    /// German (Belgium)
    German_Belgium = 91,
    /// German (Liechtenstein)
    German_Liechtenstein = 92,
    /// German (Luxembourg)
    German_Luxembourg = 93,
    /// German (Swiss)
    German_Swiss = 94,
    /// Greek
    Greek = 95,
    /// Greenlandic
    Greenlandic = 96,
    /// Guarani
    Guarani = 97,
    /// Gujarati
    Gujarati = 98,
    /// Hausa
    Hausa = 99,
    /// Hebrew
    Hebrew = 100,
    /// Hindi
    Hindi = 101,
    /// Hungarian
    Hungarian = 102,
    /// Icelandic
    Icelandic = 103,
    /// Indonesian
    Indonesian = 104,
    /// Interlingua
    Interlingua = 105,
    /// Interlingue
    Interlingue = 106,
    /// Inuktitut
    Inuktitut = 107,
    /// Inupiak
    Inupiak = 108,
    /// Irish
    Irish = 109,
    /// Italian
    Italian = 110,
    /// Italian (Swiss)
    Italian_Swiss = 111,
    /// Japanese
    Japanese = 112,
    /// Javanese
    Javanese = 113,
    /// Kannada
    Kannada = 114,
    /// Kashmiri
    Kashmiri = 115,
    /// Kashmiri (India)
    Kashmiri_India = 116,
    /// Kazakh
    Kazakh = 117,
    /// Kernewek
    Kernewek = 118,
    /// Kinyarwanda
    Kinyarwanda = 119,
    /// Kirghiz
    Kirghiz = 120,
    /// Kirundi
    Kirundi = 121,
    /// Konkani
    Konkani = 122,
    /// Korean
    Korean = 123,
    /// Kurdish
    Kurdish = 124,
    /// Laothian
    Laothian = 125,
    /// Latin
    Latin = 126,
    /// Latvian
    Latvian = 127,
    /// Lingala
    Lingala = 128,
    /// Lithuanian
    Lithuanian = 129,
    /// Macedonian
    Macedonian = 130,
    /// Malagasy
    Malagasy = 131,
    /// Malay
    Malay = 132,
    /// Malay (Brunei Darussalam)
    Malay_BruneiDarussalam = 133,
    /// Malay (Malaysia)
    Malay_Malaysia = 134,
    /// Malayalam
    Malayalam = 135,
    /// Maltese
    Maltese = 136,
    /// Manipuri
    Manipuri = 137,
    /// Maori
    Maori = 138,
    /// Marathi
    Marathi = 139,
    /// Moldavian
    Moldavian = 140,
    /// Mongolian
    Mongolian = 141,
    /// Nauru
    Nauru = 142,
    /// Nepali
    Nepali = 143,
    /// Nepali (India)
    Nepali_India = 144,
    /// Norwegian (Bokmal)
    Norwegian_Bokmal = 145,
    /// Norwegian (Nynorsk)
    Norwegian_Nynorsk = 146,
    /// Occitan
    Occitan = 147,
    /// Oriya
    Oriya = 148,
    /// Oromo
    Oromo = 149,
    /// Pashto
    Pashto = 150,
    /// Polish
    Polish = 151,
    /// Portuguese
    Portuguese = 152,
    /// Portuguese (Brazilian)
    Portuguese_Brazilian = 153,
    /// Punjabi
    Punjabi = 154,
    /// Quechua
    Quechua = 155,
    /// RhaetoRomance
    RhaetoRomance = 156,
    /// Romanian
    Romanian = 157,
    /// Russian
    Russian = 158,
    /// Russian (Ukraine)
    Russian_Ukraine = 159,
    /// Sami
    Sami = 160,
    /// Samoan
    Samoan = 161,
    /// Sangho
    Sangho = 162,
    /// Sanskrit
    Sanskrit = 163,
    /// Scots Gaelic
    ScotsGaelic = 164,
    /// Serbian
    Serbian = 165,
    /// Serbian (Cyrillic)
    Serbian_Cyrillic = 166,
    /// Serbian (Latin)
    Serbian_Latin = 167,
    /// SerboCroatian
    SerboCroatian = 168,
    /// Sesotho
    Sesotho = 169,
    /// Setswana
    Setswana = 170,
    /// Shona
    Shona = 171,
    /// Sindhi
    Sindhi = 172,
    /// Sinhalese
    Sinhalese = 173,
    /// Siswati
    Siswati = 174,
    /// Slovak
    Slovak = 175,
    /// Slovenian
    Slovenian = 176,
    /// Somali
    Somali = 177,
    /// Spanish
    Spanish = 178,
    /// Spanish (Argentina)
    Spanish_Argentina = 179,
    /// Spanish (Bolivia)
    Spanish_Bolivia = 180,
    /// Spanish (Chile)
    Spanish_Chile = 181,
    /// Spanish (Colombia)
    Spanish_Colombia = 182,
    /// Spanish (Costa Rica)
    Spanish_CostaRica = 183,
    /// Spanish (Dominican Republic)
    Spanish_DominicanRepublic = 184,
    /// Spanish (Ecuador)
    Spanish_Ecuador = 185,
    /// Spanish (El Salvador)
    Spanish_ElSalvador = 186,
    /// Spanish (Guatemala)
    Spanish_Guatemala = 187,
    /// Spanish (Honduras)
    Spanish_Honduras = 188,
    /// Spanish (Mexico)
    Spanish_Mexican = 189,
    /// Spanish (Modern)
    Spanish_Modern = 190,
    /// Spanish (Nicaragua)
    Spanish_Nicaragua = 191,
    /// Spanish (Panama)
    Spanish_Panama = 192,
    /// Spanish (Paraguay)
    Spanish_Paraguay = 193,
    /// Spanish (Peru)
    Spanish_Peru = 194,
    /// Spanish (Puerto Rico)
    Spanish_PuertoRico = 195,
    /// Spanish (Uruguay)
    Spanish_Uruguay = 196,
    /// Spanish (US)
    Spanish_Us = 197,
    /// Spanish (Venezuela)
    Spanish_Venezuela = 198,
    /// Sundanese
    Sundanese = 199,
    /// Swahili
    Swahili = 200,
    /// Swedish
    Swedish = 201,
    /// Swedish (Finland)
    Swedish_Finland = 202,
    /// Tagalog
    Tagalog = 203,
    /// Tajik
    Tajik = 204,
    /// Tamil
    Tamil = 205,
    /// Tatar
    Tatar = 206,
    /// Telugu
    Telugu = 207,
    /// Thai
    Thai = 208,
    /// Tibetan
    Tibetan = 209,
    /// Tigrinya
    Tigrinya = 210,
    /// Tonga
    Tonga = 211,
    /// Tsonga
    Tsonga = 212,
    /// Turkish
    Turkish = 213,
    /// Turkmen
    Turkmen = 214,
    /// Twi
    Twi = 215,
    /// Uighur
    Uighur = 216,
    /// Ukrainian
    Ukrainian = 217,
    /// Urdu
    Urdu = 218,
    /// Urdu (India)
    Urdu_India = 219,
    /// Urdu (Pakistan)
    Urdu_Pakistan = 220,
    /// Uzbek
    Uzbek = 221,
    /// Uzbek (Cyrillic)
    Uzbek_Cyrillic = 222,
    /// Uzbek (Latin)
    Uzbek_Latin = 223,
    /// Valencian
    Valencian = 224,
    /// Vietnamese
    Vietnamese = 225,
    /// Volapuk
    Volapuk = 226,
    /// Welsh
    Welsh = 227,
    /// Wolof
    Wolof = 228,
    /// Xhosa
    Xhosa = 229,
    /// Yiddish
    Yiddish = 230,
    /// Yoruba
    Yoruba = 231,
    /// Zhuang
    Zhuang = 232,
    /// Zulu
    Zulu = 233,
    /// User defined language (must be last)
    UserDefined = 234,
}

impl Language {
    /// Convert the language to its integer representation.
    #[inline]
    pub fn as_i32(self) -> i32 {
        self as i32
    }

    /// Try to create a Language from an integer value.
    ///
    /// Returns `None` if the value doesn't correspond to a valid language.
    pub fn from_i32(val: i32) -> Option<Self> {
        // For safety, we only convert known values
        if val >= 0 && val <= Language::UserDefined as i32 {
            // This is safe because we've verified the value is in range
            // and the enum is repr(i32)
            Some(unsafe { std::mem::transmute::<i32, Language>(val) })
        } else {
            None
        }
    }
}

impl From<Language> for i32 {
    fn from(lang: Language) -> i32 {
        lang.as_i32()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_values() {
        assert_eq!(Language::Default.as_i32(), 0);
        assert_eq!(Language::Unknown.as_i32(), 1);
        assert_eq!(Language::English.as_i32(), 58);
        assert_eq!(Language::French.as_i32(), 80);
        assert_eq!(Language::German.as_i32(), 89);
    }

    #[test]
    fn test_from_i32() {
        assert_eq!(Language::from_i32(0), Some(Language::Default));
        assert_eq!(Language::from_i32(58), Some(Language::English));
        assert_eq!(Language::from_i32(-1), None);
        assert_eq!(Language::from_i32(1000), None);
    }
}
