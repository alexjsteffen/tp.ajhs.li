use yew::prelude::*;

#[function_component(Experience)]
pub fn experience() -> Html {
    html! {
        <section id="expertise" class="section">
            <div class="container">
                <h2 class="h2">{ "A Strategic Partner, Not Just a Lawyer" }</h2>
                <div class="grid-2">
                    <div>
                        <p class="body">
                            { "Startups move fast, and traditional legal models often can't keep up. My practice is built for speed and precision. With over five years of experience guiding emerging companies, I understand that legal advice needs to facilitate business goals, not block them. I combine rigorous legal analysis with practical financial strategy to help you take calculated risks." }
                        </p>
                    </div>
                    <div>
                        <p class="body">
                            { "When you work with me, you work directly with me—not a junior associate. I have successfully guided founders through critical seed funding, complex IP negotiations, and international expansion. My goal is to ensure your corporate framework is robust enough to secure investment today and scalable enough to support your exit tomorrow." }
                        </p>
                    </div>
                </div>
            </div>
        </section>
    }
}
