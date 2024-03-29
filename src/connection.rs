use {
    crate::{device_worker::DeviceWorker, Device, Error},
    coap::{
        message::{
            packet::{ObserveOption, Packet},
            request::Method,
            response::{CoAPResponse, Status},
        },
        CoAPRequest, IsMessage,
    },
    std::{
        io::{self, Read, Write},
        net::{IpAddr, SocketAddr, UdpSocket},
        time::Duration,
    },
    udp_dtls::{ConnectorIdentity, DtlsConnector, DtlsStream, PskIdentity, UdpChannel},
};

const TF_PORT: u16 = 5684;

#[derive(Debug)]
pub struct TradfriConnection {
    stream: DtlsStream<UdpChannel>,
    addr: IpAddr,
    key_name: String,
    pre_shared_key: String,
}

impl TradfriConnection {
    pub fn new<A: Into<IpAddr>>(addr: A, identity: &[u8], key: &[u8]) -> crate::Result<Self> {
        Self::new_with_timeout(addr, identity, key, None)
    }

    pub fn new_with_timeout<A: Into<IpAddr>>(
        addr: A,
        identity: &[u8],
        key: &[u8],
        timeout: Option<u64>,
    ) -> crate::Result<Self> {
        let connector = DtlsConnector::builder()
            .danger_accept_invalid_certs(true)
            .danger_accept_invalid_hostnames(true)
            .use_sni(false)
            .add_cipher("PSK-AES128-CCM8")
            .identity(ConnectorIdentity::Psk(PskIdentity::new(identity, key)))
            .min_protocol_version(Some(udp_dtls::Protocol::Dtlsv12))
            .max_protocol_version(Some(udp_dtls::Protocol::Dtlsv12))
            .build()?;

        let addr = addr.into();
        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        socket.set_nonblocking(false).unwrap();
        if let Some(timeout) = timeout {
            socket.set_read_timeout(Some(Duration::from_secs(timeout)))?;
            socket.set_write_timeout(Some(Duration::from_secs(timeout)))?;
        }

        let client_channel = UdpChannel {
            socket,
            remote_addr: SocketAddr::new(addr.clone(), TF_PORT),
        };

        Ok(Self {
            stream: connector.connect("", client_channel)?,
            addr: addr.into(),
            key_name: String::from_utf8_lossy(identity).to_owned().to_string(),
            pre_shared_key: String::from_utf8_lossy(key).to_owned().to_string(),
        })
    }

    pub fn devices(&mut self) -> crate::Result<Vec<Device>> {
        let mut req = CoAPRequest::new();
        req.set_path("15001");
        req.set_method(Method::Get);

        self.send(req)?;

        let response = self.receive()?;

        let device_ids: Vec<u32> = serde_json::from_slice(&response.message.payload)?;
        let mut devices = Vec::<Device>::with_capacity(device_ids.len());

        for device_id in device_ids {
            let mut req = coap::CoAPRequest::new();
            req.set_path(&format!("15001/{}", device_id));
            req.set_method(Method::Get);

            self.send(req)?;

            let response = self.receive()?;

            match Device::new(self.worker(), &response.message.payload) {
                Ok(device) => devices.push(device),
                Err(e) => eprintln!("{:?}", e),
            };
        }

        Ok(devices)
    }

    pub fn observe<F>(&mut self, resource_path: &str, cb: F) -> crate::Result<()>
    where
        F: Fn(Packet),
    {
        // Mostly stolen from the coap crate

        let mut message_id = 0u16;
        let mut req = CoAPRequest::new();
        req.set_path(resource_path);
        req.set_observe(vec![ObserveOption::Register as u8]);
        req.set_message_id(Self::gen_message_id(&mut message_id));

        self.send(req)?;

        let response = self.receive()?;
        if *response.get_status() != Status::Content {
            return Err(Error::new("Resource not found"));
        }

        loop {
            let res = self.receive()?;
            cb(res.message);
        }
    }

    pub fn send(&mut self, req: CoAPRequest) -> crate::Result<usize> {
        Ok(self.write(&req.message.to_bytes()?)?)
    }

    pub fn receive(&mut self) -> crate::Result<CoAPResponse> {
        let mut buf = [0u8; crate::BUF_SIZE];
        let len = self.read(&mut buf)?;
        let packet = Packet::from_bytes(&buf[0..len])?;

        Ok(CoAPResponse { message: packet })
    }

    pub fn set_timeout(&mut self, secs: Option<u64>) -> crate::Result<()> {
        self.stream
            .get_mut()
            .socket
            .set_read_timeout(secs.map(|v| Duration::from_secs(v)))?;
        self.stream
            .get_mut()
            .socket
            .set_write_timeout(secs.map(|v| Duration::from_secs(v)))?;
        Ok(())
    }

    fn gen_message_id(message_id: &mut u16) -> u16 {
        (*message_id) += 1;
        return *message_id;
    }

    fn worker(&self) -> DeviceWorker {
        DeviceWorker::new(
            self.addr,
            self.key_name.clone(),
            self.pre_shared_key.clone(),
        )
    }
}

impl Read for TradfriConnection {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.stream.read(buf)
    }
}

impl Write for TradfriConnection {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stream.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.stream.flush()
    }
}
