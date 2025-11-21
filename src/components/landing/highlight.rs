use yew::prelude::*;

#[function_component(Highlight)]
pub fn highlight() -> Html {
    html! {
        <section class="section">
            <div class="container">
                <div class="highlight-band">
                    { "Big law expertise. Startup agility. Transparent pricing. I handle the legal complexities so you can focus on building the future." }
                </div>
            </div>
        </section>
    }
}
