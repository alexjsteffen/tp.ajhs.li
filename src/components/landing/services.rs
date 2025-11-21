use yew::prelude::*;

#[function_component(Services)]
pub fn services() -> Html {
    html! {
        <section id="services" class="section">
            <div class="container">
                <h2 class="visually-hidden">{ "Our Services" }</h2>
                <div class="services-grid">
                    <div class="card">
                        <h3 class="h3">{ "Startup Business Assistance" }</h3>
                        <ul>
                            <li>{ "Entity Formation" }</li>
                            <li>{ "Intellectual Property Protection" }</li>
                            <li>{ "Funding Strategies" }</li>
                            <li>{ "Regulatory Compliance" }</li>
                        </ul>
                    </div>
                    <div class="card">
                        <h3 class="h3">{ "Corporate Matters" }</h3>
                        <ul>
                            <li>{ "Contract Negotiation" }</li>
                            <li>{ "Mergers & Acquisitions" }</li>
                            <li>{ "Corporate Governance" }</li>
                        </ul>
                    </div>
                </div>
            </div>
        </section>
    }
}
