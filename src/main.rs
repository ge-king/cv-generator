#![feature(proc_macro_hygiene, decl_macro)]


use rocket::{Rocket, post, routes};
use rocket::serde::json::Json;
use serde::*;
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
struct CVInput {
    personal_info: PersonalInfo,
    experience: Vec<Experience>,
    education: Vec<Education>,
    skills: Vec<Skill>,
    languages: Vec<Language>,
    references: Vec<Reference>,
}

#[derive(Deserialize)]
struct PersonalInfo {
    name: String,
    email: String,
    phone: Option<String>,
    address: Option<String>,
    linkedin: Option<String>,
    github: Option<String>,
    personal_website: Option<String>,
    objective: Option<String>,
}

#[derive(Deserialize)]
struct Experience {
    job_title: String,
    company: String,
    location: String,
    start_date: String,
    end_date: Option<String>,  
    responsibilities: Vec<String>,
}

#[derive(Deserialize)]
struct Education {
    degree: String,
    institution: String,
    location: String,
    start_date: String,
    end_date: String,
    gpa: Option<f32>,
    honors: Option<String>,
}

#[derive(Deserialize)]
struct Skill {
    name: String,
    proficiency: Option<String>, 
}

#[derive(Deserialize)]
struct Language {
    name: String,
    proficiency: Option<String>, 
}

#[derive(Deserialize)]
struct Reference {
    name: String,
    relation: String,
    email: String,
    phone: Option<String>,
}


fn generate_latex(input: &CVInput) -> String {
    format!(
        "\\documentclass[letterpaper, 10pt]{{article}}
        \\usepackage[utf8]{{inputenc}}
        \\usepackage[left=1in, right=1in, top=1in, bottom=1in]{{geometry}}
        \\usepackage{{enumitem}}
        \\usepackage{{hyperref}}

        \\begin{{document}}

        \\section*{{Personal Details}}
        Name: {} \\
        Email: {} \\
        Phone: {} \\
        Address: {} \\
        LinkedIn: \\href{{{}}}{{LinkedIn}} \\
        GitHub: \\href{{{}}}{{GitHub}} \\
        Website: \\href{{{}}}{{Website}} \\
        Objective: {}

        \\section*{{Experience}}
        {}
        
        \\section*{{Education}}
        {}

        \\section*{{Skills}}
        \\begin{{itemize}}
        {}
        \\end{{itemize}}

        \\section*{{Languages}}
        \\begin{{itemize}}
        {}
        \\end{{itemize}}

        \\section*{{References}}
        {}

        \\end{{document}}",
        input.personal_info.name,
        input.personal_info.email,
        input.personal_info.phone.as_ref().unwrap_or(&"N/A".to_string()),
        input.personal_info.address.as_ref().unwrap_or(&"N/A".to_string()),
        input.personal_info.linkedin.as_ref().unwrap_or(&"#".to_string()),
        input.personal_info.github.as_ref().unwrap_or(&"#".to_string()),
        input.personal_info.personal_website.as_ref().unwrap_or(&"#".to_string()),
        input.personal_info.objective.as_ref().unwrap_or(&"".to_string()),
        input.experience.iter().map(|e| format_experience(e)).collect::<Vec<String>>().join("\n\n"),
        input.education.iter().map(|e| format_education(e)).collect::<Vec<String>>().join("\n\n"),
        input.skills.iter().map(|s| format!("\\item {} ({})", s.name, s.proficiency.as_ref().unwrap_or(&"N/A".to_string()))).collect::<Vec<String>>().join("\n"),
        input.languages.iter().map(|l| format!("\\item {} ({})", l.name, l.proficiency.as_ref().unwrap_or(&"N/A".to_string()))).collect::<Vec<String>>().join("\n"),
        input.references.iter().map(|r| format_reference(r)).collect::<Vec<String>>().join("\n\n")
    )
}

fn format_experience(e: &Experience) -> String {
    format!(
        "\\textbf{{{}}} at \\textit{{{}}}, {} \\
        {} - {} \\
        \\begin{{itemize}}
        {}
        \\end{{itemize}}",
        e.job_title,
        e.company,
        e.location,
        e.start_date,
        e.end_date.as_ref().unwrap_or(&"Present".to_string()),
        e.responsibilities.iter().map(|r| format!("\\item {}", r)).collect::<Vec<String>>().join("\n")
    )
}

fn format_education(e: &Education) -> String {
    format!(
        "\\textbf{{{}}} from \\textit{{{}}}, {} \\
        {} - {}",
        e.degree,
        e.institution,
        e.location,
        e.start_date,
        e.end_date
    )
}

fn format_reference(r: &Reference) -> String {
    format!(
        "\\textbf{{{}}} \\
        Relation: {} \\
        Email: {} \\
        Phone: {}",
        r.name,
        r.relation,
        r.email,
        r.phone.as_ref().unwrap_or(&"N/A".to_string())
    )
}

#[post("/generate", format = "json", data = "<CV_input>")]
fn generate_CV(CV_input: Json<CVInput>) -> String {
    let latex_code = generate_latex(&CV_input);
    // For now, we're just returning the LaTeX code.
    // Later, you can integrate a LaTeX compiler to return a PDF.
    latex_code
}

#[rocket::main]
async fn main() {
    rocket::build().mount("/", routes![generate_CV]).launch().await;
}
