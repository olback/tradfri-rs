use crate::impl_from;

#[derive(Debug)]
pub struct Error {
    cause: String
}

impl Error {

    pub fn new<C: Into<String>>(cause: C) -> Self {
        Self {
            cause: cause.into()
        }
    }

}

impl_from!(udp_dtls::Error);
impl_from!(coap::message::packet::PackageError);
impl_from!(coap::message::packet::ParseError);
impl_from!(std::io::Error);
impl_from!(serde_json::Error);

impl From<udp_dtls::HandshakeError<udp_dtls::UdpChannel>> for Error {
    fn from(err: udp_dtls::HandshakeError<udp_dtls::UdpChannel>) -> Self {
        Self {
            cause: format!("{:?}", err)
        }
    }
}

