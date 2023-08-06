use pulldown_cmark::{html, Parser};
use rocket_dyn_templates::tera::*;
use std::collections::HashMap;

pub fn markdown(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    let text = try_get_value!("markdown", "value", String, value);

    let parser = Parser::new(&text);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    Ok(to_value(html_output)?)
}
