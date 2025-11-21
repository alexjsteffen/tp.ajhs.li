use yew::prelude::*;

#[function_component(Hero)]
pub fn hero() -> Html {
    html! {
        <section class="hero section">
            <div class="container">
                <div class="grid-2">
                    <div class="hero-content">
                        <span class="eyebrow">{ "Your Trusted Legal Partner in Innovation" }</span>
                        <h1 class="h1">{ "Building Success Through Strategic Legal Guidance" }</h1>
                        <p class="lead">
                            { "At ajhs legal innovation, I specialize at the intersection of law, technology, and finance, delivering impactful legal solutions that propel startups forward. With over five years of dedicated experience working alongside emerging companies, I bring a unique blend of expertise in financial strategy, corporate structuring, and offshore account management. I have successfully assisted entrepreneurs secure critical funding, optimize their corporate frameworks, and navigate complex financial landscapes. My innovative, tailored legal services empower innovators to overcome challenges and seize opportunities in today's fast-paced business environment." }
                        </p>
                        <div class="cta-group">
                            <a href="#contact" class="btn-primary">{ "Book a Consultation" }</a>
                            <a href="#expertise" class="btn-secondary">{ "Learn More" }</a>
                        </div>
                    </div>
                    <div class="hero-decorative" aria-hidden="true">
                    </div>
                </div>
            </div>
        </section>
    }
}
