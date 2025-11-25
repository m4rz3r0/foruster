// SPDX-License-Identifier: GPL-3.0-or-later

use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufReader, Read};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use api::{AnalysisAPI, AnalysisSummary, StorageAPI};
use genpdfi::{
    elements as E,
    elements::IntoBoxedElement,
    fonts::{FontData, FontFamily as GenFontFamily},
    style as S, Context, Document, Element, PaperSize, Position, RenderResult, SimplePageDecorator,
};
use md5::{Digest as Md5Digest, Md5};
use sha2::{Digest as Sha256Digest, Sha256};
use unicode_segmentation::UnicodeSegmentation;

// ============================================================================
//  Section 1: Custom PDF Elements
// ============================================================================

/// A custom element that wraps long, unbreakable words (like Hashes or Paths)
/// at the character level to prevent table overflows.
#[derive(Clone, Debug)]
struct WordWrapParagraph {
    text: String,
    style: S::Style,
    render_idx: usize,
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

        if self.render_idx >= self.text.len() {
            return Ok(result);
        }

        let remaining_text = &self.text[self.render_idx..];
        let mut current_idx = 0;

        // We can print lines as long as we have vertical space
        while current_idx < remaining_text.len() && area.size().height >= line_height {
            let mut current_line = String::new();
            let mut last_fit_len = 0;

            // Greedily consume graphemes until width is exceeded
            for (idx, grapheme) in remaining_text[current_idx..].grapheme_indices(true) {
                let mut prospective = current_line.clone();
                prospective.push_str(grapheme);

                if effective_style.str_width(&context.font_cache, &prospective) > area.size().width
                {
                    break;
                }

                current_line = prospective;
                last_fit_len = idx + grapheme.len();
            }

            // Force at least one char if column is super narrow to avoid infinite loops
            if current_line.is_empty() && !remaining_text[current_idx..].is_empty() {
                let mut graphemes = remaining_text[current_idx..].graphemes(true);
                if let Some(g) = graphemes.next() {
                    current_line = g.to_string();
                    last_fit_len = g.len();
                }
            }

            // Render line
            area.print_str(
                &context.font_cache,
                Position::new(0, 0),
                effective_style,
                &current_line,
            )?;

            // Advance
            result.size.height += line_height;
            result.size.width = result
                .size
                .width
                .max(effective_style.str_width(&context.font_cache, &current_line));
            area.add_offset(Position::new(0, line_height));

            current_idx += last_fit_len;
        }

        self.render_idx += current_idx;
        result.has_more = self.render_idx < self.text.len();
        Ok(result)
    }
}

// ============================================================================
//  Section 2: Helpers & Data Structures
// ============================================================================

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
        md5_hasher.input(&buffer[..count]);
        sha256_hasher.update(&buffer[..count]);
        blake3_hasher.update(&buffer[..count]);
    }

    Ok(FileHashes {
        md5: hex::encode(md5_hasher.result()),
        sha256: hex::encode(sha256_hasher.finalize()),
        blake3: blake3_hasher.finalize().to_hex().to_string(),
    })
}

// ============================================================================
//  Section 3: The Report Generator
// ============================================================================

struct ReportStyles {
    title: S::Style,
    header_1: S::Style,
    header_2: S::Style,
    bold: S::Style,
    code: S::Style,
}

impl Default for ReportStyles {
    fn default() -> Self {
        Self {
            title: S::Style::new().bold().with_font_size(24),
            header_1: S::Style::new().bold().with_font_size(18),
            header_2: S::Style::new().bold().with_font_size(14),
            bold: S::Style::new().bold(),
            code: S::Style::new().with_font_size(9),
        }
    }
}

pub struct ForensicReport {
    doc: Document,
    styles: ReportStyles,
}

impl ForensicReport {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        // Load embedded fonts
        let font_regular = include_bytes!("../fonts/Roboto-Regular.ttf");
        let font_bold = include_bytes!("../fonts/Roboto-Bold.ttf");
        let font_italic = include_bytes!("../fonts/Roboto-Italic.ttf");
        let font_bold_italic = include_bytes!("../fonts/Roboto-BoldItalic.ttf");

        let font_family = GenFontFamily {
            regular: FontData::new(font_regular.to_vec(), None)?,
            bold: FontData::new(font_bold.to_vec(), None)?,
            italic: FontData::new(font_italic.to_vec(), None)?,
            bold_italic: FontData::new(font_bold_italic.to_vec(), None)?,
        };

        let mut doc = Document::new(font_family);
        doc.set_title("Forensic Analysis Report");
        doc.set_paper_size(PaperSize::A4);

        // Configure Header
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

        Ok(Self {
            doc,
            styles: ReportStyles::default(),
        })
    }

    pub fn render_title(&mut self) {
        self.doc
            .push(E::Paragraph::new("Forensic Analysis Report").styled(self.styles.title));
        self.doc.push(E::Paragraph::new(format!(
            "Report Generated: {}",
            chrono::Local::now().to_rfc2822()
        )));
        self.doc.push(E::Break::new(2.0));
    }

    pub fn render_summary(&mut self, summary: &AnalysisSummary) {
        self.doc
            .push(E::Paragraph::new("1. Executive Summary").styled(self.styles.header_1));

        let mut table = E::TableLayout::new(vec![1, 2]);
        let rows = vec![
            (
                "Total Files Scanned",
                summary.total_files_scanned.to_string(),
            ),
            (
                "Total Files Analyzed",
                summary.total_files_analyzed.to_string(),
            ),
            (
                "Matched Files Found",
                summary.total_matches_found.to_string(),
            ),
            (
                "Suspicious Files Found",
                summary.total_suspicious_files.to_string(),
            ),
            (
                "Analysis Duration",
                format!("{:?}", summary.analysis_duration),
            ),
        ];

        for (label, value) in rows {
            table
                .row()
                .element(E::Paragraph::new(label).styled(self.styles.bold))
                .element(E::Paragraph::new(value))
                .push()
                .expect("Failed to push summary row");
        }

        self.doc.push(table);
        self.doc.push(E::Break::new(1.5));
    }

    pub fn render_disks(&mut self, disks: &[api::Disk]) {
        self.doc
            .push(E::Paragraph::new("2. Scope of Analysis").styled(self.styles.header_1));
        self.doc
            .push(E::Paragraph::new("2.1. Analyzed Disks").styled(self.styles.header_2));

        let mut table = E::TableLayout::new(vec![1, 1, 2, 1]);
        table.set_cell_decorator(E::FrameCellDecorator::new(true, true, false));

        // Header
        table
            .row()
            .element(E::Paragraph::new("Model").styled(self.styles.bold))
            .element(E::Paragraph::new("Vendor").styled(self.styles.bold))
            .element(E::Paragraph::new("Serial Number").styled(self.styles.bold))
            .element(E::Paragraph::new("Bus").styled(self.styles.bold))
            .push()
            .expect("Failed to push disk header");

        // Rows
        for disk in disks {
            let ident = disk.identification_data();
            table
                .row()
                .element(E::Paragraph::new(ident.model().as_deref().unwrap_or("N/A")))
                .element(E::Paragraph::new(
                    ident.vendor().as_deref().unwrap_or("N/A"),
                ))
                .element(
                    WordWrapParagraph::new(ident.serial_number().as_deref().unwrap_or("N/A"))
                        .styled(self.styles.code),
                )
                .element(E::Paragraph::new(ident.bus_type().to_string()))
                .push()
                .expect("Failed to push disk row");
        }

        self.doc.push(table);
        self.doc.push(E::Break::new(1.5));
    }

    pub fn render_profiles_used(&mut self, stats: &HashMap<String, usize>) {
        self.doc
            .push(E::Paragraph::new("2.2. Analysis Profiles Used").styled(self.styles.header_2));
        let mut list = E::UnorderedList::new();
        for (profile_name, count) in stats {
            list.push(E::Paragraph::new(format!(
                "{} ({} matches)",
                profile_name, count
            )));
        }
        self.doc.push(list);
        self.doc.push(E::Break::new(1.5));
    }

    pub fn render_suspicious_files(&mut self, files: &[(PathBuf, analysis::SuspicionReason)]) {
        if files.is_empty() {
            return;
        }

        self.doc
            .push(E::Paragraph::new("3. Suspicious Findings").styled(self.styles.header_1));

        let mut table = E::TableLayout::new(vec![3, 2, 1, 3]);
        table.set_cell_decorator(E::FrameCellDecorator::new(true, true, false));

        table
            .row()
            .element(E::Paragraph::new("File Path").styled(self.styles.bold))
            .element(E::Paragraph::new("Reason").styled(self.styles.bold))
            .element(E::Paragraph::new("Size").styled(self.styles.bold))
            .element(E::Paragraph::new("Hashes (MD5, SHA256)").styled(self.styles.bold))
            .push()
            .expect("Failed to push suspicious header");

        for (path, reason) in files {
            let size_str = std::fs::metadata(path)
                .map(|m| app_core::format_size(m.len() as usize))
                .unwrap_or_else(|_| "N/A".into());

            let hashes_element = self.create_hash_element(path);

            table
                .row()
                .element(WordWrapParagraph::new(path.to_string_lossy()).styled(self.styles.code))
                .element(WordWrapParagraph::new(reason.to_string()))
                .element(E::Paragraph::new(size_str))
                .element(hashes_element)
                .push()
                .expect("Failed to push suspicious row");
        }

        self.doc.push(table);
        self.doc.push(E::Break::new(1.5));
    }

    pub fn render_detailed_findings(&mut self, api: &AnalysisAPI) {
        self.doc.push(
            E::Paragraph::new("4. Detailed Findings by Profile").styled(self.styles.header_1),
        );

        let stats = api.get_profile_statistics();

        for profile_name in stats.keys() {
            let files = api.get_files_by_profile(profile_name);
            if files.is_empty() {
                continue;
            }

            self.doc.push(
                E::Paragraph::new(format!("Profile: {}", profile_name))
                    .styled(self.styles.header_2),
            );

            let mut table = E::TableLayout::new(vec![4, 1, 5]);
            table.set_cell_decorator(E::FrameCellDecorator::new(true, true, false));

            table
                .row()
                .element(E::Paragraph::new("File Path").styled(self.styles.bold))
                .element(E::Paragraph::new("Size").styled(self.styles.bold))
                .element(E::Paragraph::new("Hashes (MD5, SHA256, BLAKE3)").styled(self.styles.bold))
                .push()
                .expect("Failed to push findings header");

            // Limit results per profile to avoid gigantic PDFs in this version
            for path in files.iter().take(200) {
                let size_str = std::fs::metadata(path)
                    .map(|m| app_core::format_size(m.len() as usize))
                    .unwrap_or_else(|_| "N/A".into());

                let hashes_element = self.create_hash_element(path);

                table
                    .row()
                    .element(
                        WordWrapParagraph::new(path.to_string_lossy()).styled(self.styles.code),
                    )
                    .element(E::Paragraph::new(size_str))
                    .element(hashes_element)
                    .push()
                    .expect("Failed to push findings row");
            }
            self.doc.push(table);
            self.doc.push(E::Break::new(1.0));
        }
    }

    fn create_hash_element(&self, path: &Path) -> Box<dyn Element> {
        match calculate_hashes(path) {
            Ok(h) => {
                let mut layout = E::LinearLayout::vertical();
                layout.push(
                    WordWrapParagraph::new(format!("MD5: {}", h.md5)).styled(self.styles.code),
                );
                layout.push(
                    WordWrapParagraph::new(format!("SHA256: {}", h.sha256))
                        .styled(self.styles.code),
                );
                layout.push(
                    WordWrapParagraph::new(format!("BLAKE3: {}", h.blake3))
                        .styled(self.styles.code),
                );
                layout.into_boxed_element()
            }
            Err(_) => E::Paragraph::new("Error calculating hashes").into_boxed_element(),
        }
    }

    pub fn save(self, path: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        self.doc.render_to_file(path).map_err(|e| e.into())
    }
}

// ============================================================================
//  Section 4: Main Entry Point
// ============================================================================

pub fn generate_pdf_report(
    analysis_api: Rc<RefCell<AnalysisAPI>>,
    storage_api: Rc<RefCell<StorageAPI>>,
) -> Result<(), Box<dyn std::error::Error>> {
    // 1. User Interaction (File Dialog)
    let save_path = rfd::FileDialog::new()
        .add_filter("PDF Document", &["pdf"])
        .set_file_name(format!(
            "Foruster_Report_{}.pdf",
            chrono::Local::now().format("%Y%m%d_%H%M%S")
        ))
        .save_file();

    let path = match save_path {
        Some(p) => p,
        None => return Ok(()), // User cancelled
    };

    // 2. Prepare Data
    let analysis = analysis_api.borrow();
    let storage = storage_api.borrow();

    let summary = analysis.get_analysis_summary()?;
    let profile_stats = analysis.get_profile_statistics();
    let disks = storage.get_disks();
    let suspicious_files = analysis.get_suspicious_files();

    // 3. Build Report
    let mut report = ForensicReport::new()?;

    report.render_title();
    report.render_summary(&summary);
    report.render_disks(disks);
    report.render_profiles_used(&profile_stats);
    report.render_suspicious_files(&suspicious_files);
    report.render_detailed_findings(&analysis);

    // 4. Save
    report.save(path)?;

    Ok(())
}
