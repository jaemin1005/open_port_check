use leptos::*;
use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;

use crate::components::loading::Loading;
use crate::components::search::SearchBar;
use crate::components::table::PortTable;
use crate::interfaces::kill::KillArgs;
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

    let (loading, set_loading) = create_signal(false);

    let fetch_ports_async = move || async move {
        set_loading.set(true);

        let args = JsValue::NULL;
        let response = invoke("get_open_ports", args).await;

        match from_value::<Vec<PortInfo>>(response) {
            Ok(results) => {
                set_ports.set(Vec::<PortInfo>::new());
                set_filter_ports.set(Vec::<PortInfo>::new());
                set_ports.set(results.clone());
                set_filter_ports.set(results.clone());
            }
            Err(_) => set_ports.set(Vec::<PortInfo>::new()),
        }

        set_loading.set(false);
    };

    create_effect(move |_| {
        spawn_local(fetch_ports_async());
    });

    let clear_event_cb = move || {
        set_filter_ports.set(ports.get());
        spawn_local(fetch_ports_async());
    };

    let delet_event_cb = move |pid: String| {
        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&KillArgs { pid: &pid }).unwrap();

            let kill = invoke("kill_process", args)
                .await
                .as_bool()
                .unwrap_or_else(|| false);
            if kill {
                fetch_ports_async().await;
            }
        });
    };

    view! {
        <div>
            <div class="w-screen fixed z-50 bg-white">
                <SearchBar ports=ports set_filter_ports=set_filter_ports clear_event=clear_event_cb/>
            </div>
            <div class="pt-10">
                <Show when=move || loading.get() == false fallback=|| view! {<Loading/>}>
                    <PortTable props=filter_ports delete_cb=delet_event_cb/>
                </Show>
            </div>
        </div>
    }
}
