use pulldown_cmark::{html, Parser};
use std::collections::HashMap;
use tera::{to_value, try_get_value, Result, Value};

pub fn markdown(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    let text = try_get_value!("markdown", "value", String, value);

    let parser = Parser::new(&text);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    Ok(to_value(html_output)?)
}
