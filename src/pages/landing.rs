use yew::prelude::*;
use crate::components::landing::*;

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    html! {
        <>
            <Header />
            <main>
                <Hero />
                <Experience />
                <Services />
                <Highlight />
                <FractionalGC />
            </main>
            <Footer />
        </>
    }
}
