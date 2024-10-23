use leptos::*;
use leptos_dom::logging::console_log;
use std::str::FromStr;

use crate::interfaces::filter::FILTER;
use crate::interfaces::port::PortInfo;

#[component]
pub fn SearchBar(
    ports: ReadSignal<Vec<PortInfo>>,
    set_filter_ports: WriteSignal<Vec<PortInfo>>,
    // static: 클로저가 프로그램의 전체 수명 동안 유효하다는 것을 보장
    // Clone: 이벤트 핸들러를 여러 요소에 걸쳐 사용해야 할 경우가 있는데, 이 경우 핸들러를 복사해서 사용
    // 'static + Clone을 명시적으로 지정하여, 이벤트 핸들러로 사용되는 클로저가 컴포넌트의 라이프타임 동안 안전하게 사용될 수 있도록 보장
    clear_event: impl Fn() + 'static + Clone,
) -> impl IntoView {
    let (filter, set_filter) = create_signal(FILTER::PROCESS);
    let input_ref = create_node_ref::<html::Input>();

    let on_search_change = move |e| {
        let value = event_target_value(&e);
        // 필터링 로직 적용

        ports.get().iter().for_each(|port| {
            console_log(format!(
                "PROCESS NAME: {} PORT: {} PID: {}",
                &port.get_process_name(),
                &port.get_port(),         
                &port.get_pid()           
            ).as_str());
        });

        set_filter_ports.set(ports.get());
        set_filter_ports.update(|ports| {
            ports.retain(|port| match filter.get() {
                FILTER::PROCESS => port
                    .get_process_name()
                    .to_lowercase()
                    .contains(&value.to_lowercase()),
                FILTER::PORT => port.get_port().contains(&value),
                FILTER::PID => port.get_pid().contains(&value),
            });
        });
    };

    view! {
        <div class="flex flex-nowrap items-center bg-white overflow-hidden px-2 py-1 justify-between mx-auto shadow-gray-200 shadow-lg rounded-lg h-min">
            <input
                node_ref=input_ref
                class="text-base text-gray-500 flex-grow outline-none px-2 w-72"
                type="text"
                placeholder=""
                on:input=on_search_change
            />
            <div class="flex flex-nowrap items-center px-2 rounded-lg space-x-4 mx-auto h-full">
                <select
                    title="list"
                    class="text-base font-extrabold text-gray-800 outline-none border-2 px-4 py-2 rounded-lg h-8"
                    prop:value={move || filter.get().to_string()}
                    on:change=move |e| {
                        let value = event_target_value(&e);
                        if let Ok(selected_filter) = FILTER::from_str(&value) {
                            set_filter.set(selected_filter);
                        }
                    }
                >
                    <option value=FILTER::PROCESS.to_string()>{FILTER::PROCESS.to_string()}</option>
                    <option value=FILTER::PORT.to_string()>{FILTER::PORT.to_string()}</option>
                    <option value=FILTER::PID.to_string()>{FILTER::PID.to_string()}</option>
                </select>
                <button
                    class="bg-indigo-600 text-white rounded-lg px-4 py-2 hover:bg-indigo-500"
                    on:click=move |_e| {
                        clear_event();
                        if let Some(input) = input_ref.get() {
                            input.set_value("");
                        }
                    }
                >
                    <img src="/public/refresh.svg" width="20" height="20" alt="Icon" />
                </button>
            </div>
        </div>
    }
}
