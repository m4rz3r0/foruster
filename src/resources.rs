// SPDX-License-Identifier: GPL-3.0-or-later
use charts_rs::Color;
use manganis::{asset, Asset};

// Consts
pub const SUSPICIOUS_FILES_ID: usize = 1337;
pub const SVG_CHART_SIZE: u32 = 1000;
pub const THUMBNAIL_SIZE: u32 = 350;

// CSS
pub const TAILWIND_CSS_URL: Asset = asset!("/assets/tailwind.css");

// Default Profiles
pub const DEFAULT_PROFILES: &str = r#"
[
    {
        "type": "image",
        "name": "Imágenes"
    },
    {
        "type": "video",
        "name": "Vídeos"
    },
    {
        "type": "audio",
        "name": "Audio"
    },
    {
        "type": "archive",
        "name": "Archivos"
    },
    {
        "type": "book",
        "name": "Libros"
    },
    {
        "type": "document",
        "name": "Documentos"
    },
    {
        "type": "application",
        "name": "Aplicaciones"
    },
    {
        "type": "custom",
        "name": "Solo PDFs",
        "extensions": ["pdf"]
    }
]"#;

// Report Template
pub const REPORT_TEMPLATE_HTML_URL: Asset = asset!("/assets/report_template.html");
pub const REPORT_TEMPLATE_CSS_URL: Asset = asset!("/assets/report_template.css");

// UI Images
pub const PENDRIVE_USB_IMAGE: Asset = asset!("/assets/img/pendrive-usb.svg");
pub const HARD_DRIVE_IMAGE: Asset = asset!("/assets/img/hard-drive.svg");

// Colors
pub const IMAGE_PROFILE_COLOR: Color = Color { r: 59, g: 130, b: 246, a: 255 };
pub const VIDEO_PROFILE_COLOR: Color = Color { r: 0, g: 0, b: 0, a: 255 };
pub const AUDIO_PROFILE_COLOR: Color = Color { r: 34, g: 197, b: 94, a: 255 };
pub const ARCHIVE_PROFILE_COLOR: Color = Color { r: 113, g: 63, b: 18, a: 255 };
pub const BOOK_PROFILE_COLOR: Color = Color { r: 168, g: 85, b: 247, a: 255 };
pub const DOCUMENT_PROFILE_COLOR: Color = Color { r: 234, g: 179, b: 8, a: 255 };
pub const APLLICATION_PROFILE_COLOR: Color = Color { r: 6, g: 182, b: 212, a: 255 };
pub const CUSTOM_PROFILE_COLOR: Color = Color { r: 236, g: 72, b: 153, a: 255 };
pub const SUSPICIOUS_FILES_COLOR: Color = Color { r: 239, g: 68, b: 68, a: 255 };