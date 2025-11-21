use yew::prelude::*;

#[function_component(FractionalGC)]
pub fn fractional_gc() -> Html {
    html! {
        <section id="about" class="section">
            <div class="container">
                <h2 class="h2">{ "Fractional General Counsel" }</h2>
                <div class="grid-2">
                    <div>
                        <p class="body">
                            { "As your on-demand legal advisor, Alexander seamlessly integrates with your team to provide high-level expertise tailored to your objectives. He delivers strategic legal advice, risk assessments, and ongoing guidance, empowering your organization to navigate complex legal landscapes and make informed decisions that drive success." }
                        </p>
                    </div>
                    <div>
                        <p class="body"><strong>{ "Key Benefits:" }</strong></p>
                        <ul class="benefits-list">
                            <li>{ "Strategic legal advice" }</li>
                            <li>{ "Proactive risk assessment" }</li>
                            <li>{ "Embedded, ongoing guidance" }</li>
                        </ul>
                        <p class="body mt-16">
                            <a href="#contact" class="cta-link">
                                { "See how fractional GC works" }
                            </a>
                        </p>
                    </div>
                </div>
            </div>
        </section>
    }
}
