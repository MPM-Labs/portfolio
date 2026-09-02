use leptos::prelude::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer>
            <section>
                <a href="https://www.linkedin.com/in/jonas-baugerud/">
                    <img src="linkedin_logo.svg" />
                </a>
                <a href="https://github.com/MrProgrammerMan">
                    <img src="github_logo.svg" />
                </a>
            </section>
            <section>
                <p>"©2026"</p>
                <p>"Jonas Hazeland Baugerud"</p>
            </section>
            <section>
                <a href="/privacy">"Privacy Policy"</a>
                <a href="#">
                    <img src="arrow_logo.svg" />
                </a>
            </section>
        </footer>
    }
}
