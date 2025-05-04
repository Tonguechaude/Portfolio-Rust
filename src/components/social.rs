use leptos::prelude::*;

#[component]
pub fn SocialLinks() -> impl IntoView {
    let links = vec![
        ("GitHub", "https://github.com/Tonguechaude", "🐙"),
        (
            "LinkedIn",
            "https://www.linkedin.com/in/evan-c-29805a276/",
            "💼",
        ),
        ("Mastodon", "https://mastodon.social/@tonguechaude", ""),
    ];

    view! {
        <div class="flex gap-4">
            <For
                each=move || links.clone()
                key=|link| link.0.to_string()
                children=move |link| {
                    view! {
                        <a
                            href={link.1}
                            target="_blank"
                            class="flex items-center gap-1 text-indigo-600 hover:underline dark:text-indigo-400"
                        >
                            <span>{link.2}</span> <span>{link.0}</span>
                        </a>
                    }
                }
            />
        </div>
    }
}
