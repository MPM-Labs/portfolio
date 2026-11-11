use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <section class="reverse">
            <div>
                <img src="portrait.avif" alt="Portrait of me, Jonas Hazeland Baugerud." />
            </div>
            <div>
                <h1>"Hello, I'm Jonas"</h1>
                <p>
                    "I study "
                    <a href="https://www.oslomet.no/studier/tkd/dataingenior">
                        "Software Engineering at OsloMet"
                    </a>
                    ", a course focusing on programming, mathematics, design philosophy and collaboration. Currently, I study full time, but I am always interested in job opportunities on the side."
                </p>
                <p>
                    "This is my portfolio. Here, you can see some projects I have completed in order to see how I work. If you want to see what I'm doing right now, you should check out "
                    <a href="https://github.com/MrProgrammerMan" target="_blank">
                        "my GitHub profile"
                    </a>" / "<a href="https://github.com/MPM-Labs" target="_blank">
                        "my GitHub project home"
                    </a>
                    " instead. You can also find information about me as a person, and get in touch with me if you want. I recommend you start in the "
                    <A href="portfolio">"portfolio"</A>"."
                </p>
            </div>
        </section>
    }
}
