use std::net::Ipv4Addr;
use std::{io, time::Duration};

use crate::{pdu, SnmpError, SnmpMessageType, SnmpPdu, SnmpResult, Value};
use tokio::net::{ToSocketAddrs, UdpSocket};
use tokio::time::timeout;

const BUFFER_SIZE: usize = 4096;

pub struct AsyncSession {
    socket: UdpSocket,
    community: Vec<u8>,
    timeout: Duration,
    req_id: i32,
    send_pdu: pdu::Buf,
    recv_buf: [u8; BUFFER_SIZE],
}

impl AsyncSession {
    pub async fn new<SA>(
        destination: SA,
        community: &[u8],
        timeout: Duration,
        req_id: i32,
    ) -> io::Result<Self>
    where
        SA: ToSocketAddrs,
    {
        let socket = UdpSocket::bind((Ipv4Addr::new(0, 0, 0, 0), 0)).await?;
        socket.connect(destination).await?;

        Ok(AsyncSession {
            socket,
            community: community.to_vec(),
            timeout,
            req_id,
            send_pdu: pdu::Buf::default(),
            recv_buf: [0; 4096],
        })
    }

    async fn send_and_recv(&mut self) -> SnmpResult<usize> {
        if self.socket.send(&self.send_pdu[..]).await.is_err() {
            return Err(SnmpError::SendError);
        }

        let Ok(len) = self.recv_with_timeout().await else {
            return Err(SnmpError::ReceiveError);
        };

        Ok(len)
    }

    async fn recv_with_timeout(&mut self) -> SnmpResult<usize> {
        let Ok(res) = timeout(self.timeout, self.socket.recv(&mut self.recv_buf[..])).await else {
            return Err(SnmpError::ReceiveError);
        };

        let Ok(usize) = res else {
            return Err(SnmpError::ReceiveError);
        };

        Ok(usize)
    }

    pub async fn get(&mut self, name: &[u32]) -> SnmpResult<SnmpPdu> {
        pdu::build_get(
            self.community.as_slice(),
            self.req_id,
            name,
            &mut self.send_pdu,
        );

        let recv_len = self.send_and_recv().await?;
        let req_id = self.req_id;
        self.req_id = self.req_id.wrapping_add(1);

        let pdu_bytes = &self.recv_buf[..recv_len];
        let resp = SnmpPdu::from_bytes(pdu_bytes)?;

        if resp.message_type != SnmpMessageType::Response {
            return Err(SnmpError::AsnWrongType);
        }

        if resp.req_id != req_id {
            return Err(SnmpError::RequestIdMismatch);
        }

        if resp.community != &self.community[..] {
            return Err(SnmpError::CommunityMismatch);
        }

        Ok(resp)
    }

    pub async fn getnext(&mut self, name: &[u32]) -> SnmpResult<SnmpPdu> {
        pdu::build_getnext(
            self.community.as_slice(),
            self.req_id,
            name,
            &mut self.send_pdu,
        );

        let recv_len = self.send_and_recv().await?;
        let req_id = self.req_id;
        self.req_id = self.req_id.wrapping_add(1);

        let pdu_bytes = &self.recv_buf[..recv_len];
        let resp = SnmpPdu::from_bytes(pdu_bytes)?;

        if resp.message_type != SnmpMessageType::Response {
            return Err(SnmpError::AsnWrongType);
        }

        if resp.req_id != req_id {
            return Err(SnmpError::RequestIdMismatch);
        }

        if resp.community != &self.community[..] {
            return Err(SnmpError::CommunityMismatch);
        }

        Ok(resp)
    }

    pub async fn getbulk(
        &mut self,
        names: &[&[u32]],
        non_repeaters: u32,
        max_repetitions: u32,
    ) -> SnmpResult<SnmpPdu> {
        pdu::build_getbulk(
            self.community.as_slice(),
            self.req_id,
            names,
            non_repeaters,
            max_repetitions,
            &mut self.send_pdu,
        );

        let recv_len = self.send_and_recv().await?;
        let req_id = self.req_id;
        self.req_id = self.req_id.wrapping_add(1);

        let pdu_bytes = &self.recv_buf[..recv_len];
        let resp = SnmpPdu::from_bytes(pdu_bytes)?;

        if resp.message_type != SnmpMessageType::Response {
            return Err(SnmpError::AsnWrongType);
        }

        if resp.req_id != req_id {
            return Err(SnmpError::RequestIdMismatch);
        }

        if resp.community != &self.community[..] {
            return Err(SnmpError::CommunityMismatch);
        }

        Ok(resp)
    }

    /// # Panics if any of the values are not one of these supported types:
    ///   - `Boolean`
    ///   - `Null`
    ///   - `Integer`
    ///   - `OctetString`
    ///   - `ObjectIdentifier`
    ///   - `IpAddress`
    ///   - `Counter32`
    ///   - `Unsigned32`
    ///   - `Timeticks`
    ///   - `Opaque`
    ///   - `Counter64`
    pub async fn set(&mut self, values: &[(&[u32], Value<'_>)]) -> SnmpResult<SnmpPdu> {
        pdu::build_set(
            self.community.as_slice(),
            self.req_id,
            values,
            &mut self.send_pdu,
        );

        let recv_len = self.send_and_recv().await?;
        let req_id = self.req_id;
        self.req_id = self.req_id.wrapping_add(1);

        let pdu_bytes = &self.recv_buf[..recv_len];
        let resp = SnmpPdu::from_bytes(pdu_bytes)?;

        if resp.message_type != SnmpMessageType::Response {
            return Err(SnmpError::AsnWrongType);
        }

        if resp.req_id != req_id {
            return Err(SnmpError::RequestIdMismatch);
        }

        if resp.community != &self.community[..] {
            return Err(SnmpError::CommunityMismatch);
        }

        Ok(resp)
    }
}
