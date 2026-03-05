use super::education::EducationData;
use super::employment::EmploymentData;
use super::projects::ProjectData;
use std::sync::LazyLock;

pub static EDUCATION_ITEMS: LazyLock<Vec<EducationData>> = LazyLock::new(|| {
    vec![
        EducationData {
            university: String::from("VIT-AP University, India"),
            major: String::from("B.tech Computer Science"),
            minor: None,
            specialization: String::from("Blockchain and Artificial Intelligence"),
            start_date: String::from("August 2023"),
            end_date: String::from("Present"),
            gpa: Some(8.0),
            transcript_link: Some(String::from(
                "https://vitap.ac.in",
            )),
        },
        EducationData {
            university: String::from("Vidyaa Vikas Metric School, India"),
            major: String::from("Computer Science"),
            minor: Some(String::from("Mathematics")),
            specialization: String::from("Higher Secondary Education"),
            start_date: String::from("September 2011"),
            end_date: String::from("June 2023"),
            gpa: Some(7.9),
            transcript_link: Some(String::from("https://thanjai.vidyaavikas.ac.in")),
        },
    ]
});

pub static PROJECT_ITEMS: LazyLock<Vec<ProjectData>> = LazyLock::new(|| {
    vec![
        ProjectData {
            title: String::from("ChainKey - Decentralized API State Machine"),
            link: String::from("https://github.com/Eniyanyosuva/ChainKey"),
            date: String::from("March 2026"),
            description: String::from(
                "A decentralized API key management protocol built on Solana that replaces
traditional key infrastructure such as databases, Redis caches, and
authentication middleware with a deterministic on-chain state machine.

ChainKey manages the full lifecycle of API credentials directly on the
Solana ledger, enabling cryptographically verifiable access control,
immutable audit trails, secure key rotation, and enforceable rate limits.
By removing centralized key storage and authentication layers, the system
eliminates single points of failure while providing transparent,
tamper-resistant credential management for modern API services.",
            ),
        },
        ProjectData {
            title: String::from("Solsplit - Splitting protocol smart contracts"),
            link: String::from("https://github.com/Eniyanyosuva/solsplit"),
            date: String::from("June 2025"),
            description: String::from(
                "A decentralized payment splitting protocol built on Solana, powered by secure on-chain smart contracts that enable instant, fee-free distribution of SOL to multiple recipients. It's designed for freelancers, content creators, and businesses that need automated, trustless payment splitting with transparency and speed.",
            ),
        },
        ProjectData {
            title: String::from("rust portfolio website"),
            link: String::from("https://github.com/Eniyanyosuva/portfolio"),
            date: String::from("June 2025"),
            description: String::from(
                "A personal portfolio website built using Rust and the Yew framework, showcasing my projects, skills, and experience in a modern, responsive design. The site leverages Rust's performance and safety features to deliver a fast and secure user experience.",
            ),
        },
    ]
});

pub static EMPLOYMENT_ITEMS: LazyLock<Vec<EmploymentData>> = LazyLock::new(|| {
    vec![
        EmploymentData {
            title: String::from("fullstack Dapp Developer"),
            company: String::from("Solmellons"),
            location: String::from("Indian, Remote"),
            start_date: String::from("October 2025"),
            end_date: String::from("Present"),
            link: String::from("https://www.linkedin.com/company/dev-watermelons/"),
            description_bullets: vec![
                String::from("Built and deployed secure, efficient smart contracts, and enhanced blockchain infrastructure to increase transaction throughput and strengthen security. Worked closely with remote, cross-functional teams to ensure smooth project delivery. Mentored junior developers, promoting best practices and high coding standards in blockchain development."),
                String::from(
                    "Led the design and development of decentralized applications on Solana using Rust, specializing in DeFi and NFT platforms",
                ),
            ],
        },
        EmploymentData {
            title: String::from("Blockchain intern"),
            company: String::from("solmellons"),
            location: String::from("India, Remote"),
            start_date: String::from("June 2025"),
            end_date: String::from("October 2025"),
            link: String::from("https://www.linkedin.com/company/dev-watermelons/"),
            description_bullets: vec![
                String::from(
                    "Improved platform scalability and security through optimized smart contract and infrastructure design Successfully launched multiple DeFi and NFT dApps, Played a key role in setting robust blockchain development standards across the team",
                ),
                String::from(
                    "Collaborated with cross-functional teams to deliver projects on time and mentored junior developers, fostering a culture of continuous learning and excellence in blockchain development.",
                ),
            ],
        },

    ]
});
