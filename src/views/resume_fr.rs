use dioxus::prelude::*;

use crate::components::{ContactItem, SkillCategory, TimelineItem};

const RESUME_CSS: Asset = asset!("/assets/styling/resume_fr.css");
const PHOTO: Asset = asset!("/assets/images/photo_theo.jpg");

#[component]
pub fn ResumeFr() -> Element {
    rsx! {
        link { rel: "stylesheet", href: RESUME_CSS }

        div { id: "resume-fr-page",

            // --- HEADER ---
            div { class: "resume-header",
                div { class: "resume-photo",
                    img { src: PHOTO }
                }
                div { class: "resume-info",
                    h1 { "Theo Guegan" }
                    h2 { "Étudiant Ingénieur en Génie Informatique" }
                    p { "Spécialisé en Systèmes Embarqués & Autonomes" }
                    p { class: "objective",
                        "Recherche un stage en Robotique (Février - Juillet 2026)"
                    }
                }
            }

            // --- MAIN CONTENT (Two Columns) ---
            div { class: "resume-body",

                // === LEFT COLUMN ===
                div { class: "resume-left-col",
                    // -- Coordonnées --
                    section { class: "left-col-section",
                        h3 { class: "left-col-title", "Coordonnées" }
                        ContactItem { icon: "📞", text: "07 82 95 45 55" }
                        ContactItem {
                            icon: "✉️",
                            text: "theo.guegan.perso@gmail.com",
                            href: "mailto:theo.guegan.perso@gmail.com",
                        }
                        ContactItem {
                            icon: "🔗",
                            text: "linkedin.com/in/guegan-theo",
                            href: "https://www.linkedin.com/in/guegan-theo",
                        }
                        ContactItem {
                            icon: "🔗",
                            text: "github.com/theguega",
                            href: "https://github.com/theguega",
                        }
                        ContactItem { icon: "🚗", text: "Permis B" }
                    }
                    // -- Profil --
                    section { class: "left-col-section",
                        h3 { class: "left-col-title", "Profil" }
                        p {
                            "Étudiant Ingénieur, spécialisé en Informatique Embarquée et Systèmes Autonomes. Recherche stage de fin d'études (6 mois) en robotique pour appliquer mes compétences (C++, Rust, Python, Contrôle, Autonomie) à des projets innovants. Motivé à relever de nouveaux défis techniques."
                        }
                    }
                    // -- Compétences --
                    section { class: "left-col-section",
                        h3 { class: "left-col-title", "Compétences" }
                        SkillCategory {
                            title: "Langages",
                            skills: "C++, Rust, Python, C, Bash, Lua",
                        }
                        SkillCategory {
                            title: "Outils",
                            skills: "Linux, Git, Docker, CMake, ROS, TDD, Flutter",
                        }
                        SkillCategory {
                            title: "Robotique & Autonomie",
                            skills: "Contrôle (PID), Nav. Autonome, Planif. Trajectoire, Fusion Capteurs (Base), Prise de Décision, Coord. Essaims, Systèmes Embarqués, RL (Base).",
                        }
                        SkillCategory {
                            title: "Soft-Skills",
                            skills: "Gestion de Projet, Travail en Équipe, Adaptabilité, Motivation, Innovation.",
                        }
                    }
                    // -- Langues --
                    section { class: "left-col-section",
                        h3 { class: "left-col-title", "Langues" }
                        p {
                            strong { "Français :" }
                            " Maternelle"
                        }
                        p {
                            strong { "Anglais :" }
                            " C1"
                        }
                        p {
                            strong { "Espagnol :" }
                            " B2"
                        }
                    }
                    // -- Activités --
                    section { class: "left-col-section",
                        h3 { class: "left-col-title", "Activités" }
                        ul {
                            li { "Street-Workout" }
                            li { "Surf" }
                            li { "Escalade" }
                            li { "Kite-Surf" }
                        }
                    }
                }

                // === RIGHT COLUMN ===
                div { class: "resume-right-col",
                    // -- Expérience Professionnelle --
                    section { class: "right-col-section",
                        h2 { class: "section-title", "Expérience Professionnelle" }
                        ul { class: "timeline",
                            TimelineItem {
                                title: "Stagiaire - Ingénieur Logiciel Embarqué Drone",
                                subtitle: "Thales Land & Air Systems | Vélizy-Villacoublay",
                                date: "Sept. 2024 – Fév. 2025",
                                points: vec![
                                    "Développement d'un moteur de script Lua embarqué en C++ (TDD) pour la gestion de missions complexes sur essaims de drones."
                                        .to_string(),
                                    "Création d'une application multiplateforme (Flutter) pour la gestion d'essaims de drones, présentée au Ministère des Armées."
                                        .to_string(),
                                    "Intégration d'un LLM local (Rust, Docker) pour commandes en langage naturel afin de faciliter la création de mission par un opérateur."
                                        .to_string(),
                                ],
                            }
                            TimelineItem {
                                title: "Chef de Projet",
                                subtitle: "Junior UTC (Junior-Entreprise) | Compiègne",
                                date: "Sept. 2023 – Sept. 2024",
                                points: vec![
                                    "Gestion de projets pour la Junior Entreprise de l'UTC pour des clients tels que Airbus et Median Technologies."
                                        .to_string(),
                                    "Gestion du cycle projet complet (cadrage, planification, contrat.), signature de 30k€ de contrats et gestion des étudiants réalisateurs."
                                        .to_string(),
                                ],
                            }
                        }
                    }
                    // -- Formation --
                    section { class: "right-col-section",
                        h2 { class: "section-title", "Formation" }
                        ul { class: "timeline",
                            TimelineItem {
                                title: "Diplôme d'Ingénieur - Informatique",
                                subtitle: "Université de Technologie de Compiègne (UTC) | Compiègne",
                                date: "Prévu 2026",
                                points: vec![
                                    "Filière : Informatique Embarquée, Systèmes Autonomes".to_string(),
                                    "GPA : 5.0/5.0".to_string(),
                                ],
                            }
                            TimelineItem {
                                title: "Programme d'échange - Ingénierie",
                                subtitle: "University of Waterloo | Waterloo, Canada",
                                date: "Sept. 2025 – Déc. 2025",
                                points: vec![
                                    "Échange de 4 mois, systèmes embarqués avancés, systèmes d'exploitation temps réel."
                                        .to_string(),
                                ],
                            }
                        }
                    }
                    // -- Projets --
                    section { class: "right-col-section",
                        h2 { class: "section-title", "Projets" }
                        ul { class: "timeline",
                            TimelineItem {
                                title: "LeRobot - Bras Robotique Téléopéré",
                                subtitle: "Projet Personnel",
                                date: "",
                                points: vec![
                                    "Implémentation de systèmes de contrôle robotique (PID) et exploration de politiques RL pour téléopération (Python)."
                                        .to_string(),
                                ],
                            }
                            TimelineItem {
                                title: "Resp. - Décision & Contrôle Véhicules Autonomes",
                                subtitle: "Challenge UTAC | Prix Meilleure École",
                                date: "Fév. – Juin 2024",
                                points: vec![
                                    "Développement d'un contrôleur pour véhicule autonome sur Matlab."
                                        .to_string(),
                                    "Conception/implémentation : navigation basée waypoint, ACC, évitement d'obstacles."
                                        .to_string(),
                                ],
                            }
                        }
                    }
                }
            }
        }
    }
}
