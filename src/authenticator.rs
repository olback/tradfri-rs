use {
    crate::TradfriConnection,
    serde::Deserialize,
    std::{
        net::SocketAddr
    },
    coap::{CoAPRequest, message::request::Method}
};

const BODY: &'static str = "{\"9090\": \"IDENTITY\"}";

#[derive(Debug, Deserialize)]
struct AuthResponse {
    #[serde(rename = "9091")]
    pre_shared_key: String,
    #[serde(rename = "9029")]
    version: String
}

pub struct TradfriAuthenticator;

impl TradfriAuthenticator {

    pub fn authenticate<A: Into<SocketAddr>>(addr: A, security_code: &str) -> crate::Result<String> {

        let mut con = TradfriConnection::new(addr, b"Client_identity", security_code.as_bytes())?;

        let mut req = CoAPRequest::new();
        req.set_path("15011/9063");
        req.set_method(Method::Post);
        req.message.set_payload(BODY.as_bytes().to_owned());

        con.send(req)?;

        let response = con.receive()?;
        let content: AuthResponse = serde_json::from_slice(&response.message.payload)?;

        Ok(content.pre_shared_key)

    }

}
