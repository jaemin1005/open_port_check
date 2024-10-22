mod app;

use app::*;
use leptos::*;

pub mod components {
    pub mod table;
    pub mod search;
}

pub mod interfaces {
    pub mod port;
    pub mod filter;
    pub mod kill;
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <App/>
        }
    })
}
