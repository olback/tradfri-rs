use {
    crate::{Device, device_worker::DeviceWorker},
    std::{
        net::{UdpSocket, SocketAddr},
        io::{self, Read, Write}
    },
    udp_dtls::{DtlsConnector, DtlsStream, UdpChannel, ConnectorIdentity, PskIdentity},
    coap::{CoAPRequest, message::{packet::Packet, request::Method}}
};

#[derive(Debug)]
pub struct TradfriConnection {
    stream: DtlsStream<UdpChannel>,
    addr: SocketAddr,
    pre_shared_key: String
}

impl TradfriConnection {

    pub(crate) fn new<A: Into<SocketAddr>>(addr: A, identity: &[u8], key: &[u8]) -> crate::Result<Self> {

        let connector = DtlsConnector::builder()
            .danger_accept_invalid_certs(true)
            .danger_accept_invalid_hostnames(true)
            .use_sni(false)
            .add_cipher("PSK-AES128-CCM8")
            .identity(ConnectorIdentity::Psk(PskIdentity::new(identity, key)))
            .min_protocol_version(Some(udp_dtls::Protocol::Dtlsv12))
            .max_protocol_version(Some(udp_dtls::Protocol::Dtlsv12))
            .build()?;

        let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
        socket.set_nonblocking(false).unwrap();

        let addr = addr.into();

        let client_channel = UdpChannel {
            socket,
            remote_addr: addr.clone()
        };

        Ok(Self {
            stream: connector.connect("", client_channel)?,
            addr: addr.into(),
            pre_shared_key: String::from_utf8_lossy(key).to_owned().to_string()
        })

    }

    pub fn connect<A: Into<SocketAddr>>(addr: A, pre_shared_key: &str) -> crate::Result<Self> {

        Self::new(addr, b"IDENTITY", pre_shared_key.as_bytes())

    }

    pub fn devices(&mut self) -> crate::Result<Vec<Device>> {

        let mut req = CoAPRequest::new();
        req.set_path("15001");
        req.set_method(Method::Get);

        let data = req.message.to_bytes()?;
        self.write(&data[..])?;

        let mut buf = [0u8; 4096];
        let len = self.read(&mut buf)?;

        let packet = Packet::from_bytes(&buf[0..len])?;

        let device_ids: Vec<u32> = serde_json::from_slice(&packet.payload)?;
        let mut devices = Vec::<Device>::with_capacity(device_ids.len());

        for device_id in device_ids {

            let mut req = coap::CoAPRequest::new();
            req.set_path(&format!("15001/{}", device_id));
            req.set_method(Method::Get);

            let data = req.message.to_bytes()?;
            self.write(&data[..])?;

            let mut buf = [0u8; 4096];
            let len = self.read(&mut buf)?;

            let packet = Packet::from_bytes(&buf[0..len])?;

            match Device::new(self.worker(), &packet.payload) {
                Ok(device) => devices.push(device),
                Err(e) => eprintln!("{:?}", e)
            };

        }

        Ok(devices)

    }

    fn worker(&self) -> DeviceWorker {

        DeviceWorker::new(self.addr, self.pre_shared_key.clone())

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


