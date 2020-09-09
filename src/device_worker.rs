use {
    std::net::SocketAddr,
    crate::TradfriConnection,
    coap::CoAPRequest
};

#[derive(Debug, Clone)]
pub struct DeviceWorker {
    addr: SocketAddr,
    pre_shared_secret: String
}

impl DeviceWorker {

    pub fn new<A: Into<SocketAddr>, S: Into<String>>(addr: A, pre_shared_secret: S) -> Self {
        Self {
            addr: addr.into(),
            pre_shared_secret: pre_shared_secret.into()
        }
    }

    pub fn send(&self, req: CoAPRequest) -> crate::Result<usize> {

        let mut con = TradfriConnection::connect(self.addr, &self.pre_shared_secret)?;
        let len = con.send(req)?;
        Ok(len)

    }

}
