use std::net::SocketAddr;
use anyhow::{Context, Result};
use axum::extract::Path;
use axum::response::{IntoResponse, Response};
use axum::Router;
use axum::routing::get;
use axum::routing::post;

const STYLES: &str = include_str!(concat!(env!("OUT_DIR"), "/style.css"));

#[tokio::main]
async fn main() -> Result<()> {
    let port = std::env::var("PORT").as_deref().unwrap_or("3000").parse().context("failed to parse port")?;

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
    NobtTemplate {
        title: "Swedish Shenanigans".to_string(),
        total: format!("EUR 1,521.00"),
        num_participants: 4,
        expenses: vec![
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
        ],
        balances_url: format!("/{nobt_id}/balances"),
    }
}

async fn balances(Path(nobt_id): Path<String>) -> impl IntoResponse {
    BalancesTemplate {
        title: "Swedish Shenanigans".to_string(),
        nobt_url: format!("/{nobt_id}"),
        balances: vec![
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
        ],
    }
}

async fn expense(Path((nobt_id, expense_id)): Path<(String, u64)>) -> impl IntoResponse {
    ExpenseTemplate {
        title: "Swedish Shenanigans".to_string(),
        name: "Flughafen Essen".to_string(),
        nobt_url: format!("/{nobt_id}"),
        deleted: false,
        delete_url: format!("/{nobt_id}/{expense_id}/delete"),
        debtee_initials: "TH".to_string(),
        debtee_name: "Thomas".to_string(),
        added_on: "28 August 2022".to_string(),
        total: "EUR 39.00".to_string(),
        debtors: vec![
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
        ],
    }
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

#[derive(askama::Template)]
#[template(path = "nobt.html")]
struct NobtTemplate {
    title: String,
    total: String,
    num_participants: u32,
    expenses: Vec<ExpenseItem>,
    balances_url: String,
}

#[derive(askama::Template)]
#[template(path = "expense.html")]
struct ExpenseTemplate {
    title: String,
    name: String,
    nobt_url: String,
    deleted: bool,
    delete_url: String,
    debtee_initials: String,
    debtee_name: String,
    added_on: String,
    total: String,
    debtors: Vec<DebtorItem>,
}

#[derive(askama::Template)]
#[template(path = "balances.html")]
struct BalancesTemplate {
    title: String,
    nobt_url: String,
    balances: Vec<BalanceItem>,
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
    is_negative: bool
}

struct ExpenseItem {
    description: String,
    amount: String,
    url: String,
    deleted: bool,
}
