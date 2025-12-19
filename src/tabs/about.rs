use leptos::prelude::*;
use stylers::style;

#[component]
pub fn About() -> impl IntoView {
    let styler_class = style! { "About",

    };

    view! {
        <div class = styler_class>
            <div>"TODO: about section"</div>
        </div>
    }
}
