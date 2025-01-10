// SPDX-License-Identifier: GPL-3.0-or-later
use dioxus::prelude::*;
use dioxus_free_icons::icons::bs_icons::{BsPen, BsPlusCircle};
use dioxus_free_icons::Icon;

use crate::{ProfileCard, Report};

#[component]
pub fn ProfileFilter() -> Element {
    let nav = navigator();

    let report = use_context::<Signal<Report>>();
    let profiles = report.peek().selected_profiles.clone();
    let profiles_info = profiles.iter().map(|profile| {
        let id = profile.id();
        rsx! {
            div {
                class: "card w-full bg-base-100 shadow-xl m-4 ring-2 ring-transparent glass",
                ProfileCard {
                    profile: profile.clone(),
                    is_form: false
                }
                div {
                    class: "card-actions justify-end m-2",
                    button {
                        class: "btn btn-accent btn-circle text-2xl",
                        onclick: move |_| {
                            nav.push(crate::Route::ModifyFilter {
                                profile_id: id,
                            });
                        },
                        if profile.filter_options().is_none() {
                            Icon {
                                icon: BsPlusCircle
                            }
                        } else {
                            Icon {
                                icon: BsPen
                            }
                        }
                    }
                }
            }
        }
    });

    rsx! {
        div {
            class: "flex flex-col w-full",
            h1 {
                class: "text-center text-4xl m-4 font-bold",
                "Añade los filtros necesarios:"
            }
            div {
                class: "flex flex-wrap justify-center",
                { profiles_info }
            }
            Link {
                class: "btn btn-primary m-4",
                to: "/filter_files",
                "Continuar"
            }
        }
    }
}
