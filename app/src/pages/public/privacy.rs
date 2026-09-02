use leptos::prelude::*;

#[component]
pub fn Privacy() -> impl IntoView {
    view! {
        <section class="full privacy">
            <div>
                <h1>"Privacy Policy"</h1>
                <p>"This website uses Google Sign-In only for admin access to private content."</p>
                <p>
                    "When you sign in with Google, Google provides me with your email address. I store that email address in my database to identify your account and to decide whether you are allowed to access the admin area."
                </p>
                <p>
                    "I do not use your Google account for marketing, advertising, analytics, or any other purpose. I do not sell your data, and I do not share it with third parties."
                </p>
                <p>
                    "The only personal information I intentionally keep for login is your email address. I do not store additional profile data from Google, and I do not use the login feature for anything beyond authentication and authorization for the site."
                </p>
                <p>
                    "If you want to contact me about your account or data, please use the "
                    <a href="/contact">"contact page"</a>" or email me directly."
                </p>
            </div>
        </section>
    }
}
