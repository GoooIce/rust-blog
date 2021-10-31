mod error_pages;
mod pages;

use perseus::define_app;

define_app! {
    templates: [
        crate::pages::index::get_template::<G>(),
        crate::pages::about::get_template::<G>()
    ],
    error_pages: crate::error_pages::get_error_pages(),
    static_aliases: {
        "/test.txt" => "static/test.txt"
    }
}
