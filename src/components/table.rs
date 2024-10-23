// components/port_table.rs
use leptos::*;

use crate::interfaces::port::PortInfo;

#[component]
pub fn PortTable(
    props: ReadSignal<Vec<PortInfo>>,
    delete_cb: impl Fn(String) + 'static + Clone,
) -> impl IntoView {
    view! {
        <div class="overflow-hidden ">
        <table class=" min-w-full rounded-xl">
            <thead>
                <tr class="bg-gray-50">
                    <th scope="col" class="p-5 text-left text-sm leading-6 font-semibold text-gray-900 capitalize rounded-t-xl"> PROCESS </th>
                    <th scope="col" class="p-5 text-left text-sm leading-6 font-semibold text-gray-900 capitalize"> PORT </th>
                    <th scope="col" class="p-5 text-left text-sm leading-6 font-semibold text-gray-900 capitalize"> PID </th>
                    <th scope="col" class="p-5 text-left text-sm leading-6 font-semibold text-gray-900 capitalize"> Kill </th>
                </tr>
            </thead>
            <tbody class="divide-y divide-gray-300 ">
            <For
                each=move || props.get()
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
