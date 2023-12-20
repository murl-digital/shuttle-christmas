use std::{sync::atomic::AtomicI64, collections::HashMap};

use actix_web::{get, HttpRequest, web::{self, ServiceConfig}, Result, HttpResponse, post};
use actix_ws::Message;
use serde::{Serialize, Deserialize};
use tokio::sync::{broadcast::Sender, Mutex};


#[derive(Serialize, Clone)]
pub struct Tweet {
    user: String,
    message: String
}

#[derive(Deserialize)]
struct SendTweet {
    message: String
}

#[derive(Debug)]
pub struct Day19State {
    pub views: AtomicI64,
    pub rooms: Mutex<HashMap<i64, Sender<Tweet>>>
}

#[post("/19/reset")]
async fn reset(state: web::Data<Day19State>) -> HttpResponse {
    state.views.store(0, std::sync::atomic::Ordering::SeqCst);
    HttpResponse::Ok().finish()
}

#[get("/19/views")]
async fn views(state: web::Data<Day19State>) -> String {
    state.views.load(std::sync::atomic::Ordering::SeqCst).to_string()
}

#[get("/19/ws/room/{number}/user/{user}")]
async fn tweet(req: HttpRequest, body: web::Payload, path: web::Path<(i64, String)>, state: web::Data<Day19State>) -> Result<HttpResponse> {
    let (room_id, username) = path.into_inner();
    let mut rooms = state.rooms.lock().await;
    let sender = match rooms.get(&room_id) {
        Some(s) => s.clone(),
        None => {
            let sender = Sender::new(1024);
            let sender_clone = sender.clone();
            rooms.insert(room_id, sender);
            sender_clone
        }
    };
    // dropping here to ensure the mutex is locked as little as possible
    drop(rooms);
    let mut rx = sender.subscribe();

    let (response, og_session, mut msg_stream) = actix_ws::handle(&req, body)?;
    
    let mut session = og_session.clone();
    actix_web::rt::spawn(async move {
        let mut rx_task = actix_web::rt::spawn(async move {
            while let Some(Ok(msg)) = msg_stream.recv().await {
                match msg {
                    Message::Ping(bytes) => {
                        if session.pong(&bytes).await.is_err() {
                            return;
                        }
                    },
                    Message::Text(msg) => {
                        if let Ok(tweet) = serde_json::from_str::<SendTweet>(&msg) {
                            if tweet.message.len() <= 128 {
                                let _ = sender.send(Tweet { user: username.clone(), message: tweet.message });
                            }
                        }
                    },
                    Message::Close(_) => {
                        break;
                    },
                    _ => {}
                }
            }

            let _ = session.close(None).await;
        });

        let mut session = og_session.clone();
        let mut tx_task = actix_web::rt::spawn(async move {
            while let Ok(tweet) = rx.recv().await {
                if session.text(serde_json::to_string(&tweet).unwrap()).await.is_err() {
                    return;
                } else {
                    // don't count a view if the message can't be sent
                    state.views.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                }
            }
        });

        tokio::select! {
            _ = (&mut tx_task) => rx_task.abort(),
            _ = (&mut rx_task) => tx_task.abort()
        }
    });

    Ok(response)
}


#[get("/19/ws/ping")]
async fn ping(req: HttpRequest, body: web::Payload) -> Result<HttpResponse> {
    let (response, mut session, mut msg_stream) = actix_ws::handle(&req, body)?;

    actix_web::rt::spawn(async move {
        let mut game_started = false;

        while let Some(Ok(msg)) = msg_stream.recv().await {
            match msg {
                Message::Ping(bytes) => {
                    if session.pong(&bytes).await.is_err() {
                        return;
                    }
                }
                Message::Text(txt) => {
                    match std::str::from_utf8(txt.as_bytes()).unwrap() {
                        "serve" if !game_started => {
                            game_started = true;
                        }
                        "ping" if game_started => {
                            if session.text("pong").await.is_err() {
                                return;
                            }
                        },
                        _ => {}
                    }
                }
                Message::Close(_) => {
                    break;
                }
                _ => {}
            }
        }

        let _ = session.close(None).await;
    });

    Ok(response)
}

pub fn day19(cfg: &mut ServiceConfig, data: web::Data<Day19State>) {
    cfg.app_data(data);
    cfg.service(ping);
    cfg.service(reset);
    cfg.service(views);
    cfg.service(tweet);
}