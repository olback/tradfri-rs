use {
    std::{net::SocketAddr, io::Write},
    crate::TradfriConnection
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

    pub fn send(&self, data: &[u8]) -> crate::Result<usize> {

        let mut con = TradfriConnection::connect(self.addr, &self.pre_shared_secret)?;
        let len = con.write(data)?;
        Ok(len)

    }


}