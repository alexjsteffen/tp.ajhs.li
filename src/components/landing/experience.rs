use yew::prelude::*;

#[function_component(Experience)]
pub fn experience() -> Html {
    html! {
        <section id="expertise" class="section">
            <div class="container">
                <h2 class="h2">{ "Experience Meets Innovation in Legal Services" }</h2>
                <div class="grid-2">
                    <div>
                        <p class="body">
                            { "With a deep understanding of both traditional legal frameworks and emerging technologies, I provide comprehensive legal support tailored to the unique needs of innovative businesses. My approach combines rigorous legal analysis with practical business insights, ensuring that your company not only complies with regulations but thrives in competitive markets." }
                        </p>
                    </div>
                    <div>
                        <p class="body">
                            { "From seed funding to Series A and beyond, I've guided startups through every stage of growth. My experience spans entity formation, intellectual property protection, contract negotiation, and strategic corporate governance, all designed to position your business for long-term success." }
                        </p>
                    </div>
                </div>
            </div>
        </section>
    }
}
