use tokio_tungstenite::tungstenite::{
    handshake::server::{ErrorResponse, Request, Response},
    http::HeaderValue,
    Message,
};

use crate::{
    events::sender::connection::close_connection_notify, helpers::constants::ws::WS_AUTH,
    models::user::User,
};

pub fn ws_disconnected(c: User, msg: Message) -> anyhow::Result<bool> {
    if msg.is_close() && msg.is_empty() {
        close_connection_notify(c)?;
        return Ok(true);
    }
    return Ok(false);
}

pub fn ws_header_validation(req: &Request, res: Response) -> Result<Response, ErrorResponse> {
    for (ref header, value) in req.headers() {
        if header.as_str() == WS_AUTH.to_lowercase() {
            println!("* {}: {:?}", header, value);
            match validate_user_id(value) {
                Ok(v) => match v {
                    true => return Ok(res),
                    false => return ws_err(Some("Invalid user id".to_string())),
                },
                Err(e) => return ws_err(Some(e.to_string())),
            }
        }
    }
    ws_err(None)
}

fn ws_err(msg: Option<String>) -> Result<Response, ErrorResponse> {
    Err(Response::builder().status(401).body(msg).unwrap())
}

fn validate_user_id(value: &HeaderValue) -> anyhow::Result<bool> {
    let id = value.to_str()?;
    Ok(id == "u_id")
}
