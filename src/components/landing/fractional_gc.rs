use yew::prelude::*;

#[function_component(FractionalGC)]
pub fn fractional_gc() -> Html {
    html! {
        <section id="about" class="section">
            <div class="container">
                <h2 class="h2">{ "Your In-House Counsel, On Demand" }</h2>
                <div class="grid-2">
                    <div>
                        <p class="body">
                            { "Hiring a full-time General Counsel is expensive, but going without one is risky. My Fractional GC service bridges that gap. I integrate seamlessly with your team to provide day-to-day guidance, contract review, and risk management. It's the proactive legal support of a dedicated executive, tailored to your budget and growth stage." }
                        </p>
                    </div>
                    <div>
                        <p class="body"><strong>{ "Why Fractional GC?" }</strong></p>
                        <ul class="benefits-list">
                            <li>{ "Predictable costs, no hourly billing surprises" }</li>
                            <li>{ "Business-context aware advice" }</li>
                            <li>{ "Faster turnaround on day-to-day contracts" }</li>
                        </ul>
                        <p class="body mt-16">
                            <a href="#contact" class="cta-link">
                                { "Explore Fractional GC Plans" }
                            </a>
                        </p>
                    </div>
                </div>
            </div>
        </section>
    }
}
