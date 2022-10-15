use genpdf::elements;
use genpdf::style;
use genpdf::Alignment;
use genpdf::Document;
use genpdf::Element;
use yaml_rust::Yaml;

pub fn add_line_break(mut pdf: Document) -> Document {
    pdf.push(elements::Break::new(1));
    pdf
}

pub fn add_cv_header(mut pdf: Document, title: &str) -> Document {
    pdf.push(
        elements::Paragraph::new(title)
            .aligned(Alignment::Center)
            .styled(
                style::Style::new()
                    .bold()
                    .with_font_size(22)
                    .with_color(style::Color::Rgb(166, 166, 166)),
            ),
    );
    pdf
}

pub fn add_section(mut pdf: Document, title: &str) -> Document {
    pdf.push(
        elements::Paragraph::new(title)
            .styled(style::Style::new().bold().italic().with_font_size(14)),
    );
    pdf
}

pub fn add_contact_details(mut pdf: Document, title: &str, details: &str) -> Document {
    pdf.push(
        elements::Paragraph::default()
            .styled_string(title, style::Style::new().bold())
            .string(details)
            .styled(style::Style::new().with_font_size(11)),
    );
    pdf
}

pub fn add_skills(mut pdf: Document, skills: &Vec<Yaml>) -> Document {
    for skill in skills {
        pdf.push(elements::BulletPoint::new(
            elements::Paragraph::default()
                .styled_string(skill["name"].as_str().unwrap(), style::Style::new().bold())
                .string(format!(" - {}", skill["description"].as_str().unwrap()))
                .styled(style::Style::new().with_font_size(11)),
        ));
    }
    pdf
}

pub fn add_work_experience(mut pdf: Document, jobs: &Vec<Yaml>) -> Document {
    for job in jobs {
        pdf.push(
            elements::Paragraph::default()
                .styled_string(
                    format!(
                        "{}-{}: ",
                        job["startDate"].as_str().unwrap(),
                        job["endDate"].as_str().unwrap()
                    ),
                    style::Style::new().bold(),
                )
                .string(format!(
                    "{} - {}",
                    job["position"].as_str().unwrap(),
                    job["name"].as_str().unwrap()
                ))
                .styled(style::Style::new().with_font_size(14)),
        );
        match job["highlights"].as_vec() {
            Some(bullet_points) => {
                for point in bullet_points {
                    pdf.push(
                        elements::BulletPoint::new(elements::Paragraph::new(
                            point.as_str().unwrap(),
                        ))
                        .styled(style::Style::new().with_font_size(11)),
                    );
                }
                pdf = add_line_break(pdf);
            }
            None => {
                pdf = add_line_break(pdf);
            }
        }
    }
    pdf
}

pub fn add_education(mut pdf: Document, institutions: &Vec<Yaml>) -> Document {
    for institution in institutions {
        pdf.push(
            elements::Paragraph::default()
                .styled_string(
                    format!(
                        "{}-{}: {}:",
                        institution["startDate"].as_str().unwrap(),
                        institution["endDate"].as_str().unwrap(),
                        institution["area"].as_str().unwrap()
                    ),
                    style::Style::new().bold(),
                )
                .string(format!(" {}", institution["institution"].as_str().unwrap()))
                .styled(style::Style::new().with_font_size(11)),
        );
    }
    pdf
}

pub fn add_languages(mut pdf: Document, languages: &Vec<Yaml>) -> Document {
    for language in languages {
        pdf.push(elements::BulletPoint::new(
            elements::Paragraph::default()
                .styled_string(
                    language["language"].as_str().unwrap(),
                    style::Style::new().bold(),
                )
                .string(" - ")
                .string(language["fluency"].as_str().unwrap())
                .styled(style::Style::new().with_font_size(11)),
        ));
    }
    pdf
}
