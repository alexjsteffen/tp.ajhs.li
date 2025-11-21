use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="site">
            <div class="container">
                <div class="row">
                    <h3 class="site-wordmark">
                        { "ajhs legal innovation" }
                    </h3>
                    <nav role="navigation" aria-label="Primary">
                        <a href="#services">{ "Services" }</a>
                        <a href="#expertise">{ "Expertise" }</a>
                        <a href="#about">{ "About" }</a>
                        <a href="#contact">{ "Contact" }</a>
                        <a href="/login" class="login-link">{ "Login" }</a>
                    </nav>
                </div>
            </div>
        </header>
    }
}
