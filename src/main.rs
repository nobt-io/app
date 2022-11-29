use anyhow::{Context, Result};
use axum::extract::Path;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::routing::post;
use axum::Router;
use render::html::HTML5Doctype;
use render::*;
use std::net::SocketAddr;

const STYLES: &str = include_str!(concat!(env!("OUT_DIR"), "/style.css"));

#[tokio::main]
async fn main() -> Result<()> {
    let port = std::env::var("PORT")
        .as_deref()
        .unwrap_or("3000")
        .parse()
        .context("failed to parse port")?;

    let app = Router::new()
        .route("/style.css", get(styles))
        .route("/:nobt_id", get(nobt))
        .route("/:nobt_id/balances", get(balances))
        .route("/:nobt_id/:expense_id", get(expense))
        .route("/:nobt_id/:expense_id/delete", post(delete_expense));

    axum::Server::bind(&SocketAddr::from(([0, 0, 0, 0], port)))
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn nobt(Path(nobt_id): Path<String>) -> impl IntoResponse {
    let title = "Swedish Shenanigans";
    let total = format!("EUR 1,521.00");
    let num_participants = 4;
    let expenses = vec![
        ExpenseItem {
            description: "Thomas paid 'Flughafen Essen'".to_owned(),
            amount: format!("EUR 39.00"),
            url: format!("/{nobt_id}/14"),
            deleted: false,
        },
        ExpenseItem {
            description: "Thomas paid 'Benni Gutschein'".to_owned(),
            amount: format!("EUR 150.00"),
            url: format!("/{nobt_id}/13"),
            deleted: true,
        },
        ExpenseItem {
            description: "Prada paid 'Taxi zum Club'".to_owned(),
            amount: format!("EUR 33.00"),
            url: format!("/{nobt_id}/12"),
            deleted: false,
        },
    ];
    let balances_url = format!("/{nobt_id}/balances");

    html_200(html! {
        <App title={title}>
            <Header>
                <h1 class={"text-xl"}>{"nobt.io"}</h1>
            </Header>
            <div class={"bg-green text-white p-4 flex flex-col gap-4"}>
                <h2 class={"text-center text-3xl"}>
                    {title}
                </h2>
                <ul class={"flex items-center justify-center space-x-4"}>
                    <li class={"inline-block"}>
                        <div class={"flex items-center gap-2 text-sm"}>
                            <Icon name={"credit_card"} />
                            {total}
                        </div>
                    </li>
                    <li class={"inline-block"}>
                        <div class={"flex items-center gap-2 text-sm"}>
                            <Icon name={"group"} />
                            {num_participants}
                        </div>
                    </li>
                </ul>
                <div class={"text-center"}>
                    <a href={balances_url} class={"uppercase inline-block bg-darkGreen px-3 py-2"}>{"Show balances"}</a>
                </div>
            </div>
            <div class={"bg-white p-4"}>
                <List>
                    {expenses
                        .into_iter()
                        .map(|expense| {
                            let classes = if expense.deleted {
                                "grow flex flex-col line-through opacity-30"
                            } else {
                                "grow flex flex-col"
                            };

                            rsx! {
                                <LinkListItem href={expense.url}>
                                    <ListItemIcon name={"receipt"}/>
                                    <span class={classes}>
                                        <span>{expense.description}</span>
                                        <span class={"text-sm text-darkGrey"}>{expense.amount}</span>
                                    </span>
                                </LinkListItem>
                            }
                        })
                        .collect::<Vec<_>>()}
                </List>
            </div>
        </App>
    })
}

async fn balances(Path(nobt_id): Path<String>) -> impl IntoResponse {
    let title = "Swedish Shenanigans";
    let nobt_url = format!("/{nobt_id}");

    let balances = vec![
        BalanceItem {
            initials: "SI".to_string(),
            name: "Simon".to_string(),
            amount: "-EUR 99.66".to_string(),
            url: format!("/{nobt_id}/balances/Simon"),
            is_negative: true,
        },
        BalanceItem {
            initials: "TH".to_string(),
            name: "Thomas".to_string(),
            amount: "EUR 390.34".to_string(),
            url: format!("/{nobt_id}/balances/Thomas"),
            is_negative: false,
        },
        BalanceItem {
            initials: "PR".to_string(),
            name: "Prada".to_string(),
            amount: "-EUR 290.68".to_string(),
            url: format!("/{nobt_id}/balances/Prada"),
            is_negative: true,
        },
        BalanceItem {
            initials: "BE".to_string(),
            name: "Beji".to_string(),
            amount: "EUR 0.00".to_string(),
            url: format!("/{nobt_id}/balances/Benji"),
            is_negative: false,
        },
    ];

    html_200(html! {
        <App title={title}>
            <Header>
                <a href={nobt_url} class={"material-symbols-outlined"}> // TODO: Figure out how to use `Icon` abstraction here.
                    {"chevron_left"}
                </a>
                <h1 class={"text-lg col-span-10 col-start-2 font-header uppercase font-bold text-center"}>{"Balances"}</h1>
            </Header>
            <div class={"bg-white p-4"}>
                <Section title={"The balances of all users in this Nobt."}>
                    <List>
                        {balances
                            .into_iter()
                            .map(|balance| {
                                let amount_classes = if balance.is_negative {
                                    "text-sm text-darkGrey text-red"
                                } else {
                                    "text-sm text-darkGrey text-green"
                                };

                                rsx! {
                                    <LinkListItem href={balance.url}>
                                        <Avatar initials={balance.initials} />
                                        <span class={"grow flex flex-col"}>
                                            <span>{balance.name}</span>
                                            <span class={amount_classes}>{balance.amount}</span>
                                        </span>
                                    </LinkListItem>
                                }
                            })
                            .collect::<Vec<_>>()}
                    </List>
                </Section>
            </div>
        </App>
    })
}

async fn expense(Path((nobt_id, expense_id)): Path<(String, u64)>) -> impl IntoResponse {
    let title = "Swedish Shenanigans";
    let name = "Flughafen Essen";
    let nobt_url = format!("/{nobt_id}");
    let deleted = false;
    let delete_url = format!("/{nobt_id}/{expense_id}/delete");
    let debtee_initials = "TH".to_owned();
    let debtee_name = "Thomas".to_owned();
    let added_on = "28 August 2022".to_owned();
    let total = "EUR 39.00".to_owned();

    let debtors = vec![
        DebtorItem {
            initials: "SI".to_string(),
            name: "Simon".to_string(),
            amount_owed: "-EUR 19.50".to_string(),
        },
        DebtorItem {
            initials: "TH".to_string(),
            name: "Thomas".to_string(),
            amount_owed: "-EUR 19.50".to_string(),
        },
    ];

    html_200(html! {
        <App title={title}>
            <Header>
                <a href={nobt_url} class={"material-symbols-outlined"}> // TODO: Figure out how to use `Icon` abstraction here.
                    {"chevron_left"}
                </a>
                <h1 class={"text-lg col-span-10 col-start-2 font-header uppercase font-bold text-center"}>{name}</h1>
            </Header>
            <div class={"bg-white p-4 space-y-4"}>
                <Section title={"Debtee"}>
                    <List>
                        <ListItem>
                            <Avatar initials={debtee_initials} />
                            {format!("{debtee_name} paid this bill.")}
                        </ListItem>
                        <ListItem>
                            <ListItemIcon name={"access_time"}/>
                            {format!("Added on {added_on}.")}
                        </ListItem>
                        <ListItem>
                            <ListItemIcon name={"credit_card"}/>
                            {format!("The invoice total is {total}.")}
                        </ListItem>
                    </List>
                </Section>
                <Section title={"Debtors"}>
                    <List>
                        {debtors
                            .into_iter()
                            .map(|debtor| rsx! {
                                <ListItem>
                                    <Avatar initials={debtor.initials} />
                                    <span class={"flex-grow"}>{debtor.name}</span>
                                    <span class={"text-red"}>{debtor.amount_owed}</span>
                                </ListItem>
                            })
                            .collect::<Vec<_>>()}
                    </List>
                </Section>
                {(!deleted).then(|| rsx! {
                   <Section title={"Actions"}>
                        <List>
                            <FormListItem href={delete_url} confirm={"Deleting a bill is permanent. Proceed?"}>
                                <ListItemIcon name={"delete"}/>
                                {"Delete this bill"}
                            </FormListItem>
                        </List>
                    </Section>
                })}
            </div>
        </App>
    })
}

/// Deletes an expense from a nobt.
///
/// This returns a 303 See Other which is the appropriate way of sending the user somewhere else
/// after a successful POST request.
///
/// See <https://www.rfc-editor.org/rfc/rfc9110.html#name-303-see-other>.
async fn delete_expense(Path((nobt_id, _expense_id)): Path<(String, u64)>) -> impl IntoResponse {
    Response::builder()
        .status(303)
        .header("Location", format!("/{nobt_id}"))
        .body(String::new())
        .unwrap()
}

async fn styles() -> impl IntoResponse {
    Response::builder()
        .header("Content-Type", "text/css")
        .body(STYLES.to_owned())
        .unwrap()
}

struct ExpenseItem {
    description: String,
    amount: String,
    url: String,
    deleted: bool,
}

fn html_200(body: String) -> Response<String> {
    Response::builder()
        .status(200)
        .header("Content-Type", "text/html")
        .body(body)
        .unwrap()
}

struct DebtorItem {
    initials: String,
    name: String,
    amount_owed: String,
}

struct BalanceItem {
    initials: String,
    name: String,
    amount: String,
    url: String,
    is_negative: bool,
}

#[component]
fn App<'a, C>(title: &'a str, children: C)
where
    C: Render,
{
    rsx! {
        <>
            <HTML5Doctype />
            <head>
                <title>{title}</title>
                <meta charset={"utf-8"} />
                <meta name={"google-site-verification"} content={"RxNEUdqyb3p6Q7WHOTY2C5hzwOFMwFUcjRFvYNFoRf0"} />
                <meta name={"viewport"} content={"width=device-width, initial-scale=1"} />
                <meta name={"description"} content={"Nobt.io is a free service to split bills among your friends. It is super simple and ease to use. Create a nobt, share the link with your friends and start splitting bills."} />
                <meta name={"keywords"} content={"nobt,nobtio,bills,friends,ease,payments,settle up,split bills,money,trips,roadtrips,lunch,party"} />
                <link href={"https://fonts.googleapis.com/css?family=Courgette|Comfortaa:700"} rel={"stylesheet"} />
                <link href={"https://fonts.googleapis.com/css2?family=Material+Symbols+Outlined:opsz,wght,FILL,GRAD@48,500,1,0"} rel={"stylesheet"}/>
                <link href={"/style.css"} rel={"stylesheet"}/>
                <script src={"https://unpkg.com/htmx.org@1.8.4"} integrity={"sha384-wg5Y/JwF7VxGk4zLsJEcAojRtlVp1FKKdGy1qN+OMtdq72WRvX/EdRdqg/LOhYeV"} crossorigin={"anonymous"}>{""}</script>
            </head>
            <body hx-boost={"true"} class={"bg-lightGrey h-screen"}>
                <div class={"sm:pt-10"}>
                    <div class={"container mx-auto shadow-lg rounded-lg max-w-3xl"}>
                        {children}
                    </div>
                </div>
            </body>
        </>
    }
}

#[component]
fn Section<'a, C>(title: &'a str, children: C)
where
    C: Render,
{
    rsx! {
        <section class={"space-y-4"}>
            <h2 class={"text-darkGrey text-xs"}>{title}</h2>
            {children}
        </section>
    }
}

#[component]
fn List<C>(children: C)
where
    C: Render,
{
    rsx! {
        <ul class={"flex flex-col gap-4"}>
            {children}
        </ul>
    }
}

#[component]
fn ListItem<C>(children: C)
where
    C: Render,
{
    rsx! {
        <li class={"block flex items-center gap-4"}>
            {children}
        </li>
    }
}

#[component]
fn LinkListItem<C>(href: String, children: C)
where
    C: Render,
{
    rsx! {
        <li>
            <a class={"block flex items-center gap-4"} href={href}>
                {children}
                <Icon name={"chevron_right"} />
            </a>
        </li>
    }
}

#[component]
fn FormListItem<C>(href: String, confirm: &'static str, children: C)
where
    C: Render,
{
    rsx! {
        <li>
            <form action={href} method={"post"} hx-confirm={confirm}>
                <button type={"submit"} class={"block flex items-center gap-4 w-full"}>
                    {children}
                </button>
            </form>
        </li>
    }
}

#[component]
fn Icon<'a>(name: &'a str) {
    rsx! {
        <span class={"material-symbols-outlined"}>{name}</span>
    }
}

#[component]
fn ListItemIcon<'a>(name: &'a str) {
    rsx! {
        <span class={"material-symbols-outlined text-darkGrey"}>{name}</span>
    }
}

#[component]
fn Avatar(initials: String) {
    rsx! {
        <div class={"flex items-center justify-center bg-green text-bold rounded-full h-6 w-6 text-xs text-white leading-normal"}>
            {initials}
        </div>
    }
}

#[component]
fn Header<C>(children: C)
where
    C: Render,
{
    rsx! {
        <header class={"bg-grey text-white px-4 h-16 grid grid-cols-12 items-center"}>
            {children}
        </header>
    }
}
