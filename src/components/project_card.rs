use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct ProjectCardProps {
    title: String,
    image_url: String,
    image_alt: String,
    description_points: Vec<Element>,
    project_link: Option<String>,
    repo_link: Option<String>,
}

#[component]
pub fn ProjectCard(props: ProjectCardProps) -> Element {
    let project_link_element = |children: Element| -> Element {
        if let Some(link) = &props.project_link {
            rsx! {
                a {
                    href: "{link}",
                    target: "_blank",
                    rel: "noopener noreferrer",
                    {children}
                }
            }
        } else {
            children
        }
    };

    rsx! {
        article { class: "project-card",
            div { class: "project-image-container",
                {project_link_element(rsx! {
                    img { src: "{props.image_url}", alt: "{props.image_alt}" }
                })}
            }
            div { class: "project-details",
                h3 {
                    {project_link_element(rsx! {
                    "{props.title}"
                    })}
                }
                ul { class: "project-points", {props.description_points.into_iter()} }

                // Optional Repo Link Button
                {props.repo_link.as_ref().map(|link| rsx! {
                    div { class: "project-repo-link",
                        a { href: "{link}", target: "_blank", rel: "noopener noreferrer", "View Repository" }
                    }
                })}
            }
        }
    }
}

pub fn project_li(content: &str) -> Element {
    if content.contains("](") && content.contains("[") {
        if let Some(start_text) = content.find('[') {
            if let Some(end_text) = content.find(']') {
                if let Some(start_url) = content.find("](") {
                    if let Some(end_url) = content.find(')') {
                        if start_url == end_text && end_url > start_url + 2 {
                            let before = &content[..start_text];
                            let link_text = &content[start_text + 1..end_text];
                            let url = &content[start_url + 2..end_url];
                            let after = &content[end_url + 1..];

                            return rsx!(
                                li {
                                    "{before}"
                                    a {
                                        href: "{url}",
                                        target: "_blank",
                                        rel: "noopener noreferrer",
                                        "{link_text}"
                                    }
                                    "{after}"
                                }
                            );
                        }
                    }
                }
            }
        }
    }
    rsx!(
        li { "{content}" }
    )
}
