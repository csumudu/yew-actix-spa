use yew::prelude::*;

#[function_component]
pub fn Header() -> Html {
    html! {
            <nav class="navbar navbar-expand-lg  bg-dark" data-bs-theme="dark">
                <div class="container-fluid">
                    <a class="navbar-brand" href="/">{"Rust Image Gallery - Single Page App"}</a>
                    <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNav" aria-controls="navbarNav" aria-expanded="false" aria-label="Toggle navigation">
                    <span class="navbar-toggler-icon"></span>
                    </button>
                    <div class="collapse navbar-collapse" id="navbarNav">
                    <ul class="navbar-nav">
                        <li class="nav-item">
                        <a class="nav-link" href="/counter">{"Counter"}</a>
                        </li>
                        <li class="nav-item">
                        <a class="nav-link" href="#">{"Pricing"}</a>
                        </li>
                        <li class="nav-item">
                        <a class="nav-link disabled">{"Disabled"}</a>
                        </li>
                    </ul>
                    </div>
                </div>
            </nav>
        }
}
