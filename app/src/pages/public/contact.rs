use leptos::prelude::*;

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <section>
            <div>
                <img src="outdoors.avif" alt="Me standing outdoors in a mountainous area." />
            </div>
            <div>
                <h1>"Contact"</h1>
                <ul>
                    <li>"Name: Jonas Hazeland Baugerud"</li>
                    <li>
                        "E-mail: "
                        <a href="mailto:jonas.baugerud@gmail.com">"jonas.baugerud@gmail.com"</a>
                    </li>
                    <li>
                        "OsloMet e-mail: "
                        <a href="mailto:jobau8311@oslomet.no">"jobau8311@oslomet.no"</a>
                    </li>
                    <li>"UiO e-mail: "<a href="mailto:jonashba@uio.no">"jonashba@uio.no"</a></li>
                    <li>
                        "You can also reach out to me through my LinkedIn profile, found "
                        <a
                            href="https://www.linkedin.com/in/jonas-baugerud/"
                            aria-label="My linkedin profile"
                        >
                            "here"
                        </a>" or at the bottom of the page."
                    </li>
                </ul>
            </div>
        </section>
    }
}
