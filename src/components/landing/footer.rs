use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <footer id="contact" class="site">
            <div class="container">
                <div class="footer-grid">
                    <div class="footer-column">
                        <h4>{ "Practice Areas" }</h4>
                        <ul>
                            <li><a href="#services">{ "Startup Business Assistance" }</a></li>
                            <li><a href="#services">{ "Corporate Matters" }</a></li>
                            <li><a href="#about">{ "Fractional General Counsel" }</a></li>
                            <li><a href="#expertise">{ "Legal Innovation" }</a></li>
                        </ul>
                    </div>
                    <div class="footer-column">
                        <h4>{ "Company" }</h4>
                        <ul>
                            <li><a href="#expertise">{ "About Us" }</a></li>
                            <li><a href="#contact">{ "Contact" }</a></li>
                            <li><a href="https://ajhs.li">{ "ajhs.li" }</a></li>
                            <li><a href="https://hardysteffen.com">{ "Hardy Steffen" }</a></li>
                        </ul>
                    </div>
                    <div class="footer-column">
                        <h4>{ "Contact" }</h4>
                        <ul>
                            <li><a href="mailto:contact@ajhs.li">{ "contact@ajhs.li" }</a></li>
                            <li>{ "ajhs legal innovation" }</li>
                            <li>{ "Your Legal Partner in Innovation" }</li>
                        </ul>
                    </div>
                </div>
                <div class="footer-bottom">
                    <p>
                        { "© 2024 ajhs Legal Innovation | " }
                        <a href="/privacy">{ "Privacy" }</a>
                        { " | " }
                        <a href="/terms">{ "Terms" }</a>
                    </p>
                </div>
            </div>
        </footer>
    }
}
