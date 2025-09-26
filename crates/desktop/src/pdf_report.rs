// SPDX-License-Identifier: GPL-3.0-or-later
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::Path;
use std::rc::Rc;
use std::cell::RefCell;

use api::{AnalysisAPI, StorageAPI, Disk};
use genpdfi::{elements as E, style as S, Mm, PaperSize, RenderResult, Size};
use genpdfi::fonts::{FontData, FontFamily as GenFontFamily};
use genpdfi::{Context, Document, Element, PageDecorator, Position, SimplePageDecorator};
use genpdfi::elements::IntoBoxedElement;
use md5::{Digest, Md5};
use sha2::Sha256;

// Add this to crates/desktop/src/pdf_report.rs

use unicode_segmentation::UnicodeSegmentation;

// A custom element that wraps long, unbreakable words at the character level.
#[derive(Clone, Debug)]
struct WordWrapParagraph {
    text: String,
    style: S::Style,
    render_idx: usize, // The starting byte index for the next render call
}

impl WordWrapParagraph {
    fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            style: S::Style::new(),
            render_idx: 0,
        }
    }

    fn styled(mut self, style: S::Style) -> Self {
        self.style = style;
        self
    }
}

impl Element for WordWrapParagraph {
    fn render(
        &mut self,
        context: &Context,
        mut area: genpdfi::render::Area<'_>,
        style: S::Style,
    ) -> Result<RenderResult, genpdfi::error::Error> {
        let mut result = RenderResult::default();
        let effective_style = style.and(self.style);
        let line_height = effective_style.line_height(&context.font_cache);

        // If there's no text left to render, we're done.
        if self.render_idx >= self.text.len() {
            return Ok(result);
        }

        let mut remaining_text = &self.text[self.render_idx..];

        while !remaining_text.is_empty() && area.size().height >= line_height {
            let mut current_line = String::new();
            let mut last_fit_idx = 0;

            // Use graphemes to handle Unicode correctly
            for (idx, grapheme) in remaining_text.grapheme_indices(true) {
                let mut prospective_line = current_line.clone();
                prospective_line.push_str(grapheme);

                if effective_style.str_width(&context.font_cache, &prospective_line) > area.size().width {
                    // The new grapheme doesn't fit, so the line is complete.
                    break;
                }

                // It fits, update the line and the index of the last character that fit.
                current_line = prospective_line;
                last_fit_idx = idx + grapheme.len();
            }

            // If not a single grapheme fits, the area is too narrow.
            // Avoid an infinite loop. We return what we've done and signal more content.
            if current_line.is_empty() && !remaining_text.is_empty() {
                result.has_more = true;
                return Ok(result);
            }

            // Render the line that fits.
            area.print_str(
                &context.font_cache,
                Position::new(0, 0),
                effective_style,
                &current_line,
            )?;

            // Update state for the next line/page
            result.size.height += line_height;
            result.size.width = result.size.width.max(effective_style.str_width(&context.font_cache, &current_line));
            area.add_offset(Position::new(0, line_height));
            self.render_idx += last_fit_idx;
            remaining_text = &self.text[self.render_idx..];
        }

        result.has_more = self.render_idx < self.text.len();
        Ok(result)
    }
}

// --- Hashing Logic ---
struct FileHashes {
    md5: String,
    sha256: String,
    blake3: String,
}

fn calculate_hashes(path: &Path) -> io::Result<FileHashes> {
    let input = File::open(path)?;
    let mut reader = BufReader::new(input);

    let mut md5_hasher = Md5::new();
    let mut sha256_hasher = Sha256::new();
    let mut blake3_hasher = blake3::Hasher::new();

    let mut buffer = [0; 8192];
    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        md5_hasher.update(&buffer[..count]);
        sha256_hasher.update(&buffer[..count]);
        blake3_hasher.update(&buffer[..count]);
    }

    Ok(FileHashes {
        md5: hex::encode(md5_hasher.finalize()),
        sha256: hex::encode(sha256_hasher.finalize()),
        blake3: blake3_hasher.finalize().to_hex().to_string(),
    })
}

// --- PDF Generation ---
pub fn generate_pdf_report(
    analysis_api: Rc<RefCell<AnalysisAPI>>,
    storage_api: Rc<RefCell<StorageAPI>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let save_path = rfd::FileDialog::new()
        .add_filter("PDF Document", &["pdf"])
        .set_file_name(format!(
            "Foruster_Report_{}.pdf",
            chrono::Local::now().format("%Y%m%d_%H%M%S")
        ))
        .save_file();
    let path = match save_path {
        Some(path) => path,
        None => return Ok(()),
    };
    let font_regular = include_bytes!("../ui/assets/fonts/Roboto-Regular.ttf");
    let font_bold = include_bytes!("../ui/assets/fonts/Roboto-Bold.ttf");
    let font_italic = include_bytes!("../ui/assets/fonts/Roboto-Italic.ttf");
    let font_bold_italic = include_bytes!("../ui/assets/fonts/Roboto-BoldItalic.ttf");
    let font_family = GenFontFamily {
        regular: FontData::new(font_regular.to_vec(), None)?,
        bold: FontData::new(font_bold.to_vec(), None)?,
        italic: FontData::new(font_italic.to_vec(), None)?,
        bold_italic: FontData::new(font_bold_italic.to_vec(), None)?,
    };
    let mut doc = Document::new(font_family);
    doc.set_title("Forensic Analysis Report");
    doc.set_paper_size(PaperSize::A4);
    let mut decorator = SimplePageDecorator::new();
    decorator.set_margins(15);
    decorator.set_header(|page| {
        let mut layout = E::LinearLayout::vertical();
        let mut title = E::Paragraph::new(format!("Forensic Analysis Report - Page {}", page));
        title.set_alignment(genpdfi::Alignment::Center);
        layout.push(title);
        layout.push(E::Break::new(1.0));
        layout
    });
    doc.set_page_decorator(decorator);

    let style_title = S::Style::new().bold().with_font_size(24);
    let style_h1 = S::Style::new().bold().with_font_size(18);
    let style_h2 = S::Style::new().bold().with_font_size(14);
    let style_bold = S::Style::new().bold();
    let _style_body = S::Style::new().with_font_size(10);
    let style_code = S::Style::new().with_font_size(9);

    doc.push(E::Paragraph::new("Forensic Analysis Report").styled(style_title));
    doc.push(E::Paragraph::new(format!("Report Generated: {}", chrono::Local::now().to_rfc2822())));
    doc.push(E::Break::new(2.0));

    let analysis_api = analysis_api.borrow();
    let storage_api = storage_api.borrow();
    let summary = analysis_api.get_analysis_summary()?;
    let profiles_stats = analysis_api.get_profile_statistics();

    doc.push(E::Paragraph::new("1. Executive Summary").styled(style_h1));
    let mut summary_table = E::TableLayout::new(vec![1, 2]);
    summary_table.row()
        .element(E::Paragraph::new("Total Files Scanned").styled(style_bold))
        .element(E::Paragraph::new(summary.total_files_scanned.to_string()))
        .push()?;
    summary_table.row()
        .element(E::Paragraph::new("Total Files Analyzed").styled(style_bold))
        .element(E::Paragraph::new(summary.total_files_analyzed.to_string()))
        .push()?;
    summary_table.row()
        .element(E::Paragraph::new("Matched Files Found").styled(style_bold))
        .element(E::Paragraph::new(summary.total_matches_found.to_string()))
        .push()?;
    summary_table.row()
        .element(E::Paragraph::new("Suspicious Files Found").styled(style_bold))
        .element(E::Paragraph::new(summary.total_suspicious_files.to_string()))
        .push()?;
    summary_table.row()
        .element(E::Paragraph::new("Analysis Duration").styled(style_bold))
        .element(E::Paragraph::new(format!("{:?}", summary.analysis_duration)))
        .push()?;
    doc.push(summary_table);
    doc.push(E::Break::new(1.5));

    doc.push(E::Paragraph::new("2. Scope of Analysis").styled(style_h1));
    doc.push(E::Paragraph::new("2.1. Analyzed Disks").styled(style_h2));
    let mut disks_table = E::TableLayout::new(vec![1, 1, 2, 1]);
    disks_table.set_cell_decorator(E::FrameCellDecorator::new(true, true, false));
    disks_table.row()
        .element(E::Paragraph::new("Model").styled(style_bold))
        .element(E::Paragraph::new("Vendor").styled(style_bold))
        .element(E::Paragraph::new("Serial Number").styled(style_bold))
        .element(E::Paragraph::new("Bus").styled(style_bold))
        .push()?;
    for disk in storage_api.get_disks() {
        disks_table.row()
            .element(E::Paragraph::new(disk.identification_data().model().as_deref().unwrap_or("N/A")))
            .element(E::Paragraph::new(disk.identification_data().vendor().as_deref().unwrap_or("N/A")))
            .element(WordWrapParagraph::new(disk.identification_data().serial_number().as_deref().unwrap_or("N/A")).styled(style_code))
            .element(E::Paragraph::new(disk.identification_data().bus_type().to_string()))
            .push()?;
    }
    doc.push(disks_table);
    doc.push(E::Break::new(1.5));

    doc.push(E::Paragraph::new("2.2. Analysis Profiles Used").styled(style_h2));
    let mut profiles_list = E::UnorderedList::new();
    for (profile_name, count) in profiles_stats.clone() {
        profiles_list.push(E::Paragraph::new(format!("{} ({} matches)", profile_name, count)));
    }
    doc.push(profiles_list);
    doc.push(E::Break::new(1.5));
    let suspicious_files = analysis_api.get_suspicious_files();
    if !suspicious_files.is_empty() {
        doc.push(E::Paragraph::new("3. Suspicious Findings").styled(style_h1));
        let mut suspicious_table = E::TableLayout::new(vec![3, 2, 1, 3]);
        suspicious_table.set_cell_decorator(E::FrameCellDecorator::new(true, true, false));
        suspicious_table.row()
            .element(E::Paragraph::new("File Path").styled(style_bold))
            .element(E::Paragraph::new("Reason").styled(style_bold))
            .element(E::Paragraph::new("Size").styled(style_bold))
            .element(E::Paragraph::new("Hashes (MD5, SHA256)").styled(style_bold))
            .push()?;
        for (path, reason) in suspicious_files {
            let size_str = std::fs::metadata(&path).map(|m| app_core::format_size(m.len() as usize)).unwrap_or_else(|_| "N/A".into());
            let hashes = calculate_hashes(&path);
            let hashes_element = match hashes {
                Ok(h) => {
                    let mut layout = E::LinearLayout::vertical(); // CHANGED: Use a layout for multiple lines
                    layout.push(WordWrapParagraph::new(format!("MD5: {}", h.md5)).styled(style_code));
                    layout.push(WordWrapParagraph::new(format!("SHA256: {}", h.sha256)).styled(style_code));
                    layout.into_boxed_element()
                }
                Err(_) => E::Paragraph::new("Error calculating hashes").into_boxed_element()
            };
            suspicious_table.row()
                .element(WordWrapParagraph::new(path.to_string_lossy()).styled(style_code)) // CHANGED
                .element(WordWrapParagraph::new(reason.to_string())) // CHANGED
                .element(E::Paragraph::new(size_str))
                .element(hashes_element) // CHANGED
                .push()?;
        }
        doc.push(suspicious_table);
        doc.push(E::Break::new(1.5));
    }

    doc.push(E::Paragraph::new("4. Detailed Findings by Profile").styled(style_h1));
    for profile_name in analysis_api.get_profile_statistics().keys() {
        let files = analysis_api.get_files_by_profile(profile_name);
        if files.is_empty() { continue; }

        // Use a simple counter for section numbering to avoid complex logic.
        doc.push(E::Paragraph::new(format!("Profile: {}", profile_name)).styled(style_h2));

        let mut files_table = E::TableLayout::new(vec![4, 1, 5]);
        files_table.set_cell_decorator(E::FrameCellDecorator::new(true, true, false));
        files_table.row()
            .element(E::Paragraph::new("File Path").styled(style_bold))
            .element(E::Paragraph::new("Size").styled(style_bold))
            .element(E::Paragraph::new("Hashes (MD5, SHA256, BLAKE3)").styled(style_bold))
            .push()?;

        for path in files.iter().take(200) { // Limit results per profile to keep PDF size manageable
            let size_str = std::fs::metadata(path).map(|m| app_core::format_size(m.len() as usize)).unwrap_or_else(|_| "N/A".into());
            let hashes = calculate_hashes(path);

            // CHANGED: Use a LinearLayout for multi-line hash display and WordWrapParagraph for each hash
            let hashes_element = match hashes {
                Ok(h) => {
                    let mut layout = E::LinearLayout::vertical();
                    layout.push(WordWrapParagraph::new(format!("MD5: {}", h.md5)).styled(style_code));
                    layout.push(WordWrapParagraph::new(format!("SHA256: {}", h.sha256)).styled(style_code));
                    layout.push(WordWrapParagraph::new(format!("BLAKE3: {}", h.blake3)).styled(style_code));
                    layout.into_boxed_element()
                }
                Err(_) => E::Paragraph::new("Error calculating hashes").into_boxed_element()
            };

            files_table.row()
                .element(WordWrapParagraph::new(path.to_string_lossy()).styled(style_code)) // CHANGED
                .element(E::Paragraph::new(size_str))
                .element(hashes_element) // CHANGED
                .push()?;
        }
        doc.push(files_table);
        doc.push(E::Break::new(1.0));
    }

    doc.render_to_file(path)?;
    println!("PDF report generated successfully."); // CHANGED: Moved success message here
    Ok(())
}