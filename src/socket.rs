use rand::Rng;
use std::{
    net::SocketAddr,
    sync::{
        atomic::{AtomicI64, AtomicU8},
        Arc,
    },
};
use tokio::{
    net::UdpSocket,
    sync::{mpsc::channel, Mutex, Notify, RwLock},
    time::{sleep, timeout},
};

use crate::{
    error::{RaknetError, Result},
    raknet_log_error, raknet_log_info,
};
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::mpsc::{Receiver, Sender};

use crate::{arq::*, packet::*, raknet_log_debug, utils::*};

pub struct RaknetSocket {
    local_addr: SocketAddr,
    peer_addr: SocketAddr,
    user_data_receiver: Arc<Mutex<Receiver<Vec<u8>>>>,
    recvq: Arc<Mutex<RecvQ>>,
    sendq: Arc<RwLock<SendQ>>,
    close_notifier: Arc<tokio::sync::Semaphore>,
    last_heartbeat_time: Arc<AtomicI64>,
    enable_loss: Arc<AtomicBool>,
    loss_rate: Arc<AtomicU8>,
    incomming_notifier: Arc<Notify>,
    sender: Sender<(Vec<u8>, SocketAddr, bool, u8)>,
    drop_notifier: Arc<Notify>,
    raknet_version: u8,
}

impl RaknetSocket {
    
    pub async fn from(
        addr: &SocketAddr,
        s: &Arc<UdpSocket>,
        receiver: Receiver<Vec<u8>>,
        mtu: u16,
        collecter: Arc<Mutex<Sender<SocketAddr>>>,
        raknet_version: u8,
    ) -> Self { panic!("STUB: not implemented") }

    async fn handle(
        frame: &FrameSetPacket,
        peer_addr: &SocketAddr,
        local_addr: &SocketAddr,
        sendq: &RwLock<SendQ>,
        user_data_sender: &Sender<Vec<u8>>,
        incomming_notify: &Notify,
    ) -> Result<bool> { panic!("STUB: not implemented") }

    async fn sendto(
        s: &UdpSocket,
        buf: &[u8],
        target: &SocketAddr,
        enable_loss: bool,
        loss_rate: u8,
    ) -> tokio::io::Result<usize> { panic!("STUB: not implemented") }

    pub async fn connect(addr: &SocketAddr) -> Result<Self> { panic!("STUB: not implemented") }

    pub async fn connect_with_version(addr: &SocketAddr, raknet_version: u8) -> Result<Self> { panic!("STUB: not implemented") }

    fn start_receiver(
        &self,
        s: &Arc<UdpSocket>,
        mut receiver: Receiver<Vec<u8>>,
        user_data_sender: Sender<Vec<u8>>,
    ) { panic!("STUB: not implemented") }

    fn start_sender(
        &self,
        s: &Arc<UdpSocket>,
        mut receiver: Receiver<(Vec<u8>, SocketAddr, bool, u8)>,
    ) { panic!("STUB: not implemented") }

    fn start_tick(&self, s: &Arc<UdpSocket>, collecter: Option<Arc<Mutex<Sender<SocketAddr>>>>) { panic!("STUB: not implemented") }

    pub async fn close(&self) -> Result<()> { panic!("STUB: not implemented") }

    pub async fn ping(addr: &SocketAddr) -> Result<(i64, String)> { panic!("STUB: not implemented") }

    pub async fn send(&self, buf: &[u8], r: Reliability) -> Result<()> { panic!("STUB: not implemented") }

    pub async fn flush(&self) -> Result<()> { panic!("STUB: not implemented") }

    pub async fn recv(&self) -> Result<Vec<u8>> { panic!("STUB: not implemented") }

    pub fn peer_addr(&self) -> Result<SocketAddr> { panic!("STUB: not implemented") }

    pub fn local_addr(&self) -> Result<SocketAddr> { panic!("STUB: not implemented") }

    pub fn raknet_version(&self) -> Result<u8> { panic!("STUB: not implemented") }

    pub fn set_loss_rate(&mut self, stage: u8) { panic!("STUB: not implemented") }

    async fn drop_watcher(&self) { panic!("STUB: not implemented") }
}

impl Drop for RaknetSocket {
    fn drop(&mut self) { panic!("STUB: not implemented") }
}
