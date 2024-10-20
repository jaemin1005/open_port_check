use leptos::*;
use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;

use crate::components::search::SearchBar;
use crate::components::table::PortTable;
use crate::interfaces::port::PortInfo;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn App() -> impl IntoView {
    let (ports, set_ports) = create_signal(Vec::<PortInfo>::new());

    let (filter_ports, set_filter_ports) = create_signal(Vec::<PortInfo>::new());

    create_effect(move |_| {
        spawn_local(async move {
            // Tauri 명령 호출
            let args = JsValue::NULL;
            let response = invoke("get_open_ports", args).await;

            match from_value::<Vec<PortInfo>>(response) {
                Ok(ports) => {
                    set_ports.set(ports.clone());
                    set_filter_ports.set(ports.clone())
                }
                Err(_) => set_ports.set(Vec::<PortInfo>::new()),
            }
        });
    });

    let clear_event_cb = move || {
        set_filter_ports.set(ports.get());
    };

    view! {
        <div>
            <div class="w-screen fixed z-50 bg-white">
                <SearchBar ports=ports set_filter_ports=set_filter_ports clear_event=clear_event_cb/>
            </div>
            <div class="pt-10">
                <PortTable props=filter_ports/>
            </div>
        </div>
    }
}
