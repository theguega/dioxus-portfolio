use crate::Route;
use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");

#[component]
pub fn Navbar() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        div { id: "navbar",
            Link { to: Route::Projects {}, "Projects" }
            Link { to: Route::ResumeEn {}, "Resume (EN)" }
            Link { to: Route::ResumeFr {}, "CV (FR)" }
        }

        Outlet::<Route> {}
    }
}
