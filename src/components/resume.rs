use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ContactItemProps {
    icon: String,
    text: String,
    href: Option<String>,
}

#[component]
pub fn ContactItem(props: ContactItemProps) -> Element {
    rsx! {
        div { class: "contact-item",
            span { class: "icon", "{props.icon}" }
            if let Some(link) = props.href {
                a {
                    href: "{link}",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    "{props.text}"
                }
            } else {
                span { "{props.text}" }
            }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct SkillCategoryProps {
    title: String,
    skills: String,
}

#[component]
pub fn SkillCategory(props: SkillCategoryProps) -> Element {
    rsx! {
        div { class: "skill-category",
            strong { "{props.title} :" }
            span { " {props.skills}" }
        }
    }
}

#[derive(Props, PartialEq, Clone)]
pub struct TimelineItemProps {
    title: String,
    subtitle: String,
    date: String,
    points: Vec<String>,
}

#[component]
pub fn TimelineItem(props: TimelineItemProps) -> Element {
    rsx! {
        li { class: "timeline-item",
            div { class: "timeline-marker" }
            div { class: "timeline-content",
                div { class: "timeline-header",
                    h4 { class: "timeline-title", "{props.title}" }
                    if !props.date.is_empty() {
                        span { class: "timeline-date", "{props.date}" }
                    }
                }
                p { class: "timeline-subtitle", "{props.subtitle}" }
                ul { class: "timeline-points",
                    {props.points.iter().map(|point| rsx! {
                        li { "{point}" }
                    })}
                }
            }
        }
    }
}
