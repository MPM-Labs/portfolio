use leptos::prelude::*;

#[component]
pub fn About() -> impl IntoView {
    view! {
        <span class="line">
            <p>"01"</p>
        </span>
        <section id="about">
            <div>
                <img
                    src="nixcon.avif"
                    alt="Me and some friends at NixCon, meeting Eelco Dolstra."
                />
            </div>
            <div>
                <h1>"About"</h1>
                <ul>
                    <li>"Name: Jonas Hazeland Baugerud"</li>
                    <li>"Student at Software Engineering, Oslomet"</li>
                    <li>"Born: 03/07/2004"</li>
                    <li>"Study location: Pilestredet, Oslo, Norway"</li>
                </ul>
            </div>
        </section>
        <span class="line">
            <p>"02"</p>
        </span>
        <section class="full" id="rel-ex">
            <h2>"Relevant experience"</h2>
            <ul>
                <span class="arrow">
                    <span></span>
                    <span></span>
                    <span></span>
                </span>
                <li>
                    <p>"August 2026 - Today"</p>
                    <p>"Functional Programming at UiO"</p>
                </li>
                <li>
                    <p>"August 2026 - Today"</p>
                    <p>"TA in algorithms and data structures"</p>
                </li>
                <li>
                    <p>"June 2026 - August 2026"</p>
                    <p>
                        <a href="https://github.com/The-quantum-6/Metronomicon" target="_blank">
                            "The Metronomicon project"
                        </a>
                    </p>
                </li>
                <li>
                    <p>"January 2026 - June 2026"</p>
                    <p>"TA in web programming"</p>
                </li>
                <li>
                    <p>"August 2024 - Today"</p>
                    <p>"Software Engineering at OsloMet"</p>
                </li>
                <li>
                    <p>"August 2023 - June 2024"</p>
                    <p>"Software Engineering at NTNU"</p>
                </li>
                <li>
                    <p>"August 2021 - June 2023"</p>
                    <p>"Computer science 1 & 2 (upper secondary school) Focus on web and Python"</p>
                </li>
                <li>
                    <p>"July 2004 - Today"</p>
                    <p>"A love for technology and a passion for knowing more."</p>
                </li>
            </ul>
        </section>
        <span class="line">
            <p>"03"</p>
        </span>
        <section id="irrel-ex">
            <div>
                <h2>"Irrelevant Experience"</h2>
                <p>
                    "If you made it this far and you're still reading, I figure you may want to know more about me as a person."
                </p>
                <p>
                    "I do genuinely have a lot of passion for technology and programming, and spend a lot of time learning about technologies and tools that aren't taught in university. When I'm not doing that, I enjoy being outdoors, reading or playing piano."
                </p>
                <p>
                    "I grew up in Elverum, outside Hamar. Currently, I live in Ås with my partner. I much prefer having a little more peace and quiet than if we were to live in central Oslo. The train is only 15 minutes anyways, so it doesn't make a big difference travel-wise."
                </p>
                <p>
                    "I'm interested in coffee, in a \"wine-snob\" sort of way. It's basically the same thing. Just as people obsess over the origins and terroir of wine, people do with coffee too. (You might not know that coffee is actually a fresh product! It should be bought whole-bean and list the roasting date(not the best-before date) on the bag.)"
                </p>
            </div>
            <div>
                <img src="coffee.avif" alt="Me tasting coffee." />
            </div>
        </section>
    }
}
