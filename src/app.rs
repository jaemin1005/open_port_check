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
    let (filter_ports, set_filter_ports) = create_signal(Vec::<PortInfo>::new());

    let (loading, set_loading) = create_signal(false);

    let fetch_ports = create_resource(
        || (), // 의존성이 없으므로 빈 튜플 사용
        move |_| async move {
            set_loading.set(true);

            let args = JsValue::NULL;
            let response = invoke("get_open_ports", args).await;

            match from_value::<Vec<PortInfo>>(response) {
                Ok(results) => {
                    set_loading.set(false);
                    set_filter_ports.set(results.clone());
                    return results;
                },
                Err(_) => {
                    set_loading.set(false);
                    set_filter_ports.set(Vec::<PortInfo>::new());
                    return Vec::<PortInfo>::new();
                }
            }
        },
    );

    let clear_event_cb = move || {
        fetch_ports.refetch();
    };

    let delet_event_cb = move |pid: String| {
        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&KillArgs { pid: &pid }).unwrap();

            let kill = invoke("kill_process", args)
                .await
                .as_bool()
                .unwrap_or_else(|| false);
            if kill {
                fetch_ports.refetch();
            }
        });
    };

    view! {
        <div>
            <div class="w-screen fixed z-50 bg-white">
                <SearchBar ports=fetch_ports.get().unwrap_or_default() set_filter_ports=set_filter_ports clear_event=clear_event_cb/>
            </div>
            <div class="pt-10">
                <Show when=move || loading.get() == false fallback=|| view! {<Loading/>}>
                    <PortTable props=filter_ports delete_cb=delet_event_cb/>
                </Show>
            </div>
        </div>
    }
}
