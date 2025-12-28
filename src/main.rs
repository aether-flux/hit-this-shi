use std::{hash::{DefaultHasher, Hash, Hasher}, net::SocketAddr};

use axum::{extract::ConnectInfo, http::{HeaderMap, StatusCode}, response::IntoResponse, routing::get, Router, ServiceExt};

async fn special_quote(headers: HeaderMap, ConnectInfo(addr): ConnectInfo<SocketAddr>) -> impl IntoResponse {
    // collection of lines
    let lines: Vec<&'static str> = vec![
        "You are stuck with this message. Refreshing ain't gonna help you, lil bro.",
        "This message has been deleted.",
        "Cheer up dude. Are you seriously gonna step into a new year looking like that?",
        "If someone sent you this link, just accept your fate. You are gonna be annoyed this year as well.",
        "Did you expect some motivational quote? Too bad, you are on your own lil bro.",
        "This message was written in your fate. You can't do anything about it. Accept your fate.",
        "Looks boring? Can't be more boring than you at least.\n\nNevermind its new year so I won't be mean. Happy new year, my dude!",
        "Keep all the shit of 2025 in 2025, and start 2026 with a clean slate. The new year doesn't need your old problems.",
        "You have been a very naughty kid this year. No message for you.",
        "Touch some grass, lil bro. How long has it been since you were sitting on your screen?",
        "Aaaahhh my weakness!! A stupid person!!! Noooooo!!!!! (anyways, happy new year)",
        "F*ck around and find out. This message is the consequence of your actions. Refreshing won't help anymore.",
        "Nuh uh.",
        "How long have you even been sitting in front of your screen? Go away dude, socialize or something!",
        "Were you excited about your message? Well guess what, this is all that you get. And btw, refreshing will not help.",
        "One last plot twist for 2025. Do it. This is a sign. Just do it for the plot.",
        "Yo, fellow overthinker! How are you doing, dude?",
        "Statistically speaking, this message was always meant for you to see.",
        "Its the new year dude. Why are you here? Go celebrate or something.",
        "This server is doing fine. You, however... Get your shit together, its a new freaking year, you know!",
        "Nothing important was supposed to happen here. Go back now.",
        "There is no deep meaning. This is all there is.",
        "I just wanna tell you one thing, so sit down and brace yourself:\n\"Skill issue\"\n\nHappy new year btw",
        "This message was assigned to you by Rust 🦀 at runtime. You can not go against its flow.",
        "You have reached the wrong place at the right time.",
        "You have reached the right place at the wrong time.",
        "If this annoyed you even a little, then it's working as intended.",
        "You could be doing literally anything else right now, but look at you looking at plain text responses.",
        "All choices you have ever made in your entire life till now have all led to you reading this message. Accept your fate.",
        "Drink water.",
    ];

    // extract IP
    let ip = addr.ip().to_string();

    // extract user-agent
    let ua = headers.get("user-agent").and_then(|v| v.to_str().ok()).unwrap_or("unknown");

    // build fingerprint
    let fingerprint = format!("{} {}", ip, ua);

    // hash fingerprint
    let mut hasher = DefaultHasher::new();
    fingerprint.hash(&mut hasher);
    let hash = hasher.finish();

    // pick response
    let idx = (hash % lines.len() as u64) as usize;
    let response = format!("{}\n", lines[idx]);

    // return plain text
    (
        StatusCode::OK,
        [("Content-Type", "text/plain; charset=utf-8")],
        response,
    )

    // "Hello, world!"
}

// #[shuttle_runtime::main]
// async fn main() -> shuttle_axum::ShuttleAxum {
//     let router = Router::new().route("/", get(special_quote));
//
//     Ok(router.into())
// }

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(special_quote));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);

    // axum::Server::bind(&addr)
    //     .serve(app.into_make_service_with_connect_info::<SocketAddr>())
    //     .await
    //     .unwrap();

    // let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
