use leptos::prelude::*;
use stylers::style;

#[component]
pub fn Header() -> impl IntoView {
    let styler_class = style! { "Header",
        .header {
            background: #0000AA;
            color: #FFFFFF;
            padding: 0.75rem 0;
            text-align: center;
            font-weight: bold;
            border-bottom: 0.125rem solid #FFFFFF;
            font-size: 1.125rem;
        }

        @media (max-width: 768px) {
            .header {
                padding: 0.5rem 0;
                font-size: 1rem;
            }
        }
    };

    view! { class = styler_class,
      <div class="header">
        "WYATT AVILLA"
      </div>
    }
}
