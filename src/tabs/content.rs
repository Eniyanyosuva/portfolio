use crate::tabs::Tab;
use leptos::prelude::*;
use stylers::style;

#[component]
pub fn TabContent(active_tab: RwSignal<&'static Tab>) -> impl IntoView {
    let styler_class = style! { "TabContent",
        .content {
            flex: 1;
            padding: 16px;
            overflow-y: auto;
            background: #AAAAAA;
            color: #000000;
        }
    };

    view! { class = styler_class,
      <div class="content">
          {move || active_tab.get().content() }
      </div>
    }
}
