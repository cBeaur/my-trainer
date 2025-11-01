use dioxus::prelude::*;

const NAVBAR_CSS: Asset = asset!("/assets/styling/navbar.css");
const FERRIS_SVG: Asset = asset!("/assets/Original_Ferris.svg");
const BARBELL_SVG: Asset = asset!("/assets/barbell.png");

#[component]
pub fn Navbar(children: Element) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: NAVBAR_CSS }

        div {
            id: "navbar",
            {children}

            div {
                id: "navbar-logo",
                img {
                    id: "barbell0",
                    src: BARBELL_SVG,
                    alt: "An orange barbell icon"
                }

                img {
                    id: "ferris",
                    src: FERRIS_SVG,
                    alt: "Ferris the crab"
                }

                img {
                    id: "barbell1",
                    src: BARBELL_SVG,
                    alt: "An orange barbell icon"
                }
            }

        }
    }
}
