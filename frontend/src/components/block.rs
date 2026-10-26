use leptos::prelude::*;

const BLOCK_CLASSES: &str = "
    mx-auto w-full sm:w-5/8
    space-y-4 break-words text-sm leading-7

    bg-[#AF76D4] px-4 py-2
    shadow-[8px_8px_0px_0px_rgba(0,0,0,0.25)]

    [&_h1]:text-3xl
    [&_h2]:text-2xl
    [&_h3]:text-xl

    [&_a]:underline
    [&_ul]:list-disc [&_ul]:pl-2
    [&_ol]:list-decimal [&_ol]:pl-2

    [&_blockquote]:bg-[#66AAB2]
    [&_blockquote]:px-4 [&_blockquote]:py-2
    [&_blockquote]:shadow-[8px_8px_0px_0px_rgba(0,0,0,0.25)]

    [&_pre]:overflow-x-auto
    [&_pre]:bg-[#4382A3]
    [&_pre]:px-4 [&_pre]:py-2
    [&_pre]:shadow-[8px_8px_0px_0px_rgba(0,0,0,0.25)]

    [&_img]:max-w-full

    [&_.table-wrapper]:w-full
    [&_.table-wrapper]:overflow-x-auto
    [&_.table-wrapper]:bg-[#66AAB2]
    [&_.table-wrapper]:px-4 [&_.table-wrapper]:py-2
    [&_.table-wrapper]:shadow-[8px_8px_0px_0px_rgba(0,0,0,0.25)]

    [&_table]:w-full
    [&_td]:border [&_td]:px-2 [&_td]:py-1
    [&_th]:border [&_th]:px-2 [&_th]:py-1

    [&_.inline-code]:bg-[rgba(0,0,0,0.25)]
    [&_.inline-code]:px-2 [&_.inline-code]:py-1
";

#[component]
pub fn Block(children: Children) -> impl IntoView {
    return view! {
        <div class="w-full px-4 pb-4 sm:px-0">
            <div class=BLOCK_CLASSES>
                {children()}
            </div>
        </div>
    };
}
