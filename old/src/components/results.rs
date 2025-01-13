// SPDX-License-Identifier: GPL-3.0-or-later
use std::path::PathBuf;

use charts_rs::{Color, PieChart, Series};
use dioxus::prelude::*;
use native_dialog::FileDialog;
use select::document::Document;
use select::predicate::Name;
use simple_pdf_generator::{Asset, AssetType, PrintOptions};
use simple_pdf_generator_derive::PdfTemplate;

use crate::{
    get_profile_color, get_suspicious_files, show_error, AppState, Disk, FileEntry, ModalInfo, Report, REPORT_TEMPLATE_CSS_URL, REPORT_TEMPLATE_HTML_URL, SUSPICIOUS_FILES_COLOR, SVG_CHART_SIZE
};

struct ChartProfile {
    pub id: usize,
    pub name: String,
    pub color: Color,
    pub files_count: usize,
}

#[component]
pub fn Results() -> Element {
    let app_state = use_context::<Signal<AppState>>();
    let hashing_progress = app_state.peek().hashing_progress;

    let report = use_context::<Signal<Report>>();
    let profiles = report.peek().selected_profiles.clone();

    let suspicious_files = get_suspicious_files(report);
    let suspicious_file_len = suspicious_files.len();

    let represented_profiles = profiles
        .iter()
        .flat_map(|profile| {
            let id = profile.id();
            let name = profile.name().to_string();
            let color = get_profile_color(profile.profile_type());
            let files_count = profile.files().len();
            if files_count > 0 {
                Some(ChartProfile {
                    id,
                    name,
                    color,
                    files_count,
                })
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    let mut series_values = represented_profiles
        .iter()
        .map(|profile| {
            Series::new(
                format!(
                    "{} ({} {})",
                    profile.name,
                    profile.files_count,
                    if profile.files_count == 1 {
                        "archivo"
                    } else {
                        "archivos"
                    }
                ),
                vec![profile.files_count as f32],
            )
        })
        .collect::<Vec<Series>>();

    if suspicious_file_len > 0 {
        series_values.push(Series::new(
            format!(
                "Archivos sospechosos ({} {})",
                suspicious_file_len,
                if suspicious_file_len == 1 {
                    "archivo"
                } else {
                    "archivos"
                }
            ),
            vec![suspicious_file_len as f32],
        ));
    }

    let mut pie_chart = PieChart::new(series_values);

    pie_chart.width = SVG_CHART_SIZE as f32;
    pie_chart.height = SVG_CHART_SIZE as f32 / 2.0;
    pie_chart.series_colors = represented_profiles.iter().map(|p| p.color).collect();
    pie_chart.series_colors.push(SUSPICIOUS_FILES_COLOR);

    let mut ids = represented_profiles
        .iter()
        .map(|p| p.id)
        .collect::<Vec<usize>>();
    ids.push(1337);
    let svg_chart = convert_chart_to_rsx(pie_chart.svg().unwrap(), ids);

    rsx! {
        div {
            class: "flex flex-col items-center justify-center",

            div {
                { svg_chart }
            }

            if hashing_progress() == 100 {
                div {
                    class: "text-center w-full",
                    button {
                        onclick: move |_| { export_report(report) },
                        class: "btn btn-primary m-4",
                        "Exportar informe"
                    }
                }
            } else {
                div {
                    class: "text-center w-full",
                    p {
                        class: "text-xl font-bold",
                        "Calculando hash de los archivos ({hashing_progress} %)"
                    }
                }
            }
        }
    }
}

fn convert_chart_to_rsx(chart: String, ids: Vec<usize>) -> Element {
    let document = Document::from(chart.as_str());

    let mut rsx_paths = vec![];
    for path in document.find(Name("path")) {
        let d = path.attr("d").unwrap();
        let fill = path.attr("fill").unwrap();
        let stroke = path.attr("stroke");
        let rsx_path = if let Some(stroke) = stroke {
            let stroke_width = path.attr("stroke-width").unwrap();
            rsx! {
                path {
                    d: d,
                    fill: fill,
                    stroke: stroke,
                    stroke_width: stroke_width,
                }
            }
        } else {
            rsx! {
                path {
                    d: d,
                    fill: fill,
                }
            }
        };

        rsx_paths.push(rsx_path);
    }

    let mut rsx_texts = vec![];
    for text in document.find(Name("text")) {
        let rsx_text = rsx! {
            text {
                font_size: r#"{text.attr("font-size").unwrap()}"#,
                x: r#"{text.attr("x").unwrap()}"#,
                y: r#"{text.attr("y").unwrap()}"#,
                font_family: r#"{text.attr("font-family").unwrap()}"#,
                fill: r#"{text.attr("fill").unwrap()}"#,
                {text.text()}
            }
        };

        rsx_texts.push(rsx_text);
    }

    let mut rsx_slices = vec![];
    for (text_index, text) in rsx_texts.iter().enumerate() {
        let id = ids[text_index];
        rsx_slices.push(rsx! {
            g {
                class: "group cursor-pointer",
                onclick: move |_| {
                    navigator().push(crate::Route::DetailedResults { id });
                },

                { rsx_paths[text_index * 2].clone() }
                { rsx_paths[text_index * 2 + 1].clone() }
                { text }
            }
        });
    }

    let rsx_chart = rsx! {
        svg {
            class: "m-4",
            width: "{SVG_CHART_SIZE}",
            height: "{SVG_CHART_SIZE/2}",
            view_box: "0 0 {SVG_CHART_SIZE} {SVG_CHART_SIZE/2}",
            xmlns: "http://www.w3.org/2000/svg",

            g {
                class: "container",

                { rsx_slices.iter() }
            }
        }
    };

    rsx_chart
}

#[component]
fn SuspiciousFileCard(file: FileEntry) -> Element {
    let modal_info = use_context::<Signal<ModalInfo>>();
    let file_type = match infer::get(file.magic_bytes()) {
        Some(t) => t.extension(),
        None => "unknown",
    };

    let extension = match file.extension() {
        Some(e) => e.to_str().unwrap(),
        None => "unknown",
    };

    let folder_path = match file.path().parent() {
        Some(p) => p.to_path_buf(),
        None => PathBuf::new(),
    };

    rsx! {
        div {
            class: "card w-96 bg-base-100 shadow-xl m-4",
            div {
                class: "card-body",
                h2 {
                    class: "card-title break-all",
                    { file.name().to_string_lossy() }
                }
                p {
                    class: "justify-start text-sm break-words",
                    { file.path().to_string_lossy() }
                }

                div {
                    class: "flex text-left",
                    p {
                        class: "text-sm font-bold",
                        { format!("El fichero es un {} pero tiene la extensión {}", file_type, extension) }
                    }
                }

                div {
                    class: "card-actions justify-end",
                    button {
                        onclick: move |_| {
                            if let Err(e) = open::that(&folder_path) {
                                show_error(modal_info, &e.to_string())
                            }
                        },
                        class: "btn",
                        "Abrir carpeta contenedora"
                    }
                }
            }
        }
    }
}

#[derive(PdfTemplate)]
struct ReportTemplateArgs {
    date: String,
    profile_name: String,
    profile_extensions: String,
    #[PdfTableData]
    selected_disks: Vec<Disk>,
    #[PdfTableData]
    filtered_files: Vec<FileEntry>,
}

async fn export_report(report: Signal<Report>) {
    let selected_disks = report.peek().selected_disks.clone();
    let selected_profiles = report.peek().selected_profiles.clone();
    let html_path = format!("dist{}", REPORT_TEMPLATE_HTML_URL);
    let css_path = format!("dist{}", REPORT_TEMPLATE_CSS_URL);

    let output_path = match FileDialog::new().set_location("~/").show_open_single_dir() {
        Ok(Some(path)) => path,
        _ => {
            show_error(
                use_context::<Signal<ModalInfo>>(),
                "No se ha seleccionado una carpeta válida",
            );
            return;
        }
    };

    let date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let assets = [Asset {
        path: css_path.into(),
        r#type: AssetType::Style,
    }];

    for profile in selected_profiles {
        let pdf_name = format!("{}_informe.pdf", profile.name());
        let pdf_output_path = output_path.join(pdf_name);

        let template_args = ReportTemplateArgs {
            date: date.clone(),
            profile_name: profile.name().to_string(),
            profile_extensions: profile.extensions().join(", "),
            selected_disks: selected_disks.clone(),
            filtered_files: profile.files().to_vec(),
        };

        let print_options = PrintOptions {
            paper_width: Some(210.0),  // A4 paper size in mm
            paper_height: Some(297.0), // A4 paper size in mm
            margin_top: Some(10.0),    // 10mm margin
            margin_bottom: Some(10.0), // 10mm margin
            margin_left: Some(10.0),   // 10mm margin
            margin_right: Some(10.0),  // 10mm margin
            ..PrintOptions::default()
        };

        let pdf_buf = match template_args
            .generate_pdf(html_path.clone().into(), &assets, &print_options)
            .await
        {
            Ok(buf) => buf,
            Err(_) => {
                vec![]
            }
        };

        let _ = std::fs::write(pdf_output_path.clone(), pdf_buf);
    }
}
