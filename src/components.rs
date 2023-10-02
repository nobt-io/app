use rscx::{component, html};

#[component]
pub fn Head(title: &str) -> String {
    html! {
        <head>
            <title>{title}</title>
            <meta charset="utf-8" />
            <meta name="google-site-verification" content="RxNEUdqyb3p6Q7WHOTY2C5hzwOFMwFUcjRFvYNFoRf0" />
            <meta name="viewport" content="width=device-width, initial-scale=1" />
            <meta name="description" content="Nobt.io is a free service to split bills among your friends. It is super simple and ease to use. Create a nobt, share the link with your friends and start splitting bills." />
            <meta name="keywords" content="nobt,nobtio,bills,friends,ease,payments,settle up,split bills,money,trips,roadtrips,lunch,party" />
            <link href="https://fonts.googleapis.com/css?family=Courgette|Comfortaa:700" rel="stylesheet" />
            <link href="https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@48,500,1,0" rel="stylesheet"/>
            <link href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.1.1/css/all.min.css" rel="stylesheet" />
            <link href="/style.css" rel="stylesheet"/>
            <script src="https://unpkg.com/htmx.org@1.9.6/dist/htmx.js" crossorigin="anonymous" />
            <script src="https://unpkg.com/htmx.org/dist/ext/preload.js" crossorigin="anonymous" />
        </head>
    }
}
