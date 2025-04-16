use dioxus::prelude::*;

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        p {
            " Go back to the "
            a { href: "/", "homepage" }
            " or try a different link."
        }
        {}
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}
