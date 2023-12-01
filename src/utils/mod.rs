mod color;
mod fs;
mod http;
mod run;

pub use color::{
    print_err, print_fail, print_filename, print_link, print_out, print_write_filename,
};
pub use fs::{
    create_folder_if_not_exists, is_json_content_same, read_previous_json_file, write_to_json_file,
};
pub use http::{get_province, send_http_request};
pub use run::start;
