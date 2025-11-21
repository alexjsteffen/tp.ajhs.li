use yew::prelude::*;

#[function_component(Highlight)]
pub fn highlight() -> Html {
    html! {
        <section class="section">
            <div class="container">
                <div class="highlight-band">
                    { "I offer tailored legal solutions that empower startups and businesses to navigate complex legal landscapes with confidence." }
                </div>
            </div>
        </section>
    }
}
