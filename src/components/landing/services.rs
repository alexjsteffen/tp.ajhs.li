use yew::prelude::*;

#[function_component(Services)]
pub fn services() -> Html {
    html! {
        <section id="services" class="section">
            <div class="container">
                <h2 class="visually-hidden">{ "Our Services" }</h2>
                <div class="services-grid">
                    <div class="card">
                        <h3 class="h3">{ "Launch & Structure" }</h3>
                        <ul>
                            <li>{ "Entity Selection & Formation" }</li>
                            <li>{ "Cross-Border Corporate Structuring" }</li>
                            <li>{ "Co-Founder Agreements" }</li>
                            <li>{ "Intellectual Property Strategy" }</li>
                        </ul>
                    </div>
                    <div class="card">
                        <h3 class="h3">{ "Growth & Finance" }</h3>
                        <ul>
                            <li>{ "Venture Capital & Fundraising" }</li>
                            <li>{ "Mergers & Acquisitions (M&A)" }</li>
                            <li>{ "Commercial Contracts & Licensing" }</li>
                            <li>{ "Regulatory & Financial Compliance" }</li>
                        </ul>
                    </div>
                </div>
            </div>
        </section>
    }
}
