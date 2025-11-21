use yew::prelude::*;

#[function_component(Hero)]
pub fn hero() -> Html {
    html! {
        <section class="hero section">
            <div class="container">
                <div class="grid-2">
                    <div class="hero-content">
                        <span class="eyebrow">{ "Legal Strategy for High-Growth Startups" }</span>
                        <h1 class="h1">{ "Scale Your Business with Confidence." }</h1>
                        <p class="lead">
                            { "I bridge the gap between law, technology, and finance to help founders navigate the complex path from formation to exit. As a specialist in startup growth and corporate structuring, I provide more than just legal answers—I deliver strategic solutions. Get the agility of a dedicated partner with the expertise of a senior counsel, all without the big-firm overhead." }
                        </p>
                        <div class="cta-group">
                            <a href="#contact" class="btn-primary">{ "Schedule a Free Consultation" }</a>
                            <a href="#expertise" class="btn-secondary">{ "View Practice Areas" }</a>
                        </div>
                    </div>
                    <div class="hero-decorative" aria-hidden="true">
                    </div>
                </div>
            </div>
        </section>
    }
}
