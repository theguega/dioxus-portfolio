use dioxus::prelude::*;

use crate::components::{project_li, ProjectCard};

const PROJECTS_CSS: Asset = asset!("/assets/styling/projects.css");
const HEADER_SVG: Option<Asset> = None;

const THALES_IMG_URL: &str = "https://www.thalesgroup.com/sites/default/files/prezly/images/Design%20sans%20titre%20%281%29_0.png";
const TURTLEBOT_IMG_URL: &str =
    "https://robodyne-services.com/wp-content/uploads/2021/10/turtlebot3.png";
const AUTONOMOUS_VEHICLE_IMG_URL: &str = "https://github.com/theguega/Autonomous-Vehicles-Decisions-Control/blob/main/doc/doc1.jpg?raw=true";
const AI_GO_IMG_URL: &str =
    "https://github.com/theguega/AI-for-Go-like-game/blob/main/doc/dodo.jpg?raw=true";
const LOCAL_LLM_IMG_URL: &str =
    "https://github.com/theguega/Local-LLM-WebServer/blob/main/frontend/public/favico.png?raw=true";
const LEROBOT_IMG_URL: &str =
    "https://github.com/huggingface/lerobot/blob/main/media/so100/leader_follower.webp?raw=true";

#[component]
pub fn Projects() -> Element {
    rsx! {
        link { rel: "stylesheet", href: PROJECTS_CSS }

        div { id: "projects-page",
            if let Some(svg) = HEADER_SVG {
                img {
                    class: "projects-header-svg",
                    src: svg,
                    alt: "Projects Header Graphic",
                }
            }

            h1 { "Projects Overview" }

            div { class: "projects-list",

                // Project 1: Thales Internship
                ProjectCard {
                    title: "Internship at Thales LAS - Autonomous Drone Swarm".to_string(),
                    image_url: THALES_IMG_URL.to_string(),
                    image_alt: "Thales Drone Swarm".to_string(),
                    project_link: Some(
                        "https://www.thalesgroup.com/en/worldwide/defence-and-security/press_release/thales-demonstrates-its-capacity-deploy-drone-swarms"
                            .to_string(),
                    ),
                    description_points: vec![
                        project_li(
                            "Developed a real-time embedded Lua scripting engine in C++ (TDD), enabling complex, on-drone mission logic customization for autonomous tasks.",
                        ),
                        project_li(
                            "Built a cross-platform Flutter application for drone swarm management, enhancing multi-device coordination, and presented capabilities to the French Ministry of Defence.",
                        ),
                        project_li(
                            "Integrated a local LLM using Rust and Docker for natural-language drone commands, contributing to a groundbreaking autonomous drone swarm project showcased to media.",
                        ),
                        project_li(
                            "Learn more: [Thales demonstrates its capacity to deploy drone swarms](https://www.thalesgroup.com/en/worldwide/defence-and-security/press_release/thales-demonstrates-its-capacity-deploy-drone-swarms)",
                        ),
                    ],
                    repo_link: None,
                }

                // Project 2: TurtleBot Maze Mapping
                ProjectCard {
                    title: "TurtleBot Maze Mapping".to_string(),
                    image_url: TURTLEBOT_IMG_URL.to_string(),
                    image_alt: "TurtleBot Robot".to_string(),
                    project_link: Some("https://github.com/theguega/Ros-Maze-Mapping".to_string()),
                    description_points: vec![
                        project_li("Developed a maze-mapping solution using ROS."),
                        project_li("Integrated camera, LiDAR, ultrasound, and odometry."),
                        project_li("Enabled autonomous exploration and mapping of unknown environments."),
                    ],
                    repo_link: Some("https://github.com/theguega/Ros-Maze-Mapping".to_string()),
                }

                // Project 3: Autonomous Vehicle Control
                ProjectCard {
                    title: "Autonomous Vehicle Control Architecture".to_string(),
                    image_url: AUTONOMOUS_VEHICLE_IMG_URL.to_string(),
                    image_alt: "Autonomous Vehicle Control Diagram".to_string(),
                    project_link: Some(
                        "https://github.com/theguega/Autonomous-Vehicles-Decisions-Control".to_string(),
                    ),
                    description_points: vec![
                        project_li("Designed adaptive control architectures for automated vehicles."),
                        project_li(
                            "Implemented Cruise Control (ACC), static obstacle avoidance, and waypoint navigation.",
                        ),
                    ],
                    repo_link: Some(
                        "https://github.com/theguega/Autonomous-Vehicles-Decisions-Control".to_string(),
                    ),
                }

                // Project 4: AI for Go-like Games
                ProjectCard {
                    title: "AI Strategy for Go-like Games".to_string(),
                    image_url: AI_GO_IMG_URL.to_string(),
                    image_alt: "Dodo board game screenshot".to_string(),
                    project_link: Some("https://github.com/theguega/AI-for-Go-like-game".to_string()),
                    description_points: vec![
                        project_li("Implemented AI strategies for Dodo and Gopher games (Python)."),
                        project_li("Utilized Alpha-Beta pruning and Monte Carlo Tree Search (MCTS)."),
                    ],
                    repo_link: Some("https://github.com/theguega/AI-for-Go-like-game".to_string()),
                }

                // Project 5: Local LLM Webserver
                ProjectCard {
                    title: "Local LLM Webserver".to_string(),
                    image_url: LOCAL_LLM_IMG_URL.to_string(),
                    image_alt: "LLM Webserver Icon".to_string(),
                    project_link: Some("https://github.com/theguega/Local-LLM-WebServer".to_string()),
                    description_points: vec![
                        project_li("A simple web-server to run local LLMs."),
                        project_li("Built using Rust backend and Next.js + Tailwind frontend."),
                    ],
                    repo_link: Some("https://github.com/theguega/Local-LLM-WebServer".to_string()),
                }

                // Project 6: LeRobot - Robotic Arm
                ProjectCard {
                    title: "LeRobot - Robotic Arm".to_string(),
                    image_url: LEROBOT_IMG_URL.to_string(),
                    image_alt: "LeRobot robotic arm simulation".to_string(),
                    project_link: Some("https://huggingface.co/lerobot".to_string()),
                    description_points: vec![
                        project_li(
                            "Personal project inspired by [huggingface/lerobot](https://huggingface.co/lerobot).",
                        ),
                        project_li("Aiming to learn robotics and reinforcement learning."),
                        project_li("Building upon the open-source foundation of LeRobot."),
                        project_li("Implementing RL algorithms for dexterous manipulation."),
                        project_li("Exploring inverse kinematics and motion planning."),
                    ],
                    repo_link: Some("https://huggingface.co/lerobot".to_string()),
                }
            }
        }
    }
}
