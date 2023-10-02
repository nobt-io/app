use axum::response::{Html, IntoResponse};
use rscx::{component, html, EscapeAttribute};
use crate::components::Head;

const NBSP: &str = "\u{00a0}";

pub async fn index() -> impl IntoResponse {
    Html(html! {
        <!DOCTYPE html>
        <Head title="nobt.io: Split your bills with ease">
            <script src="/team.js" />
        </Head>
        <body hx-boost="true" hx-ext="preload">
            <header class="bg-transparent fixed top-0 w-full text-white p-5"> // TODO: Perhaps make this change background on scroll.
                <nav class="flex">
                    <div class="grow">
                        <a class="p-4" href="/">nobt.io</a>
                    </div>
                    <div class="text-right">
                        // TODO: Add icons here
                        <a class="p-4" href="#about">About</a>
                        <a class="p-4" href="#features">Features</a>
                        <a class="p-4" href="#team">Team</a>
                    </div>
                </nav>
            </header>

            <section class="h-screen bg-landing-page bg-cover bg-center flex items-center justify-center sm:grid grid-cols-12">
                <div class="col-span-7 text-right p-10 text-white">
                    <h1 class="text-4xl font-bold">split your bills</h1>
                    <h2 class="text-3xl font-handWritten mb-10">with ease</h2>
                    <a class="bg-black/60 hover:bg-darkGreen hover:border-darkGreen px-6 py-3 rounded-full text-md border-white border-2 uppercase text-sm" href="/create">Get started - Create a Nobt</a>
                </div>
                <div class="col-span-5">
                    // TODO: Add phone with screenshot here.
                </div>
            </section>

            <section id="about" class="py-20 bg-gray-100">
                <div class="container mx-auto px-4 md:w-3/5">
                    <div class="text-center mb-10">
                        <h2 class="text-4xl">You will{NBSP}<span class="fa-regular fa-heart text-red"></span>{NBSP}nobt.io</h2>
                        <p class="mt-4 text-xl">Nobt.io is a free service that solves the tedious task of splitting several bills among your friends with ease.</p>
                        <p class="mt-2 text-xl">Try it for your holiday.</p> // TODO: Make this dynamic "typewriter" text.
                    </div>
                    <div class="flex justify-center mt-6">
                        <a class="bg-darkGreen px-6 py-3 rounded-full text-md text-white uppercase" href="/create">Get started - Create a Nobt</a>
                    </div>
                </div>
            </section>

            <section id="features" class="py-20 bg-darkGreen text-white">
                <div class="container mx-auto px-4">
                    <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8">
                        <div class="p-4 text-center">
                            <div class="flex justify-center mb-4">
                                <i class="fa-solid fa-gauge-high text-4xl text-black"></i>
                            </div>
                            <h3 class="text-2xl mb-4">Easy to Use</h3>
                            <p>Nobt.io works without registration. Simply create a nobt and share the link with your friends.</p>
                        </div>

                        <div class="p-4 text-center">
                            <div class="flex justify-center mb-4">
                                <i class="fa-solid fa-cloud text-4xl text-black"></i>
                            </div>
                            <h3 class="text-2xl mb-4">Available Everywhere</h3>
                            <p>Nobts are stored in the cloud, so you can access them from any device, anytime, no matter where you are.</p>
                        </div>

                        <div class="p-4 text-center">
                            <div class="flex justify-center mb-4">
                                <i class="fa-brands fa-github text-4xl text-black"></i>
                            </div>
                            <h3 class="text-2xl mb-4">Open Source</h3>
                            <p>"We believe that the Web should be open that's why we share everything about nobt.io, except your data."</p>
                        </div>
                    </div>
                </div>
            </section>

            <section id="team" class="py-20 bg-white text-center">
                <div class="container mx-auto px-4 md:w-3/5">
                    <div class="mb-10">
                        <h2 class="text-4xl">The team behind nobt.io.</h2>
                        <p class="mt-4">Crafted with{NBSP}<span class="fa-solid fa-mug-saucer"></span>{NBSP}in Vienna and Sydney.</p>
                        <p class="mt-4">What started as a hackathon by three motivated developers soon turned out to be an actually helpful companion in our daily life. Nobt.io is our effort to share this idea with all of you. We hope you enjoy it as much as we do.</p>
                    </div>
                    <div>
                        <div id="teamMember1" style="background-position: 0px -600px;" class="group m-2 inline-block h-[200px] w-[200px] bg-[url('/thomas.png')]">
                            <TeamMember name="Thomas" github="thomaseizinger" linked_in="thomas-eizinger-b45a37144" homepage="https://eizinger.io" />
                        </div>
                        <div id="teamMember2" style="background-position: 0px -1000px;" class="group m-2 inline-block h-[200px] w-[200px] bg-[url('/david.png')]">
                            <TeamMember name="David" github="duffleit" linked_in="David_Leitner4" homepage="https://leitner.io" />
                        </div>
                        <div id="teamMember3" style="background-position: 0px -800px;" class="group m-2 inline-block h-[200px] w-[200px] bg-[url('/matthias.png')]">
                            <TeamMember name="Matthias" github="KreMat" linked_in="Matthias_Kreuzriegler" homepage="https://kreuzriegler.at" />
                        </div>
                    </div>
                </div>
            </section>

            // TODO Add icons here
            <footer>
                <a href="mailto:hello@nobt.io">Contact Us</a>
                <a href="https://twitter.com/nobtio">Twitter</a>
                <a href="https://github.com/nobt-io">GitHub</a>
            </footer>
        </body>
    })
}

#[component]
fn TeamMember(name: &str, github: &str, linked_in: &str, homepage: &str) -> String {
    html! {
        <div class="invisible group-hover:visible bg-black opacity-80 justify-center w-full h-full text-white flex flex-col gap-4">
            <h3 class="text-2xl font-handWritten">{name}</h3>
            <ul class="flex justify-center gap-2">
                <li>
                    <a href=format!("https://github.com/{github}") target="_blank">
                        <i class="text-xl fa-brands fa-github">
                        </i>
                    </a>
                </li>
                <li>
                    <a href=format!("https://www.linkedin.com/in/{linked_in}") target="_blank">
                        <i class="text-xl fa-brands fa-linkedin">
                        </i>
                    </a>
                </li>
                <li>
                    <a href=homepage target="_blank">
                        <i class="text-xl fa-solid fa-house">
                        </i>
                    </a>
                </li>
            </ul>
        </div>
    }
}

