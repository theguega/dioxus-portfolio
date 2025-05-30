use dioxus::prelude::*;

const RESUME_EN_CSS: Asset = asset!("/assets/styling/resume_en.css");

#[component]
pub fn ResumeEn() -> Element {
    rsx! {
        link { rel: "stylesheet", href: RESUME_EN_CSS }
        div { id: "resume-en-page-single-col",
            header { class: "resume-en-header",
                h1 { "Theo Guegan" }
                h2 { "Robotics Internship - From February to July 2026" }
                div { class: "contact-block-en",
                    span { class: "contact-item-en",
                        span { class: "icon", "📍" }
                        "French Citizen"
                    }
                    span { class: "contact-sep-en", "|" }
                    span { class: "contact-item-en",
                        span { class: "icon", "✉️" }
                        a { href: "mailto:theo.guegan.perso@gmail.com", "theo.guegan.perso@gmail.com" }
                    }
                    br {}
                    span { class: "contact-item-en",
                        span { class: "icon", "🔗" }
                        a {
                            href: "https://www.linkedin.com/in/guegan-theo",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            "linkedin.com/in/guegan-theo"
                        }
                    }
                    span { class: "contact-sep-en", "|" }
                    span { class: "contact-item-en",
                        span { class: "icon", "🔗" }
                        a {
                            href: "https://github.com/theguega",
                            target: "_blank",
                            rel: "noopener noreferrer",
                            "github.com/theguega"
                        }
                    }
                }
            }

            // --- SUMMARY ---
            p { class: "resume-en-summary",
                "Computer Engineering student, specializing in Embedded Computing and Autonomous Systems at Université de Technologie de Compiègne (France). Seeking a challenging 6-month robotics internship to apply strong skills in C++, Rust, and Python for embedded software, control systems, and robotics in innovative projects within a leading technology company."
            }

            // --- EXPERIENCE SECTION ---
            section { class: "section-en",
                h2 { class: "section-title-en", "EXPERIENCE" }

                article { class: "entry-en",
                    div { class: "entry-header-en",
                        span { class: "entry-title-en", "Intern Embedded Drone Software Engineer" }
                        span { class: "entry-date-en", "September 2024 – February 2025" }
                    }
                    p { class: "entry-subtitle-en",
                        "Thales Land & Air Systems | Vélizy-Villacoublay, France"
                    }
                    ul { class: "entry-points-en",
                        li {
                            "Developed a real-time embedded Lua scripting engine in C++ (TDD), enabling complex, on-drone mission logic customization for autonomous tasks."
                        }
                        li {
                            "Built a cross-platform Flutter application for drone swarm management, enhancing multi-device coordination, and presented capabilities to the French Ministry of Defence."
                        }
                        li {
                            "Integrated a local LLM using Rust and Docker for natural-language drone commands, contributing to a groundbreaking autonomous drone swarm project showcased to media."
                        }
                    }
                }

                article { class: "entry-en",
                    div { class: "entry-header-en",
                        span { class: "entry-title-en", "Volunteer Business Manager" }
                        span { class: "entry-date-en", "September 2023 – September 2024" }
                    }
                    p { class: "entry-subtitle-en",
                        "Junior UTC (Student Association) | Compiègne, France"
                    }
                    ul { class: "entry-points-en",
                        li {
                            "Led technical project execution for clients including Airbus and Median Technologies, delivering professional solutions."
                        }
                        li {
                            "Managed full project lifecycle (scoping, planning, contracts), signing over €30,000 in projects by coordinating student engineers."
                        }
                    }
                }
            }

            // --- EDUCATION SECTION ---
            section { class: "section-en",
                h2 { class: "section-title-en", "EDUCATION" }

                article { class: "entry-en",
                    div { class: "entry-header-en",
                        span { class: "entry-title-en", "Master of Science - Computer Engineering" }
                        span { class: "entry-date-en", "Expected in 2026" }
                    }
                    p { class: "entry-subtitle-en",
                        "Université de Technologie de Compiègne (UTC) | Compiègne, France"
                    }
                    ul { class: "entry-points-en",
                        li { "Minor: Embedded Computing, Equipment and Autonomous Systems." }
                        li { "GPA: 4.0/4.0." }
                    }
                }

                article { class: "entry-en",
                    div { class: "entry-header-en",
                        span { class: "entry-title-en", "Exchange Program - Faculty of Engineering" }
                        span { class: "entry-date-en", "September 2025 – December 2025" }
                    }
                    p { class: "entry-subtitle-en", "University of Waterloo | Waterloo, Canada" }
                    ul { class: "entry-points-en",
                        li { "Focused coursework on embedded systems integration and networks." }
                    }
                }
            }

            // --- PROJECTS SECTION ---
            section { class: "section-en",
                h2 { class: "section-title-en", "PROJECTS" }

                article { class: "entry-en",
                    div { class: "entry-header-en",
                        span { class: "entry-title-en", "LeRobot - Teleoperated Robotic Arm Project" }
                    }
                    p { class: "entry-subtitle-en",
                        "Personal Project | Replicated Hugging Face LeRobot"
                    }
                    ul { class: "entry-points-en",
                        li {
                            "Gained practical experience implementing low-cost, real-world robotics control systems and exploring reinforcement learning (RL) policies for teleoperation tasks using Python."
                        }
                    }
                }

                article { class: "entry-en",
                    div { class: "entry-header-en",
                        span { class: "entry-title-en",
                            "Leader - Decision Making & Control for Autonomous Vehicle Fleet"
                        }
                        span { class: "entry-date-en", "2024 – 2025" }
                    }
                    p { class: "entry-subtitle-en",
                        "UTAC Challenge | 2024 - 1st: Best School Award | 2025 - 1st: Free Category"
                    }
                    ul { class: "entry-points-en",
                        li {
                            "Led team development of a robust autonomous navigation stack, achieving 1st place in the UTAC European Competition."
                        }
                        li {
                            "Designed and implemented key features: target-based navigation, adaptive cruise control (ACC), and static obstacle avoidance in MATLAB."
                        }
                    }
                }

                article { class: "entry-en",
                    div { class: "entry-header-en",
                        span { class: "entry-title-en", "Coding Competition" }
                        span { class: "entry-date-en", "2025" }
                    }
                    p { class: "entry-subtitle-en", "FIT Coding Challenge | Bosnia-Herzegovina" }
                    ul { class: "entry-points-en",
                        li { "Solved complex LeetCode style problems in C++ under time constraints." }
                    }
                }

                article { class: "entry-en",
                    div { class: "entry-header-en",
                        span { class: "entry-title-en", "Distributed Payment App" }
                    }
                    p { class: "entry-subtitle-en", "Personal Project" }
                    ul { class: "entry-points-en",
                        li {
                            "Developed a peer-to-peer payment application replicating our school's app using a distributed systems approach in Rust, with a full-stack, cross-platform interface built with Dioxus."
                        }
                    }
                }

                article { class: "entry-en",
                    div { class: "entry-header-en",
                        span { class: "entry-title-en", "Real-Time Kernel" }
                    }
                    p { class: "entry-subtitle-en", "Personal Project" }
                    ul { class: "entry-points-en",
                        li {
                            "Implemented a bare-metal real-time kernel in C for the STM32H747I-DISCO board, featuring mutexes and semaphores to manage basic tasks."
                        }
                    }
                }
            }


            // --- SKILLS SECTION ---
            section { class: "skills-section-en",
                h2 { class: "section-title-en", "SKILLS" }
                p { class: "skill-listing-en",
                    strong { "Programming Languages:" }
                    "Python, C++, Rust, C, Bash, Lua, Dart"
                }
                p { class: "skill-listing-en",
                    strong { "Technologies & Tools:" }
                    "Linux, Git, Docker, CMake, ROS, Real-Time Operating Systems (RTOS), TDD, Flutter"
                }
                p { class: "skill-listing-en",
                    strong { "Robotics & Autonomy:" }
                    "Control Systems, Autonomous Navigation, Path Planning, Sensor Fusion, Decision-Making, Embedded Systems, Real-Time Systems, Reinforcement Learning, LLM Integration"
                }
            }
        }
    }
}
