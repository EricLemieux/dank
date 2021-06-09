use handlebars::Handlebars;
use std::collections::HashMap;

/// Generate an html page that contains all of the downloaded images.
pub fn generate_html(images: Vec<String>) -> String {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string("html", include_str!("templates/index.hbs"))
        .unwrap();

    let mut template_data = HashMap::new();
    template_data.insert("images", images);

    handlebars.render("html", &template_data).unwrap()
}
