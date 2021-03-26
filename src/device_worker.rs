use {crate::TradfriConnection, coap::CoAPRequest, std::net::IpAddr};

#[derive(Debug, Clone)]
pub struct DeviceWorker {
    addr: IpAddr,
    key_name: String,
    pre_shared_secret: String,
}

impl DeviceWorker {
    pub fn new<A: Into<IpAddr>, K: Into<String>, S: Into<String>>(
        addr: A,
        key_name: K,
        pre_shared_secret: S,
    ) -> Self {
        Self {
            addr: addr.into(),
            key_name: key_name.into(),
            pre_shared_secret: pre_shared_secret.into(),
        }
    }

    pub fn send(&self, req: CoAPRequest) -> crate::Result<usize> {
        let mut con =
            TradfriConnection::connect(self.addr, &self.key_name, &self.pre_shared_secret)?;
        let len = con.send(req)?;
        Ok(len)
    }
}
