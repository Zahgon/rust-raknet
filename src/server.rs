use std::collections::HashMap;
use std::{net::SocketAddr, sync::Arc};
use tokio::net::UdpSocket;
use tokio::sync::mpsc::channel;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::{Mutex, Notify};

use crate::error::{RaknetError, Result};
use crate::packet::*;
use crate::utils::*;
use crate::{raknet_log_debug, raknet_log_error, socket::*};

const SERVER_NAME: &str = "Rust Raknet Server";
const MAX_CONNECTION: u32 = 99999;

type SessionSender = (i64, Sender<Vec<u8>>);

pub struct RaknetListener {
    motd: String,
    socket: Option<Arc<UdpSocket>>,
    guid: u64,
    listened: bool,
    connection_receiver: Receiver<RaknetSocket>,
    connection_sender: Sender<RaknetSocket>,
    sessions: Arc<Mutex<HashMap<SocketAddr, SessionSender>>>,
    close_notifier: Arc<tokio::sync::Semaphore>,
    all_session_closed_notifier: Arc<Notify>,
    drop_notifier: Arc<Notify>,
    version_map: Arc<Mutex<HashMap<String, u8>>>,
}

impl RaknetListener {
    
    pub async fn bind(sockaddr: &SocketAddr) -> Result<Self> { panic!("STUB: not implemented") }

    pub async fn from_std(s: std::net::UdpSocket) -> Result<Self> { panic!("STUB: not implemented") }

    async fn start_session_collect(
        &self,
        socket: &Arc<UdpSocket>,
        sessions: &Arc<Mutex<HashMap<SocketAddr, SessionSender>>>,
        mut collect_receiver: Receiver<SocketAddr>,
    ) { panic!("STUB: not implemented") }

    pub async fn listen(&mut self) { panic!("STUB: not implemented") }

    pub async fn accept(&mut self) -> Result<RaknetSocket> { panic!("STUB: not implemented") }

    pub async fn set_motd(
        &mut self,
        server_name: &str,
        max_connection: u32,
        mc_protocol_version: &str,
        mc_version: &str,
        game_type: &str,
        port: u16,
    ) { panic!("STUB: not implemented") }

    pub async fn get_motd(&self) -> String { panic!("STUB: not implemented") }

    pub fn local_addr(&self) -> Result<SocketAddr> { panic!("STUB: not implemented") }

    pub async fn close(&mut self) -> Result<()> { panic!("STUB: not implemented") }

    pub fn set_full_motd(&mut self, motd: String) -> Result<()> { panic!("STUB: not implemented") }

    pub async fn get_peer_raknet_version(&self, peer: &SocketAddr) -> Result<u8> { panic!("STUB: not implemented") }

    async fn drop_watcher(&self) { panic!("STUB: not implemented") }
}

impl Drop for RaknetListener {
    fn drop(&mut self) { panic!("STUB: not implemented") }
}
