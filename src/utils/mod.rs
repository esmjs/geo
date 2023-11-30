mod color;
mod fs;
mod http;
mod run;

pub use color::{print_err, print_fail, print_link};
pub use fs::{is_json_content_same, read_previous_json_file, write_to_json_file};
pub use http::{get_province, send_http_request};
pub use run::start;
