use std::str::FromStr;

use ev::MouseEvent;
// components/port_table.rs
use crate::interfaces::{filter::FILTER, port::PortInfo, sort::SORT};
use leptos::*;
use leptos_dom::logging::console_error;
use wasm_bindgen::JsCast;

#[component]
pub fn PortTable(
    props: ReadSignal<Vec<PortInfo>>,
    delete_cb: impl Fn(String) + 'static + Clone,
) -> impl IntoView {
    let (filter, set_filter) = create_signal(FILTER::PROCESS);
    let (select_sort, set_select_sort) = create_signal(SORT::NONE);

    let on_header_click = move |e: MouseEvent| {
        if let Some(target) = e.target() {
            // 대상 요소를 HtmlElement로 변환
            // 요소의 텍스트 내용(innerText)을 가져옴
            if let Some(element) = target.dyn_ref::<web_sys::HtmlElement>() {
                let header_text = element.inner_text();

                let event_value = match FILTER::from_str(&header_text) {
                    Ok(ok) => ok,
                    Err(_) => {
                        console_error("Failed to parse FILTER value");
                        return;
                    }
                };

                if filter.get() == event_value {
                    set_select_sort.update(|option| {
                        *option = match *option {
                            SORT::NONE => SORT::ASC,
                            SORT::ASC => SORT::DESC,
                            SORT::DESC => SORT::NONE,
                        };
                    });
                } else {
                    set_filter.set(event_value);
                    set_select_sort.set(SORT::ASC);
                }
            }
        }
    };

    // create_memo를 이용해 계산된 값을 뷰를 이용하는데 그린다.
    // 현재 상태에 따라 값 갱신
    let sort_and_filter_ports = create_memo(move |_| {
        let mut filtered_ports = props.get();
        let current_filter = filter.get();
        let current_sort = select_sort.get();

        match current_filter {
            FILTER::PROCESS => match current_sort {
                SORT::ASC => {
                    filtered_ports.sort_by(|a, b| a.get_process_name().cmp(&b.get_process_name()));
                }
                SORT::DESC => {
                    filtered_ports.sort_by(|a, b| b.get_process_name().cmp(&a.get_process_name()));
                }
                SORT::NONE => {}
            },
            FILTER::PORT => match current_sort {
                SORT::ASC => {
                    filtered_ports
                        .sort_by(|a, b| a.get_port_as_usize().cmp(&b.get_port_as_usize()));
                }
                SORT::DESC => {
                    filtered_ports
                        .sort_by(|a, b| b.get_port_as_usize().cmp(&a.get_port_as_usize()));
                }
                SORT::NONE => {}
            },
            FILTER::PID => match current_sort {
                SORT::ASC => {
                    filtered_ports.sort_by(|a, b| a.get_pid_as_usize().cmp(&b.get_pid_as_usize()));
                }
                SORT::DESC => {
                    filtered_ports.sort_by(|a, b| b.get_pid_as_usize().cmp(&a.get_pid_as_usize()));
                }
                SORT::NONE => {}
            },
        }

        filtered_ports
    });

    view! {
        <div class="overflow-hidden ">
        <table class=" min-w-full rounded-xl">
            <thead>
                <tr class="bg-gray-50 select-none">
                    <th scope="col" class="p-5 text-left text-sm leading-6 font-semibold text-gray-900 capitalize rounded-t-xl cursor-pointer" on:click=on_header_click>{FILTER::PROCESS.to_string()}</th>
                    <th scope="col" class="p-5 text-left text-sm leading-6 font-semibold text-gray-900 capitalize cursor-pointer" on:click=on_header_click>{FILTER::PORT.to_string()}</th>
                    <th scope="col" class="p-5 text-left text-sm leading-6 font-semibold text-gray-900 capitalize cursor-pointer" on:click=on_header_click>{FILTER::PID.to_string()}</th>
                    <th scope="col" class="p-5 text-left text-sm leading-6 font-semibold text-gray-900 capitalize"> Kill </th>
                </tr>
            </thead>
            <tbody class="divide-y divide-gray-300 ">
            <For
                each=move || sort_and_filter_ports.get()
                key=|prop| format!("{}_{}", prop.get_pid(), prop.get_port())
                children=move |port_info: PortInfo| {
                        let delete_cb = delete_cb.clone();

                        view!
                        {
                            <tr class="bg-white transition-all duration-500 hover:bg-gray-100">
                                <td class="p-5 whitespace-nowrap text-sm leading-6 font-medium text-gray-900 ">{port_info.get_process_name()}</td>
                                <td class="p-5 whitespace-nowrap text-sm leading-6 font-medium text-gray-900">{port_info.get_port()}</td>
                                <td class="p-5 whitespace-nowrap text-sm leading-6 font-medium text-gray-900">{port_info.get_pid()}</td>
                                <td class=" p-5 ">
                                    <button class="p-2 rounded-full  group transition-all duration-500  flex item-center hover:bg-gray-600" on:click= move |_e| {
                                        delete_cb(port_info.get_pid());
                                    }>
                                        <img src="/public/trash.svg" width="20" height="20" alt="Icon" />
                                    </button>
                                </td>
                            </tr>
                        }
                    }
                />
            </tbody>
        </table>
    </div>
    }
}
