//! Color definitions for wxDragon.
//!
//! This module provides the Colour struct for representing RGBA colors,
//! along with common color constants and conversion utilities.

use wxdragon_sys as ffi;

/// Represents an RGBA color.
///
/// A color with red, green, blue, and alpha components.
/// The alpha value ranges from 0 (transparent) to 255 (opaque).
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Colour {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8, // Alpha component (0-255, 255 is opaque)
}

impl Colour {
    /// Creates a new color with the given RGBA values.
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Colour { r, g, b, a }
    }

    /// Creates a new color with full opacity.
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Colour { r, g, b, a: 255 }
    }

    /// Create from a u32 in RGBA order (e.g., 0xRRGGBBAA)
    pub fn from_u32(val: u32) -> Self {
        Colour {
            r: ((val >> 24) & 0xFF) as u8,
            g: ((val >> 16) & 0xFF) as u8,
            b: ((val >> 8) & 0xFF) as u8,
            a: (val & 0xFF) as u8,
        }
    }

    /// Convert to u32 in RGBA order
    pub fn as_u32(&self) -> u32 {
        ((self.r as u32) << 24) | ((self.g as u32) << 16) | ((self.b as u32) << 8) | (self.a as u32)
    }

    /// Convert to wxd_Colour_t for use with the C API
    pub fn to_raw(&self) -> ffi::wxd_Colour_t {
        ffi::wxd_Colour_t {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }

    /// Returns a darker version of this color
    pub fn darker(&self, factor: f32) -> Self {
        let factor = factor.clamp(0.0, 1.0);
        Colour {
            r: (self.r as f32 * factor) as u8,
            g: (self.g as f32 * factor) as u8,
            b: (self.b as f32 * factor) as u8,
            a: self.a,
        }
    }

    /// Returns a lighter version of this color
    pub fn lighter(&self, factor: f32) -> Self {
        let factor = factor.clamp(0.0, 1.0);
        Colour {
            r: (self.r as f32 + (255.0 - self.r as f32) * factor) as u8,
            g: (self.g as f32 + (255.0 - self.g as f32) * factor) as u8,
            b: (self.b as f32 + (255.0 - self.b as f32) * factor) as u8,
            a: self.a,
        }
    }

    /// Creates a darker version of this color by reducing brightness.
    pub fn darken(&self, factor: f64) -> Self {
        let factor = factor.clamp(0.0, 1.0);
        Self {
            r: ((self.r as f64 * (1.0 - factor)) as u8),
            g: ((self.g as f64 * (1.0 - factor)) as u8),
            b: ((self.b as f64 * (1.0 - factor)) as u8),
            a: self.a,
        }
    }
}

impl From<Colour> for ffi::wxd_Colour_t {
    fn from(c: Colour) -> Self {
        ffi::wxd_Colour_t {
            r: c.r,
            g: c.g,
            b: c.b,
            a: c.a,
        }
    }
}

impl From<ffi::wxd_Colour_t> for Colour {
    fn from(c: ffi::wxd_Colour_t) -> Self {
        Colour {
            r: c.r,
            g: c.g,
            b: c.b,
            a: c.a,
        }
    }
}

/// Common color constants
pub mod colours {
    use super::Colour;

    pub const BLACK: Colour = Colour {
        r: 0,
        g: 0,
        b: 0,
        a: 255,
    };

    pub const WHITE: Colour = Colour {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };

    pub const RED: Colour = Colour {
        r: 255,
        g: 0,
        b: 0,
        a: 255,
    };

    pub const GREEN: Colour = Colour {
        r: 0,
        g: 255,
        b: 0,
        a: 255,
    };

    pub const BLUE: Colour = Colour {
        r: 0,
        g: 0,
        b: 255,
        a: 255,
    };

    pub const YELLOW: Colour = Colour {
        r: 255,
        g: 255,
        b: 0,
        a: 255,
    };

    pub const CYAN: Colour = Colour {
        r: 0,
        g: 255,
        b: 255,
        a: 255,
    };

    pub const MAGENTA: Colour = Colour {
        r: 255,
        g: 0,
        b: 255,
        a: 255,
    };

    pub const TRANSPARENT: Colour = Colour { r: 0, g: 0, b: 0, a: 0 };

    pub const GRAY: Colour = Colour {
        r: 128,
        g: 128,
        b: 128,
        a: 255,
    };

    pub const LIGHT_GRAY: Colour = Colour {
        r: 192,
        g: 192,
        b: 192,
        a: 255,
    };

    pub const DARK_GRAY: Colour = Colour {
        r: 64,
        g: 64,
        b: 64,
        a: 255,
    };

    // Tailwind CSS Color Palette

    // Gray
    pub mod gray {
        use super::Colour;

        pub const GRAY_50: Colour = Colour::rgb(249, 250, 251);
        pub const GRAY_100: Colour = Colour::rgb(243, 244, 246);
        pub const GRAY_200: Colour = Colour::rgb(229, 231, 235);
        pub const GRAY_300: Colour = Colour::rgb(209, 213, 219);
        pub const GRAY_400: Colour = Colour::rgb(156, 163, 175);
        pub const GRAY_500: Colour = Colour::rgb(107, 114, 128);
        pub const GRAY_600: Colour = Colour::rgb(75, 85, 99);
        pub const GRAY_700: Colour = Colour::rgb(55, 65, 81);
        pub const GRAY_800: Colour = Colour::rgb(31, 41, 55);
        pub const GRAY_900: Colour = Colour::rgb(17, 24, 39);
        pub const GRAY_950: Colour = Colour::rgb(3, 7, 18);
    }

    // Red
    pub mod red {
        use super::Colour;

        pub const RED_50: Colour = Colour::rgb(254, 242, 242);
        pub const RED_100: Colour = Colour::rgb(254, 226, 226);
        pub const RED_200: Colour = Colour::rgb(254, 202, 202);
        pub const RED_300: Colour = Colour::rgb(252, 165, 165);
        pub const RED_400: Colour = Colour::rgb(248, 113, 113);
        pub const RED_500: Colour = Colour::rgb(239, 68, 68);
        pub const RED_600: Colour = Colour::rgb(220, 38, 38);
        pub const RED_700: Colour = Colour::rgb(185, 28, 28);
        pub const RED_800: Colour = Colour::rgb(153, 27, 27);
        pub const RED_900: Colour = Colour::rgb(127, 29, 29);
        pub const RED_950: Colour = Colour::rgb(69, 10, 10);
    }

    // Orange
    pub mod orange {
        use super::Colour;

        pub const ORANGE_50: Colour = Colour::rgb(255, 247, 237);
        pub const ORANGE_100: Colour = Colour::rgb(255, 237, 213);
        pub const ORANGE_200: Colour = Colour::rgb(254, 215, 170);
        pub const ORANGE_300: Colour = Colour::rgb(253, 186, 116);
        pub const ORANGE_400: Colour = Colour::rgb(251, 146, 60);
        pub const ORANGE_500: Colour = Colour::rgb(249, 115, 22);
        pub const ORANGE_600: Colour = Colour::rgb(234, 88, 12);
        pub const ORANGE_700: Colour = Colour::rgb(194, 65, 12);
        pub const ORANGE_800: Colour = Colour::rgb(154, 52, 18);
        pub const ORANGE_900: Colour = Colour::rgb(124, 45, 18);
        pub const ORANGE_950: Colour = Colour::rgb(67, 20, 7);
    }

    // Amber
    pub mod amber {
        use super::Colour;

        pub const AMBER_50: Colour = Colour::rgb(255, 251, 235);
        pub const AMBER_100: Colour = Colour::rgb(254, 243, 199);
        pub const AMBER_200: Colour = Colour::rgb(253, 230, 138);
        pub const AMBER_300: Colour = Colour::rgb(252, 211, 77);
        pub const AMBER_400: Colour = Colour::rgb(251, 191, 36);
        pub const AMBER_500: Colour = Colour::rgb(245, 158, 11);
        pub const AMBER_600: Colour = Colour::rgb(217, 119, 6);
        pub const AMBER_700: Colour = Colour::rgb(180, 83, 9);
        pub const AMBER_800: Colour = Colour::rgb(146, 64, 14);
        pub const AMBER_900: Colour = Colour::rgb(120, 53, 15);
        pub const AMBER_950: Colour = Colour::rgb(69, 26, 3);
    }

    // Yellow
    pub mod yellow {
        use super::Colour;

        pub const YELLOW_50: Colour = Colour::rgb(254, 252, 232);
        pub const YELLOW_100: Colour = Colour::rgb(254, 249, 195);
        pub const YELLOW_200: Colour = Colour::rgb(254, 240, 138);
        pub const YELLOW_300: Colour = Colour::rgb(253, 224, 71);
        pub const YELLOW_400: Colour = Colour::rgb(250, 204, 21);
        pub const YELLOW_500: Colour = Colour::rgb(234, 179, 8);
        pub const YELLOW_600: Colour = Colour::rgb(202, 138, 4);
        pub const YELLOW_700: Colour = Colour::rgb(161, 98, 7);
        pub const YELLOW_800: Colour = Colour::rgb(133, 77, 14);
        pub const YELLOW_900: Colour = Colour::rgb(113, 63, 18);
        pub const YELLOW_950: Colour = Colour::rgb(66, 32, 6);
    }

    // Lime
    pub mod lime {
        use super::Colour;

        pub const LIME_50: Colour = Colour::rgb(247, 254, 231);
        pub const LIME_100: Colour = Colour::rgb(236, 252, 203);
        pub const LIME_200: Colour = Colour::rgb(217, 249, 157);
        pub const LIME_300: Colour = Colour::rgb(190, 242, 100);
        pub const LIME_400: Colour = Colour::rgb(163, 230, 53);
        pub const LIME_500: Colour = Colour::rgb(132, 204, 22);
        pub const LIME_600: Colour = Colour::rgb(101, 163, 13);
        pub const LIME_700: Colour = Colour::rgb(77, 124, 15);
        pub const LIME_800: Colour = Colour::rgb(63, 98, 18);
        pub const LIME_900: Colour = Colour::rgb(54, 83, 20);
        pub const LIME_950: Colour = Colour::rgb(26, 46, 5);
    }

    // Green
    pub mod green {
        use super::Colour;

        pub const GREEN_50: Colour = Colour::rgb(240, 253, 244);
        pub const GREEN_100: Colour = Colour::rgb(220, 252, 231);
        pub const GREEN_200: Colour = Colour::rgb(187, 247, 208);
        pub const GREEN_300: Colour = Colour::rgb(134, 239, 172);
        pub const GREEN_400: Colour = Colour::rgb(74, 222, 128);
        pub const GREEN_500: Colour = Colour::rgb(34, 197, 94);
        pub const GREEN_600: Colour = Colour::rgb(22, 163, 74);
        pub const GREEN_700: Colour = Colour::rgb(21, 128, 61);
        pub const GREEN_800: Colour = Colour::rgb(22, 101, 52);
        pub const GREEN_900: Colour = Colour::rgb(20, 83, 45);
        pub const GREEN_950: Colour = Colour::rgb(5, 46, 22);
    }

    // Emerald
    pub mod emerald {
        use super::Colour;

        pub const EMERALD_50: Colour = Colour::rgb(236, 253, 245);
        pub const EMERALD_100: Colour = Colour::rgb(209, 250, 229);
        pub const EMERALD_200: Colour = Colour::rgb(167, 243, 208);
        pub const EMERALD_300: Colour = Colour::rgb(110, 231, 183);
        pub const EMERALD_400: Colour = Colour::rgb(52, 211, 153);
        pub const EMERALD_500: Colour = Colour::rgb(16, 185, 129);
        pub const EMERALD_600: Colour = Colour::rgb(5, 150, 105);
        pub const EMERALD_700: Colour = Colour::rgb(4, 120, 87);
        pub const EMERALD_800: Colour = Colour::rgb(6, 95, 70);
        pub const EMERALD_900: Colour = Colour::rgb(6, 78, 59);
        pub const EMERALD_950: Colour = Colour::rgb(2, 44, 34);
    }

    // Teal
    pub mod teal {
        use super::Colour;

        pub const TEAL_50: Colour = Colour::rgb(240, 253, 250);
        pub const TEAL_100: Colour = Colour::rgb(204, 251, 241);
        pub const TEAL_200: Colour = Colour::rgb(153, 246, 228);
        pub const TEAL_300: Colour = Colour::rgb(94, 234, 212);
        pub const TEAL_400: Colour = Colour::rgb(45, 212, 191);
        pub const TEAL_500: Colour = Colour::rgb(20, 184, 166);
        pub const TEAL_600: Colour = Colour::rgb(13, 148, 136);
        pub const TEAL_700: Colour = Colour::rgb(15, 118, 110);
        pub const TEAL_800: Colour = Colour::rgb(17, 94, 89);
        pub const TEAL_900: Colour = Colour::rgb(19, 78, 74);
        pub const TEAL_950: Colour = Colour::rgb(4, 47, 46);
    }

    // Cyan
    pub mod cyan {
        use super::Colour;

        pub const CYAN_50: Colour = Colour::rgb(236, 254, 255);
        pub const CYAN_100: Colour = Colour::rgb(207, 250, 254);
        pub const CYAN_200: Colour = Colour::rgb(165, 243, 252);
        pub const CYAN_300: Colour = Colour::rgb(103, 232, 249);
        pub const CYAN_400: Colour = Colour::rgb(34, 211, 238);
        pub const CYAN_500: Colour = Colour::rgb(6, 182, 212);
        pub const CYAN_600: Colour = Colour::rgb(8, 145, 178);
        pub const CYAN_700: Colour = Colour::rgb(14, 116, 144);
        pub const CYAN_800: Colour = Colour::rgb(21, 94, 117);
        pub const CYAN_900: Colour = Colour::rgb(22, 78, 99);
        pub const CYAN_950: Colour = Colour::rgb(8, 51, 68);
    }

    // Sky
    pub mod sky {
        use super::Colour;

        pub const SKY_50: Colour = Colour::rgb(240, 249, 255);
        pub const SKY_100: Colour = Colour::rgb(224, 242, 254);
        pub const SKY_200: Colour = Colour::rgb(186, 230, 253);
        pub const SKY_300: Colour = Colour::rgb(125, 211, 252);
        pub const SKY_400: Colour = Colour::rgb(56, 189, 248);
        pub const SKY_500: Colour = Colour::rgb(14, 165, 233);
        pub const SKY_600: Colour = Colour::rgb(2, 132, 199);
        pub const SKY_700: Colour = Colour::rgb(3, 105, 161);
        pub const SKY_800: Colour = Colour::rgb(7, 89, 133);
        pub const SKY_900: Colour = Colour::rgb(12, 74, 110);
        pub const SKY_950: Colour = Colour::rgb(8, 47, 73);
    }

    // Blue
    pub mod blue {
        use super::Colour;

        pub const BLUE_50: Colour = Colour::rgb(239, 246, 255);
        pub const BLUE_100: Colour = Colour::rgb(219, 234, 254);
        pub const BLUE_200: Colour = Colour::rgb(191, 219, 254);
        pub const BLUE_300: Colour = Colour::rgb(147, 197, 253);
        pub const BLUE_400: Colour = Colour::rgb(96, 165, 250);
        pub const BLUE_500: Colour = Colour::rgb(59, 130, 246);
        pub const BLUE_600: Colour = Colour::rgb(37, 99, 235);
        pub const BLUE_700: Colour = Colour::rgb(29, 78, 216);
        pub const BLUE_800: Colour = Colour::rgb(30, 64, 175);
        pub const BLUE_900: Colour = Colour::rgb(30, 58, 138);
        pub const BLUE_950: Colour = Colour::rgb(23, 37, 84);
    }

    // Indigo
    pub mod indigo {
        use super::Colour;

        pub const INDIGO_50: Colour = Colour::rgb(238, 242, 255);
        pub const INDIGO_100: Colour = Colour::rgb(224, 231, 255);
        pub const INDIGO_200: Colour = Colour::rgb(199, 210, 254);
        pub const INDIGO_300: Colour = Colour::rgb(165, 180, 252);
        pub const INDIGO_400: Colour = Colour::rgb(129, 140, 248);
        pub const INDIGO_500: Colour = Colour::rgb(99, 102, 241);
        pub const INDIGO_600: Colour = Colour::rgb(79, 70, 229);
        pub const INDIGO_700: Colour = Colour::rgb(67, 56, 202);
        pub const INDIGO_800: Colour = Colour::rgb(55, 48, 163);
        pub const INDIGO_900: Colour = Colour::rgb(49, 46, 129);
        pub const INDIGO_950: Colour = Colour::rgb(30, 27, 75);
    }

    // Violet
    pub mod violet {
        use super::Colour;

        pub const VIOLET_50: Colour = Colour::rgb(245, 243, 255);
        pub const VIOLET_100: Colour = Colour::rgb(237, 233, 254);
        pub const VIOLET_200: Colour = Colour::rgb(221, 214, 254);
        pub const VIOLET_300: Colour = Colour::rgb(196, 181, 253);
        pub const VIOLET_400: Colour = Colour::rgb(167, 139, 250);
        pub const VIOLET_500: Colour = Colour::rgb(139, 92, 246);
        pub const VIOLET_600: Colour = Colour::rgb(124, 58, 237);
        pub const VIOLET_700: Colour = Colour::rgb(109, 40, 217);
        pub const VIOLET_800: Colour = Colour::rgb(91, 33, 182);
        pub const VIOLET_900: Colour = Colour::rgb(76, 29, 149);
        pub const VIOLET_950: Colour = Colour::rgb(46, 16, 101);
    }

    // Purple
    pub mod purple {
        use super::Colour;

        pub const PURPLE_50: Colour = Colour::rgb(250, 245, 255);
        pub const PURPLE_100: Colour = Colour::rgb(243, 232, 255);
        pub const PURPLE_200: Colour = Colour::rgb(233, 213, 255);
        pub const PURPLE_300: Colour = Colour::rgb(216, 180, 254);
        pub const PURPLE_400: Colour = Colour::rgb(192, 132, 252);
        pub const PURPLE_500: Colour = Colour::rgb(168, 85, 247);
        pub const PURPLE_600: Colour = Colour::rgb(147, 51, 234);
        pub const PURPLE_700: Colour = Colour::rgb(126, 34, 206);
        pub const PURPLE_800: Colour = Colour::rgb(107, 33, 168);
        pub const PURPLE_900: Colour = Colour::rgb(88, 28, 135);
        pub const PURPLE_950: Colour = Colour::rgb(59, 7, 100);
    }

    // Fuchsia
    pub mod fuchsia {
        use super::Colour;

        pub const FUCHSIA_50: Colour = Colour::rgb(253, 244, 255);
        pub const FUCHSIA_100: Colour = Colour::rgb(250, 232, 255);
        pub const FUCHSIA_200: Colour = Colour::rgb(245, 208, 254);
        pub const FUCHSIA_300: Colour = Colour::rgb(240, 171, 252);
        pub const FUCHSIA_400: Colour = Colour::rgb(232, 121, 249);
        pub const FUCHSIA_500: Colour = Colour::rgb(217, 70, 239);
        pub const FUCHSIA_600: Colour = Colour::rgb(192, 38, 211);
        pub const FUCHSIA_700: Colour = Colour::rgb(162, 28, 175);
        pub const FUCHSIA_800: Colour = Colour::rgb(134, 25, 143);
        pub const FUCHSIA_900: Colour = Colour::rgb(112, 26, 117);
        pub const FUCHSIA_950: Colour = Colour::rgb(74, 4, 78);
    }

    // Pink
    pub mod pink {
        use super::Colour;

        pub const PINK_50: Colour = Colour::rgb(253, 242, 248);
        pub const PINK_100: Colour = Colour::rgb(252, 231, 243);
        pub const PINK_200: Colour = Colour::rgb(251, 207, 232);
        pub const PINK_300: Colour = Colour::rgb(249, 168, 212);
        pub const PINK_400: Colour = Colour::rgb(244, 114, 182);
        pub const PINK_500: Colour = Colour::rgb(236, 72, 153);
        pub const PINK_600: Colour = Colour::rgb(219, 39, 119);
        pub const PINK_700: Colour = Colour::rgb(190, 24, 93);
        pub const PINK_800: Colour = Colour::rgb(157, 23, 77);
        pub const PINK_900: Colour = Colour::rgb(131, 24, 67);
        pub const PINK_950: Colour = Colour::rgb(80, 7, 36);
    }

    // Rose
    pub mod rose {
        use super::Colour;

        pub const ROSE_50: Colour = Colour::rgb(255, 241, 242);
        pub const ROSE_100: Colour = Colour::rgb(255, 228, 230);
        pub const ROSE_200: Colour = Colour::rgb(254, 205, 211);
        pub const ROSE_300: Colour = Colour::rgb(253, 164, 175);
        pub const ROSE_400: Colour = Colour::rgb(251, 113, 133);
        pub const ROSE_500: Colour = Colour::rgb(244, 63, 94);
        pub const ROSE_600: Colour = Colour::rgb(225, 29, 72);
        pub const ROSE_700: Colour = Colour::rgb(190, 18, 60);
        pub const ROSE_800: Colour = Colour::rgb(159, 18, 57);
        pub const ROSE_900: Colour = Colour::rgb(136, 19, 55);
        pub const ROSE_950: Colour = Colour::rgb(76, 5, 25);
    }

    // Aliases for the 500 variants as primary colors
    pub const SLATE: Colour = gray::GRAY_500;
    pub const ZINC: Colour = gray::GRAY_500;
    pub const STONE: Colour = gray::GRAY_500;
    pub const AMBER: Colour = amber::AMBER_500;
    pub const LIME: Colour = lime::LIME_500;
    pub const EMERALD: Colour = emerald::EMERALD_500;
    pub const TEAL: Colour = teal::TEAL_500;
    pub const SKY: Colour = sky::SKY_500;
    pub const INDIGO: Colour = indigo::INDIGO_500;
    pub const VIOLET: Colour = violet::VIOLET_500;
    pub const PURPLE: Colour = purple::PURPLE_500;
    pub const FUCHSIA: Colour = fuchsia::FUCHSIA_500;
    pub const PINK: Colour = pink::PINK_500;
    pub const ROSE: Colour = rose::ROSE_500;
}
