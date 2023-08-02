use anyhow::{Context, Result};
use axum::extract::Path;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::get;
use axum::routing::post;
use axum::Router;
use axum_extra::extract::Form;
use render::html::HTML5Doctype;
use render::*;
use std::borrow::Cow;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::net::SocketAddr;

use responses::Css;
use responses::Jpeg;

mod headers;
mod responses;

const STYLES: &str = include_str!(concat!(env!("OUT_DIR"), "/style.css"));
const NOT_FOUND_IMAGE: &[u8] = include_bytes!("../stock-photo-stack-424916446.jpg");

#[tokio::main]
async fn main() -> Result<()> {
    let port = std::env::var("PORT")
        .as_deref()
        .unwrap_or("3000")
        .parse()
        .context("failed to parse port")?;

    let app = Router::new()
        .route("/style.css", get(|| async { Css(STYLES) }))
        .route("/not_found.jpg", get(|| async { Jpeg(NOT_FOUND_IMAGE) }))
        .route("/:nobt_id", get(nobt))
        .route("/:nobt_id/bill", get(new_bill))
        .route("/:nobt_id/bill", post(new_bill))
        .route("/:nobt_id/bill/new", post(add_new_bill))
        .route("/:nobt_id/bill/debtee", post(choose_bill_debtee))
        .route("/:nobt_id/bill/debtors", get(choose_bill_debtors))
        .route("/:nobt_id/bill/debtors", post(choose_bill_debtors))
        .route("/:nobt_id/balances", get(balances))
        .route("/:nobt_id/balances/:name", get(individual_balance))
        .route("/:nobt_id/:expense_id", get(expense))
        .route("/:nobt_id/:expense_id/delete", post(delete_expense))
        .fallback(not_found);

    axum::Server::bind(&SocketAddr::from(([0, 0, 0, 0], port)))
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

#[derive(serde::Deserialize, serde::Serialize)]
struct NewBillParameters {
    name: Option<String>,
    total: Option<f64>,
    debtee: Option<String>,
    debtors: Option<HashSet<String>>,
}

async fn nobt(Path(nobt_id): Path<String>) -> impl IntoResponse {
    let title = "Swedish Shenanigans";
    let total = 1521.00;
    let currency = "EUR".to_owned();
    let num_participants = 4;
    let expenses = vec![
        ExpenseItem {
            description: "Thomas paid 'Flughafen Essen'".to_owned(),
            amount: 39.00,
            url: format!("/{nobt_id}/14"),
            deleted: false,
        },
        ExpenseItem {
            description: "Thomas paid 'Benni Gutschein'".to_owned(),
            amount: 150.00,
            url: format!("/{nobt_id}/13"),
            deleted: true,
        },
        ExpenseItem {
            description: "Prada paid 'Taxi zum Club'".to_owned(),
            amount: 33.00,
            url: format!("/{nobt_id}/12"),
            deleted: false,
        },
    ];
    let balances_url = format!("/{nobt_id}/balances");

    Html(html! {
        <App title={title}>
            <Header>
                <h1 class={"text-xl"}>{"nobt.io"}</h1>
            </Header>
            <div class={"bg-turquoise text-white p-4 flex flex-col gap-4"}>
                <h2 class={"text-center text-3xl"}>
                    {title}
                </h2>
                <ul class={"flex items-center justify-center space-x-4"}>
                    <li class={"inline-block"}>
                        <div class={"flex items-center gap-2 text-sm"}>
                            <Icon name={"credit_card"} />
                            <Amount currency={&currency} value={total} classes={""}/>
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
                        .iter()
                        .map(|expense| {
                            let classes = if expense.deleted {
                                "grow flex flex-col line-through opacity-30"
                            } else {
                                "grow flex flex-col"
                            };

                            rsx! {
                                <LinkListItem href={&expense.url}>
                                    <ListItemIcon name={"receipt"}/>
                                    <span class={classes}>
                                        <span>{expense.description.as_str()}</span>
                                        <Amount currency={&currency} value={expense.amount} classes={"text-darkGrey"}/>
                                    </span>
                                </LinkListItem>
                            }
                        })
                        .collect::<Vec<_>>()}
                </List>
            </div>
            <FAB nobt_id={&nobt_id}/>
        </App>
    })
}

async fn new_bill(
    Path(nobt_id): Path<String>,
    Form(params): Form<NewBillParameters>,
) -> impl IntoResponse {
    let title = "Swedish Shenanigans";
    let _currency = "EUR".to_owned();
    let nobt_url = format!("/{nobt_id}");
    let mut names = HashSet::from([
        "Thomas".to_owned(),
        "Simon".to_owned(),
        "Prada".to_owned(),
        "Benji".to_owned(),
    ]);

    // TODO: Merge into component?
    if let Some(new_debtee) = &params.debtee {
        names.insert(new_debtee.to_owned());
    }

    let debtors = params.debtors.as_ref().unwrap_or_else(|| &names);

    Html(html! {
        <App title={title}>
            <Header>
                <BackLink href={&nobt_url}/>
                <HeaderTitle title={"Add a bill"} />
            </Header>
            <form class={"bg-turquoise p-4 flex flex-col gap-4"}>
                <section class={"flex flex-col bg-white p-2"}>
                    <h2 class={"text-black font-bold text-sm"}>{"What did you buy?"}</h2>
                    <input required={"true"} class={"outline-none peer border-b py-2"} name={"name"} value={params.name.unwrap_or_default()} placeholder={"Trip Snacks, Train Tickets, Beer, ..."} />
                    <span class={"text-xs text-[grey]"}>{"Enter a descriptive name for what was paid for."}</span>
                </section>
                <section class={"flex flex-col bg-white p-2"}>
                    <h2 class={"text-black font-bold text-sm"}>{"How much did it cost?"}</h2>
                    <div class={"flex items-center"}>
                        <span class={"w-10 h-10 text-[grey] flex items-center justify-center text-xl"}>{"€"}</span>
                        <input required={"true"} class={"outline-none peer border-b py-2 appearance-none w-full"} name={"total"} value={params.total.map(|t| t.to_string()).unwrap_or_default()} step={"0.01"} min={"0"} type={"number"} placeholder={"0.00"} /> // TODO: Don't set 0 by default
                    </div>
                    <span class={"text-xs text-[grey]"}>{"Enter the total of this bill."}</span>
                </section>
                <section class={"flex flex-col bg-white p-2 gap-2"}>
                    <h2 class={"text-black font-bold text-sm"}>{"Who paid?"}</h2>
                    <button type={"submit"} formnovalidate={"true"} formmethod={"post"} formaction={format!("/{nobt_id}/bill/debtee")} class={"flex items-center hover:bg-hover cursor-pointer"}>
                        <span class={"w-10 h-10 flex items-center justify-center text-xl text-[grey] material-symbols-outlined"}>
                            {"person"}
                            {match params.debtee.as_deref() {
                                Some(debtee) => rsx! {
                                    <input class={"appearance-none"} required={"true"} type={"radio"} name={"debtee"} checked={"checked"} value={debtee} />
                                },
                                None => rsx! {
                                    <input class={"appearance-none"} required={"true"} type={"radio"} name={"debtee"} />
                                }
                            }}
                        </span>
                        <span class={match &params.debtee {
                            Some(_) => "text-black text-left flex-grow",
                            None => "text-[grey] text-left flex-grow",
                        }}>
                            {match &params.debtee {
                                Some(debtee) => format!("{debtee} paid the bill."),
                                None => "Select a Debtee".to_owned(),
                            }}
                        </span>
                        <span class={"w-10 h-10 flex items-center justify-center text-xl text-[grey] material-symbols-outlined"}>
                            {"edit"}
                        </span>
                    </button>
                    <span class={"text-xs text-[grey]"}>{"Select the person who paid this bill."}</span>
                </section>
                <section class={"flex flex-col bg-white p-2 gap-2"}>
                    <h2 class={"text-black font-bold text-sm"}>{"Who is involved?"}</h2>
                    <button type={"submit"} formnovalidate={"true"} formmethod={"post"} formaction={format!("/{nobt_id}/bill/debtors")} class={"flex items-center hover:bg-hover cursor-pointer"}>
                        <span class={"w-10 h-10 flex items-center justify-center text-xl text-[grey] material-symbols-outlined"}>
                            {"group"}
                        </span>
                        {debtors
                            .iter()
                            .map(|d| rsx! {
                                <input type={"hidden"} name={"debtors"} value={d}/>
                            })
                            .collect::<Vec<_>>()}
                        <span class={match &debtors.len() {
                            0 => "text-[grey] text-left flex-grow",
                            _ => "text-black text-left flex-grow",
                        }}>
                            {match debtors.len() {
                                0 => "Nobody is involved".to_owned(),
                                1 => "1 person is involved".to_owned(),
                                num => format!("{num} persons are involved."),
                            }}
                        </span>
                        <span class={"w-10 h-10 flex items-center justify-center text-xl text-[grey] material-symbols-outlined"}>
                            {"edit"}
                        </span>
                    </button>
                    <span class={"text-xs text-[grey]"}>{"Select who is involved in this bill."}</span>
                </section>
                <div>
                    <button class={"flex items-center justify-center gap-2 text-white uppercase rounded shadow px-4 py-2 bg-darkGreen"} type={"submit"} formmethod={"post"} formaction={format!("/{nobt_id}/bill/new")}>
                        <Icon name={"check_circle"} />
                        {"Add bill"}
                    </button>
                </div>
            </form>
        </App>
    })
}

async fn add_new_bill(Form(new_bill): Form<NewBillForm>) -> impl IntoResponse {
    println!("{new_bill:?}");

    ""
}

#[derive(serde::Deserialize, Debug)]
struct NewBillForm {
    name: String,
    total: f64,
    debtee: String,
    debtors: Vec<String>,
}

async fn choose_bill_debtee(
    Path(nobt_id): Path<String>,
    Form(params): Form<NewBillParameters>,
) -> impl IntoResponse {
    let title = "Swedish Shenanigans";
    let _currency = "EUR".to_owned();
    let back_link = format!("/{nobt_id}/bill");
    let mut names = HashSet::from([
        "Thomas".to_owned(),
        "Simon".to_owned(),
        "Prada".to_owned(),
        "Benji".to_owned(),
    ]);

    if let Some(debtee) = params.debtee.as_ref() {
        names.insert(debtee.to_owned());
    }
    if let Some(debtors) = params.debtors.clone() {
        names.extend(debtors.iter().map(|s| s.to_owned()));
    }

    let bill_name = params.name.as_deref();
    let selected_debtee = params.debtee.as_deref();
    let total = params.total.as_ref();
    let debtors = params.debtors.unwrap_or_default();

    Html(html! {
        <App title={title}>
            <Header>
                <BackLink href={&back_link}/>
                <HeaderTitle title={"Select debtee"} />
            </Header>
            <div class={"bg-turquoise p-4 flex flex-col gap-4"}>
                <section class={"flex flex-col bg-white p-2 gap-2"}>
                    <h2 class={"text-black font-bold text-sm"}>{"Who paid?"}</h2>

                    {names
                        .iter()
                        .map(|debtee| {
                            let is_current_debtee = selected_debtee.map(|sd| &sd == debtee).unwrap_or(false);

                            rsx! {
                                <ChooseDebteeForm nobt_id={&nobt_id} name={bill_name} total={total} debtee={debtee} debtors={&debtors} is_checked={is_current_debtee} />
                            }
                        })
                        .collect::<Vec<_>>()}
                </section>

                <section class={"flex flex-col bg-white p-2 gap-2"}>
                    <h2 class={"text-black font-bold text-sm"}>{"Someone else?"}</h2>

                    <form method={"post"} action={format!("/{nobt_id}/bill")} class={"w-full flex items-center gap-2"}>
                        {bill_name.map(|name| rsx! {
                            <input type={"hidden"} name={"name"} value={name} />
                        })}
                        {total.map(|total| rsx! {
                            <input type={"hidden"} name={"total"} value={total.to_string()} />
                        })}
                        {debtors
                            .iter()
                            .map(|d| rsx! {
                                <input type={"hidden"} name={"debtors"} value={d}/>
                            })
                            .collect::<Vec<_>>()}
                        <input class={"outline-none border-b appearance-none w-full flex-grow p-2 truncate"} type={"text"} name={"debtee"} placeholder={"Bart, Milhouse, Nelson, ..."}/>
                        <button class={"flex items-center hover:bg-hover gap-2 py-2 px-4 rounded-md shadow cursor-pointer"}>
                            <Icon name={"person_add"} />
                            {"Add"}
                        </button>
                    </form>
                </section>
            </div>
        </App>
    })
}

// TODO:
// - needs checkboxes (fake?)
// - needs submit button
// - needs add person button
async fn choose_bill_debtors(
    Path(nobt_id): Path<String>,
    Form(params): Form<NewBillParameters>,
) -> impl IntoResponse {
    let title = "Swedish Shenanigans";
    let _currency = "EUR".to_owned();
    let back_link = format!("/{nobt_id}/bill");
    let mut names = HashSet::from([
        "Thomas".to_owned(),
        "Simon".to_owned(),
        "Prada".to_owned(),
        "Benji".to_owned(),
    ]);

    if let Some(debtee) = params.debtee.as_ref() {
        names.insert(debtee.to_owned());
    }
    if let Some(debtors) = params.debtors.clone() {
        names.extend(debtors.iter().map(|s| s.to_owned()));
    }

    let bill_name = params.name.as_deref();
    let selected_debtee = params.debtee.as_deref();
    let total = params.total.as_ref();
    let debtors = params.debtors.unwrap_or(names.clone());

    Html(html! {
        <App title={title}>
            <Header>
                <BackLink href={&back_link}/>
                <HeaderTitle title={"Select debtors"} />
            </Header>
            <div class={"bg-turquoise p-4 flex flex-col gap-4"}>
                <section class={"flex flex-col bg-white p-2 gap-2"}>
                    <h2 class={"text-black font-bold text-sm"}>{"Who is in?"}</h2>

                    <form method={"post"} action={format!("/{nobt_id}/bill")}>
                        {bill_name.map(|name| rsx! {
                            <input type={"hidden"} name={"name"} value={name} />
                        })}
                        {selected_debtee.map(|debtee| rsx! {
                            <input type={"hidden"} name={"debtee"} value={debtee} />
                        })}
                        {total.map(|total| rsx! {
                            <input type={"hidden"} name={"total"} value={total.to_string()} />
                        })}
                        {names
                            .iter()
                            .map(|d| {
                                let id = format!("{d}_involved_checkbox");

                                rsx! {
                                    <div class={"flex items-center hover:bg-hover p-2 cursor-pointer"}>
                                        <label class={"flex-grow flex items-center gap-2"} for={id.clone()}>
                                            <Avatar name={d.as_str()} />
                                            {d.as_str()}
                                        </label>
                                        {if debtors.contains(d) || debtors.is_empty() {
                                            rsx! { <input id={id} type={"checkbox"} name={"debtors"} checked={"checked"} value={d.as_str()}/> }
                                        } else {
                                            rsx! { <input id={id} type={"checkbox"} name={"debtors"} value={d.as_str()}/> }
                                        }}
                                    </div>
                                }
                            })
                            .collect::<Vec<_>>()}

                        <div class={"flex flex-row-reverse"}>
                            <button type={"submit"} class={"flex items-center hover:bg-hover gap-2 py-2 px-4 rounded-md shadow w-full justify-center"}> // TODO: Make this a button component.
                                <Icon name={"check_circle"}/>
                                {"Set debtors"}
                            </button>
                        </div>
                    </form>
                </section>

                <section class={"flex flex-col bg-white p-2 gap-2"}>
                    <h2 class={"text-black font-bold text-sm"}>{"Someone else?"}</h2>

                    <form method={"post"} action={format!("/{nobt_id}/bill/debtors")} class={"w-full flex items-center gap-2"}>
                        {bill_name.map(|name| rsx! {
                            <input type={"hidden"} name={"name"} value={name} />
                        })}
                        {selected_debtee.map(|debtee| rsx! {
                            <input type={"hidden"} name={"debtee"} value={debtee} />
                        })}
                        {total.map(|total| rsx! {
                            <input type={"hidden"} name={"total"} value={total.to_string()} />
                        })}
                        {debtors
                            .iter()
                            .map(|d| rsx! {
                                <input type={"hidden"} name={"debtors"} value={d}/>
                            })
                            .collect::<Vec<_>>()}
                        <input class={"outline-none border-b appearance-none w-full flex-grow p-2 truncate"} type={"text"} name={"debtors"} placeholder={"Bart, Milhouse, Nelson, ..."}/>
                        <button class={"flex items-center hover:bg-hover gap-2 py-2 px-4 rounded-md shadow"}>
                            <Icon name={"person_add"} />
                            {"Add"}
                        </button>
                    </form>
                </section>
            </div>
        </App>
    })
}

#[component]
fn ChooseDebteeForm<'a>(
    nobt_id: &'a str,
    name: Option<&'a str>,
    total: Option<&'a f64>,
    debtee: &'a str,
    debtors: &'a HashSet<String>,
    is_checked: bool,
) {
    rsx! {
        <form method={"post"} action={format!("/{nobt_id}/bill")} class={"w-full"}>
            <input type={"hidden"} name={"debtee"} value={debtee} />
            {name.map(|name| rsx! {
                <input type={"hidden"} name={"name"} value={name} />
            })}
            {total.map(|total| rsx! {
                <input type={"hidden"} name={"total"} value={total.to_string()} />
            })}
            {debtors
                .iter()
                .map(|d| rsx! {
                    <input type={"hidden"} name={"debtors"} value={d}/>
                })
                .collect::<Vec<_>>()}
            <button class={"flex items-center hover:bg-hover gap-2 p-2 cursor-pointer w-full"}>
                <Avatar name={debtee} />
                <span class={"flex-grow text-left"}>{debtee}</span>
                <span class={"flex items-center justify-center rounded-full border border-darkGrey w-3.5 h-3.5"}>
                    {is_checked.then(|| rsx! {
                        <span class={"block bg-turquoise rounded-full w-2 h-2"}></span>
                    })}
                </span>
            </button>
        </form>
    }
}

#[component]
fn PersonRadiobox<'a>(name: &'a str, required: bool) {
    let id = format!("{name}_debtee");

    rsx! {
        <div class={"flex items-center w-full gap-2 hover:bg-hover"}>
            <label class={"flex items-center flex-grow gap-2 p-2"} for={id.clone()}>
                <Avatar name={name} />
                <span class={"flex-grow"}>{name}</span>
            </label>
            <div class={"p-2 flex items-center justify-center"}>
                <input id={id} required={required.to_string()} type={"radio"} name={"debtee"} value={name} /> // TODO: Make a custom input based on brand colors
            </div>
        </div>
    }
}

#[component]
fn PersonCheckbox<'a>(name: &'a str) {
    let id = format!("{name}_involved_checkbox");

    rsx! {
        <div class={"flex items-center w-full gap-2 hover:bg-hover"}>
            <label class={"flex items-center flex-grow gap-2 p-2"} for={id.clone()}>
                <Avatar name={name} />
                <span class={"flex-grow"}>{name}</span>
            </label>
            <div class={"p-2 flex items-center justify-center"}>
                <input id={id} checked={"true"} type={"checkbox"} name={"involved"} value={name} />  // TODO: Make a custom input based on brand colors
            </div>
        </div>
    }
}

async fn balances(Path(nobt_id): Path<String>) -> impl IntoResponse {
    let title = "Swedish Shenanigans";
    let currency = "EUR".to_owned();
    let nobt_url = format!("/{nobt_id}");

    let balances = vec![
        BalanceItem {
            name: "Simon".to_string(),
            amount: -99.66,
            url: format!("/{nobt_id}/balances/Simon"),
        },
        BalanceItem {
            name: "Thomas".to_string(),
            amount: 390.34,
            url: format!("/{nobt_id}/balances/Thomas"),
        },
        BalanceItem {
            name: "Prada".to_string(),
            amount: -290.68,
            url: format!("/{nobt_id}/balances/Prada"),
        },
        BalanceItem {
            name: "Beji".to_string(),
            amount: 0.00,
            url: format!("/{nobt_id}/balances/Benji"),
        },
    ];

    Html(html! {
        <App title={title}>
            <Header>
                <BackLink href={&nobt_url}/>
                <HeaderTitle title={"Balances"} />
            </Header>
            <div class={"bg-white p-4"}>
                <Section title={"Balance overview"} subtitle={"The balances of all users in this Nobt."}>
                    <List>
                        {balances
                            .iter()
                            .map(|balance| rsx! {
                                <LinkListItem href={&balance.url}>
                                    <Avatar name={&balance.name} />
                                    <span class={"grow flex flex-col"}>
                                        <span>{balance.name.as_str()}</span>
                                        <ThemedAmount currency={&currency} value={balance.amount} />
                                    </span>
                                </LinkListItem>
                            })
                            .collect::<Vec<_>>()}
                    </List>
                </Section>
            </div>
        </App>
    })
}

async fn individual_balance(Path((nobt_id, name)): Path<(String, String)>) -> impl IntoResponse {
    let title = "Swedish Shenanigans";
    let currency = "EUR";
    let back_url = format!("/{nobt_id}/balances");

    let debts = vec![
        DebtItem {
            name: "Prada".to_string(),
            amount: 290.68,
        },
        DebtItem {
            name: "Simon".to_string(),
            amount: 99.66,
        },
    ];
    let debt_sum = debts.iter().map(|d| d.amount).sum();
    let debts_subtitle = format!(
        "{name} owes {} to {} person{}.",
        format_amount(currency, debt_sum),
        debts.len(),
        if debts.len() > 1 { "s" } else { "" }
    );

    Html(html! {
        <App title={title}>
            <Header>
                <BackLink href={&back_url} />
                <HeaderTitle title={&name} />
            </Header>
            <div class={"bg-white p-4 flex flex-col gap-4"}>
                <Section title={"Summary"} subtitle={""}>
                    <List>
                        <ListItem>
                            <ListItemIcon name={"info"}/>
                            {format!("{name} paid 2 bills ({}).", format_amount(currency, 1705.0))}
                        </ListItem>
                        <ListItem>
                            <ListItemIcon name={"info"}/>
                            {format!("{name} participates in 13 of 14 bills.")}
                        </ListItem>
                    </List>
                </Section>
                <Section title={"Debts"} subtitle={&debts_subtitle}>
                    <List>
                        {debts
                            .iter()
                            .map(|debt| rsx! {
                                <ListItem>
                                    <Avatar name={&debt.name} />
                                    <span class={"grow flex flex-col"}>
                                        <span>{debt.name.as_str()}</span>
                                        <ThemedAmount currency={&currency} value={debt.amount} />
                                    </span>
                                </ListItem>
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
    let debtee_name = "Thomas".to_owned();
    let currency = "EUR".to_owned();
    let added_on = "28 August 2022".to_owned();
    let total = "EUR 39.00".to_owned();

    let debtors = vec![
        DebtorItem {
            name: "Simon".to_string(),
            amount_owed: -19.50,
        },
        DebtorItem {
            name: "Thomas".to_string(),
            amount_owed: -19.50,
        },
    ];

    Html(html! {
        <App title={title}>
            <Header>
                <BackLink href={&nobt_url}/>
                <HeaderTitle title={name} />
            </Header>
            <div class={"bg-white p-4 flex flex-col gap-4"}>
                <Section title={"Debtee"} subtitle={""}>
                    <List>
                        <ListItem>
                            <Avatar name={&debtee_name} />
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
                <Section title={"Debtors"} subtitle={""}>
                    <List>
                        {debtors
                            .iter()
                            .map(|debtor| rsx! {
                                <ListItem>
                                    <Avatar name={&debtor.name} />
                                    <span class={"flex-grow"}>{debtor.name.as_str()}</span>
                                    <ThemedAmount currency={&currency} value={debtor.amount_owed} />
                                </ListItem>
                            })
                            .collect::<Vec<_>>()}
                    </List>
                </Section>
                {(!deleted).then(|| rsx! {
                   <Section title={"Actions"} subtitle={""}>
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

async fn not_found() -> impl IntoResponse {
    Html(html! {
        <>
            <HTML5Doctype />
            <Head title={"Not found"} />
            <body hx-boost={"true"} class={"bg-turquoise sm:bg-lightGrey h-screen"}>
                <div class={"sm:pt-10"}>
                    <div class={"bg-turquoise container mx-auto sm:shadow-lg sm:rounded-lg max-w-3xl"}>
                        <Header>
                            <h1 class={"text-xl"}>{"nobt.io"}</h1>
                        </Header>
                        <div class={"p-12 flex flex-col gap-4 items-center"}>
                            // <div class={"bg-cover h-80 w-2/3 bg-center bg-[url('/not_found.jpg')]"}>{""}</div>

                            <h2 class={"text-lg w-72 text-center text-white"}>{"We looked everywhere but couldn't find this nobt."}</h2>

                            <a class={"bg-white rounded-md px-4 py-2 shadow"} href={"/create"}>
                                {"Create a new nobt"}
                            </a>
                        </div>
                    </div>
                </div>
            </body>
        </>
    })
}

struct ExpenseItem {
    description: String,
    amount: f64,
    url: String,
    deleted: bool,
}

struct DebtorItem {
    name: String,
    amount_owed: f64,
}

struct BalanceItem {
    name: String,
    amount: f64,
    url: String,
}

struct DebtItem {
    name: String,
    amount: f64,
}

#[component]
fn App<'a, C>(title: &'a str, children: C)
where
    C: Render,
{
    rsx! {
        <>
            <HTML5Doctype />
            <Head title={title} />
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
fn Head<'a>(title: &'a str) {
    rsx! {
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
            <script src={"https://unpkg.com/htmx.org@1.9.1/dist/htmx.js"} crossorigin={"anonymous"}>{""}</script>
            <script>
                {raw! {
                    r#"
                    htmx.on('htmx:configRequest', function (event) {
                        // debugger;

                        if (event.detail.elt?.nodeName !== 'FORM') {
                            return;
                        }

                        const submitter = event.detail.triggeringEvent?.submitter;

                        if (!submitter) {
                            return;
                        }

                        // Element also has "formAction" field, but if "formaction" attribute is not filled then it has a default value.
                        // So we specifically need to check for the existence of the attribute
                        let formAction = submitter?.attributes?.formaction?.value;
                        let formMethod = submitter?.attributes?.formmethod?.value?.toLowerCase();

                        if (formAction) {
                            let oldUrl = new URL(event.detail.path);
                            oldUrl.pathname = formAction;

                            event.detail.path = oldUrl.toString();
                        }

                        if (formMethod) {
                            event.detail.verb = formMethod;

                            if (formMethod === 'post') {
                                event.detail.headers['Content-Type'] = 'application/x-www-form-urlencoded';
                            }
                        }
                    })
                    ;"#
                }}
            </script>
        </head>
    }
}

#[component]
fn Section<'a, C>(title: &'a str, subtitle: &'a str, children: C)
where
    C: Render,
{
    rsx! {
        <section class={"flex flex-col gap-4"}>
            <div class={"flex flex-col gap-2"}>
                <h2 class={"text-darkGrey text-2xl"}>{title}</h2>
                {(!subtitle.is_empty()).then(|| rsx! {
                    <h3 class={"text-darkGrey text-xs"}>{subtitle}</h3>
                })}
            </div>
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
        <ul class={"flex flex-col"}>
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
        <li class={"block flex items-center gap-4 p-2"}>
            {children}
        </li>
    }
}

#[component]
fn LinkListItem<'a, C>(href: &'a str, children: C)
where
    C: Render,
{
    rsx! {
        <li>
            <a class={"block flex items-center gap-4 cursor-pointer hover:bg-hover p-2"} href={href}>
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
                <button type={"submit"} class={"block flex items-center gap-4 w-full cursor-pointer hover:bg-hover p-2"}>
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
fn LinkIcon<'a>(href: &'a str, name: &'a str) {
    rsx! {
        <a href={href} class={"material-symbols-outlined"}>
            {name}
        </a>
    }
}

/// A back-link component that works with progressive enhancement.
///
/// In case we have JS enabled, this will simply trigger `history.back()` which takes the user back to the previous page.
/// Without JS, we simply navigate to the desired page.
#[component]
fn BackLink<'a>(href: &'a str) {
    rsx! {
        <a class={"material-symbols-outlined"} href={href} onclick={"{ history.back(); return false; }"}> // TODO: How to use `hx-on` here?
            {"chevron_left"}
        </a>
    }
}

/// Renders an amount in a given currency.
///
/// Negative amounts will appear red.
#[component]
fn ThemedAmount<'a>(currency: &'a str, value: f64) {
    if value == 0.0 {
        rsx! { <Amount currency={currency} value={value} classes={"text-darkGrey"}/> }
    } else if value < 0.0 {
        rsx! { <Amount currency={currency} value={value} classes={"text-red"}/> }
    } else {
        rsx! { <Amount currency={currency} value={value} classes={"text-green"}/> }
    }
}

/// Renders an amount in a given currency.
///
/// Negative amounts will appear red.
#[component]
fn Amount<'a>(currency: &'a str, value: f64, classes: &'static str) {
    let formatted = format_amount(currency, value);

    rsx! {
        <span class={format!("text-sm {classes}")}>{formatted}</span>
    }
}

fn format_amount(currency: &str, value: f64) -> String {
    if value == 0.0 {
        format!("{currency} 0.00")
    } else if value < 0.0 {
        let value = value.abs();

        format!("-{currency} {value:.2}")
    } else {
        format!("{currency} {value:.2}")
    }
}

#[component]
fn ListItemIcon<'a>(name: &'a str) {
    rsx! {
        <span class={"material-symbols-outlined text-darkGrey"}>{name}</span>
    }
}

#[component]
fn Avatar<'a>(name: &'a str) {
    let initials = make_initials(&name);
    let bg_color = pick_bg_color(name);

    rsx! {
        <div class={format!("flex items-center justify-center {bg_color} text-bold rounded-full h-6 w-6 text-xs text-white leading-normal uppercase")}>
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

#[component]
fn HeaderTitle<'a>(title: &'a str) {
    rsx! {
        <h1 class={"text-lg col-span-10 col-start-2 uppercase font-bold text-center"}>{title}</h1>
    }
}

#[component]
fn FAB<'a>(nobt_id: &'a str) {
    rsx! {
        <div class={"fixed bottom-6 right-6 transform-gpu space-y-4 text-right"}>
            <input id={"fab-toggle"} type={"checkbox"} class={"hidden peer"}/>
            <FABLink href={format!("/{nobt_id}/payment")} icon={"credit_card"} text={"Pay someone"} disabled={true} index={1}/>
            <FABLink href={format!("/{nobt_id}/bill")} icon={"receipt"} text={"Add a bill"} disabled={false} index={0}/>
            <label for={"fab-toggle"} class={"relative z-20 inline-block peer-checked:rotate-[225deg] duration-300 transition-transform cursor-pointer"}>
                <FABIcon name={"add"} styles={"bg-turquoise text-white"}/>
            </label>
        </div>
    }
}

#[component]
fn FABIcon<'a>(name: &'a str, styles: &'a str) {
    rsx! {
        <span class={format!("{styles} h-14 w-14 rounded-full flex items-center justify-center material-symbols-outlined shadow-[0_0_8px_rgba(0,0,0,0.28)]")}>
            {name}
        </span>
    }
}

#[component]
fn FABLink<'a>(icon: &'a str, text: &'a str, href: String, index: u32, disabled: bool) {
    let translate_y = match index {
        0 => "translate-y-16",
        1 => "translate-y-32",
        2 => "translate-y-48",
        3 => "translate-y-64",
        4 => "translate-y-80",
        5 => "translate-y-96",
        _ => unimplemented!(),
    };

    let cursor = if disabled { "cursor-not-allowed" } else { "" };

    let link_styles = format!("relative z-10 block flex flex-row-reverse items-center gap-4 {translate_y} collapse opacity-0 peer-checked:visible peer-checked:translate-y-0 peer-checked:opacity-100 duration-300 transition-all {cursor}");

    let text_styles = if disabled {
        "bg-black12 text-black26"
    } else {
        "bg-turquoise text-white"
    };

    if disabled {
        rsx! {
            <span class={link_styles}>
                <FABIcon name={icon} styles={text_styles} />
                <span class={format!("{text_styles} px-2 py-1 rounded")}>{text}</span>
            </span>
        }
    } else {
        rsx! {
            <a href={href} class={link_styles}>
                <FABIcon name={icon} styles={text_styles} />
                <span class={format!("{text_styles} px-2 py-1 rounded")}>{text}</span>
            </a>
        }
    }
}

/// Picks an avatar color based on the name.
///
/// This function uses hashing and is thus susceptible to collisions if a nobt contains many names.
fn pick_bg_color(name: &str) -> &'static str {
    let colors = [
        "bg-[#929093]",
        "bg-[#EBDD94]",
        "bg-[#DA8D93]",
        "bg-[#BA99B8]",
        "bg-[#D7B8A3]",
        "bg-[#CD9775]",
        "bg-[#DB8F5B]",
        "bg-[#9E5C5D]",
        "bg-[#CCD0D1]",
        "bg-[#A7CCDE]",
        "bg-[#87A9C5]",
        "bg-[#255993]",
        "bg-[#89BFAF]",
        "bg-[#2EA1B4]",
        "bg-[#8A8A4C]",
        "bg-[#587942]",
    ];

    let mut hasher = DefaultHasher::new();
    name.hash(&mut hasher);
    let index = hasher.finish() as usize;

    colors[index % colors.len()]
}

fn make_initials(name: &str) -> Cow<'_, str> {
    match name.split(' ').collect::<Vec<_>>().as_slice() {
        [] => Cow::Borrowed(""), // TODO: Should never happen
        [first] => match first.len() {
            0 => unreachable!(),
            1 => Cow::Borrowed(&name[..1]),
            _ => Cow::Borrowed(&name[..2]),
        },
        [first, last] => Cow::Owned(format!("{}{}", &first[..1], &last[..1])),
        [first, .., last] => Cow::Owned(format!("{}{}", &first[..1], &last[..1])),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn make_initials_only_firstname() {
        assert_eq!(make_initials("Thomas"), "Th");
    }

    #[test]
    fn make_initials_first_and_lastname() {
        assert_eq!(make_initials("Foo Bar"), "FB");
    }

    #[test]
    fn make_initials_single_letter_first_name() {
        assert_eq!(make_initials("S"), "S");
    }

    #[test]
    fn make_initials_middle_name() {
        assert_eq!(make_initials("Bar Foo Baz"), "BB");
    }
}
