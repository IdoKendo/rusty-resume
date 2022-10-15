extern crate yaml_rust;
mod file_handler;
mod pdf_handler;
use file_handler::get_file_name;
use file_handler::parse_file;
use genpdf::fonts;
use genpdf::Document;
use genpdf::SimplePageDecorator;
use pdf_handler::add_contact_details;
use pdf_handler::add_cv_header;
use pdf_handler::add_education;
use pdf_handler::add_languages;
use pdf_handler::add_line_break;
use pdf_handler::add_section;
use pdf_handler::add_skills;
use pdf_handler::add_work_experience;
use std::env;
use yaml_rust::Yaml;

const DEFAULT_FONT_DIR: &str = "./fonts";
const DEFAULT_FONT_NAME: &str = "Cambria";

fn get_title(doc: &Yaml) -> String {
    format!(
        "{} - {}",
        doc["basics"]["name"].as_str().unwrap(),
        doc["basics"]["title"].as_str().unwrap()
    )
}

fn generate_pdf(doc: &Yaml, font_family: fonts::FontFamily<fonts::FontData>) -> Document {
    let mut pdf = Document::new(font_family);
    let mut decorator = SimplePageDecorator::new();
    let title = get_title(doc);
    decorator.set_margins(10);
    pdf.set_title(&title);
    pdf.set_page_decorator(decorator);
    pdf = add_cv_header(pdf, &title);
    pdf = add_line_break(pdf);
    pdf = add_section(pdf, "Contact Details");
    pdf = add_contact_details(pdf, "Phone: ", doc["basics"]["phone"].as_str().unwrap());
    pdf = add_contact_details(pdf, "Mail: ", doc["basics"]["email"].as_str().unwrap());
    pdf = add_line_break(pdf);
    pdf = add_section(pdf, "Skills");
    pdf = add_skills(pdf, doc["skills"].as_vec().unwrap());
    pdf = add_line_break(pdf);
    pdf = add_section(pdf, "Professional Experience");
    pdf = add_work_experience(pdf, doc["work"].as_vec().unwrap());
    pdf = add_section(pdf, "Education");
    pdf = add_education(pdf, doc["education"].as_vec().unwrap());
    pdf = add_line_break(pdf);
    pdf = add_section(pdf, "Language");
    pdf = add_languages(pdf, doc["languages"].as_vec().unwrap());
    pdf
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match get_file_name(&args) {
        Ok(file_name) => {
            let docs = parse_file(&file_name);
            let doc = &docs[0];
            let name = doc["basics"]["name"].as_str().unwrap();
            println!("Hello {}, generating your resume now!", name);
            let font_family = fonts::from_files(DEFAULT_FONT_DIR, DEFAULT_FONT_NAME, None)
                .expect("Failed to load font family");
            let pdf = generate_pdf(doc, font_family);
            pdf.render_to_file(format!("{} CV - English.pdf", name))
                .expect("Failed to write PDF file");
        }
        Err(err) => {
            print!("Error! {}", err);
        }
    }
}
