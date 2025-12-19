use pulldown_cmark::{html, Parser};
use ammonia::clean;

pub fn markdown_to_html(markdown: &str) -> String {
    // Parse markdown using pulldown-cmark
    let parser = Parser::new(markdown);

    // Convert to HTML
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // Sanitize HTML to prevent XSS attacks
    clean(&html_output)
}
