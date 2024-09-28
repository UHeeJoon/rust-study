
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use aho_corasick::{AhoCorasick, AhoCorasickBuilder, AhoCorasickKind};
use futures_util::{SinkExt, StreamExt};
use lazy_static::lazy_static;
use tokio::sync::mpsc;
use warp::Filter;
use warp::ws::{Message, WebSocket};

#[tokio::main]
async fn main() {
    // WebSocket을 처리하는 라우트
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(handle_connection)
        });

    // 서버 실행
    println!("WebSocket server running on ws://127.0.0.1:9090/ws");
    warp::serve(ws_route).run(([127, 0, 0, 1], 9090)).await;
}

// 메시지 처리 함수
async fn process_message(text: String) -> String {
    // 텍스트 검증 및 수정
    let validated_text = validate_text(&text);
    println!("output message: {}", validated_text);
    validated_text
}

lazy_static! {
    static ref NEXT_USERID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
    static ref CLIENTS: Arc<Mutex<HashMap<usize, mpsc::UnboundedSender<Result<Message, warp::Error>>>>> = Arc::new(Mutex::new(HashMap::new()));
}

// 연결 처리
async fn handle_connection(ws: WebSocket) {
    let (mut tx, mut rx) = ws.split();

    // mpsc 채널 생성 (무제한 버퍼)
    let (msg_tx, mut msg_rx) = mpsc::unbounded_channel();

    let my_id = NEXT_USERID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

    // 클라이언트 등록
    {
        let mut clients = CLIENTS.lock().unwrap();
        clients.insert(my_id, msg_tx);
    }

    // 클라이언트 메시지를 수신하고 브로드캐스트하는 Task
    let clients = CLIENTS.clone();
    tokio::spawn(async move {
        while let Some(Ok(message)) = rx.next().await {
            if let Ok(text) = message.to_str() {
                let validated_text = process_message(text.to_string()).await;
                let broadcast_message = Message::text(validated_text);
                broadcast_msg(broadcast_message, &clients).await;
            }
        }

        // 클라이언트 등록 해제
        {
            let mut clients = clients.lock().unwrap();
            clients.remove(&my_id);
        }
    });

    // 클라이언트로부터 전송된 메시지를 수신하여 처리
    while let Some(msg) = msg_rx.recv().await {
        if let Ok(msg) = msg {
            if let Err(e) = tx.send(msg).await {  // Unwrap the Result from msg
                eprintln!("Error sending message: {}", e);
            }
        }
    }
}

// 모든 클라이언트에게 메시지를 전송하는 함수
async fn broadcast_msg(msg: Message, clients: &Arc<Mutex<HashMap<usize, mpsc::UnboundedSender<Result<Message, warp::Error>>>>>) {
    let clients = clients.lock().unwrap();
    for (_, tx) in clients.iter() {
        if let Err(e) = tx.send(Ok(msg.clone())) {
            eprintln!("Failed to send message: {}", e);
        }
    }
}

lazy_static! {
    static ref AC: AhoCorasick = AhoCorasickBuilder::new()
        .kind(Option::from(AhoCorasickKind::DFA))
        .build(&["바보", "멍청이", "벌레", "html은 언어다"])
        .expect("Error creating AhoCorasick");
}

// 텍스트 검증 및 교체
fn validate_text(text: &str) -> String {
    // 패턴과 대체 문자열 정의
    let replace_with = &["착한 아이", "똑똑한 친구", "~~", "용납할 수 없는 발언"];

    // 결과를 담을 버퍼
    let mut result = Vec::with_capacity(text.len());

    // 텍스트를 교체
    if let Err(e) = AC.try_stream_replace_all(text.as_bytes(), &mut result, replace_with) {
        eprintln!("stream_replace_all failed: {:?}", e);
        return text.to_string(); // 원래 텍스트 반환
    }
    // Vec<u8>를 String으로 변환
    String::from_utf8(result).unwrap_or_default()
}