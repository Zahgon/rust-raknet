
mod arq;
mod datatype;
pub mod error;
mod fragment;
mod log;
mod packet;
mod server;
mod socket;
mod utils;

pub use crate::arq::Reliability;
pub use crate::log::enable_raknet_log;
pub use crate::server::*;
pub use crate::socket::*;

#[tokio::test]
async fn test_connect() {
    let mut server = RaknetListener::bind(&"127.0.0.1:0".parse().unwrap())
        .await
        .unwrap();
    let local_addr = server.local_addr().unwrap();
    server.listen().await;

    let notify = std::sync::Arc::new(tokio::sync::Notify::new());
    let notify2 = notify.clone();

    tokio::spawn(async move {
        let client1 = server.accept().await.unwrap();
        assert!(client1.local_addr().unwrap() == local_addr);
        client1
            .send(&[0xfe, 2, 3], Reliability::Reliable)
            .await
            .unwrap();
        notify2.notified().await;
    });
    let client2 = RaknetSocket::connect(&local_addr).await.unwrap();
    assert!(client2.peer_addr().unwrap() == local_addr);
    let buf = client2.recv().await.unwrap();
    assert!(buf == vec![0xfe, 2, 3]);

    notify.notify_one();
}

#[tokio::test]
async fn test_send_recv_fragment_data() {
    let mut server = RaknetListener::bind(&"127.0.0.1:0".parse().unwrap())
        .await
        .unwrap();
    let local_addr = server.local_addr().unwrap();
    server.listen().await;

    let notify = std::sync::Arc::new(tokio::sync::Notify::new());
    let notify2 = notify.clone();

    tokio::spawn(async move {
        let client1 = server.accept().await.unwrap();
        assert!(client1.local_addr().unwrap() == local_addr);

        let mut a = vec![3u8; 1000];
        let mut b = vec![2u8; 1000];
        let mut c = vec![0xfe; 1000];
        b.append(&mut a);
        c.append(&mut b);

        client1
            .send(&c, Reliability::ReliableOrdered)
            .await
            .unwrap();

        notify2.notified().await;
    });
    let client2 = RaknetSocket::connect(&local_addr).await.unwrap();
    assert!(client2.peer_addr().unwrap() == local_addr);
    let buf = client2.recv().await.unwrap();
    assert!(buf.len() == 3000);
    assert!(buf[0..1000] == vec![0xfe; 1000]);
    assert!(buf[1000..2000] == vec![2u8; 1000]);
    assert!(buf[2000..3000] == vec![3u8; 1000]);

    notify.notify_one();
}

#[tokio::test]
async fn test_send_recv_more_reliability_type_packet() {
    let mut server = RaknetListener::bind(&"127.0.0.1:0".parse().unwrap())
        .await
        .unwrap();
    let local_addr = server.local_addr().unwrap();
    server.listen().await;

    let notify = std::sync::Arc::new(tokio::sync::Notify::new());
    let notify2 = notify.clone();

    tokio::spawn(async move {
        let client1 = server.accept().await.unwrap();
        assert!(client1.local_addr().unwrap() == local_addr);

        client1
            .send(&[0xfe, 1, 2, 3], Reliability::Unreliable)
            .await
            .unwrap();
        let data = client1.recv().await.unwrap();
        assert!(data == [0xfe, 4, 5, 6].to_vec());

        client1
            .send(&[0xfe, 7, 8, 9], Reliability::UnreliableSequenced)
            .await
            .unwrap();
        let data = client1.recv().await.unwrap();
        assert!(data == [0xfe, 10, 11, 12].to_vec());

        client1
            .send(&[0xfe, 13, 14, 15], Reliability::Reliable)
            .await
            .unwrap();
        let data = client1.recv().await.unwrap();
        assert!(data == [0xfe, 16, 17, 18].to_vec());

        let mut a = vec![3u8; 1000];
        let mut b = vec![2u8; 1000];
        let mut c = vec![0xfe; 1000];
        b.append(&mut a);
        c.append(&mut b);

        client1
            .send(&c, Reliability::ReliableOrdered)
            .await
            .unwrap();

        let buf = client1.recv().await.unwrap();
        assert!(buf.len() == 3000);
        assert!(buf[0..1000] == vec![0xfe; 1000]);
        assert!(buf[1000..2000] == vec![2u8; 1000]);
        assert!(buf[2000..3000] == vec![3u8; 1000]);

        client1
            .send(&[0xfe, 19, 20, 21], Reliability::ReliableSequenced)
            .await
            .unwrap();
        let data = client1.recv().await.unwrap();
        assert!(data == [0xfe, 22, 23, 24].to_vec());

        notify2.notified().await;
    });
    let client2 = RaknetSocket::connect(&local_addr).await.unwrap();
    assert!(client2.peer_addr().unwrap() == local_addr);

    let buf = client2.recv().await.unwrap();
    assert!(buf == [0xfe, 1, 2, 3]);

    client2
        .send(&[0xfe, 4, 5, 6], Reliability::Unreliable)
        .await
        .unwrap();

    let buf = client2.recv().await.unwrap();
    assert!(buf == [0xfe, 7, 8, 9]);

    client2
        .send(&[0xfe, 10, 11, 12], Reliability::UnreliableSequenced)
        .await
        .unwrap();

    let buf = client2.recv().await.unwrap();
    assert!(buf == [0xfe, 13, 14, 15]);

    client2
        .send(&[0xfe, 16, 17, 18], Reliability::Reliable)
        .await
        .unwrap();

    let buf = client2.recv().await.unwrap();
    assert!(buf.len() == 3000);
    assert!(buf[0..1000] == vec![0xfe; 1000]);
    assert!(buf[1000..2000] == vec![2u8; 1000]);
    assert!(buf[2000..3000] == vec![3u8; 1000]);

    let mut a = vec![3u8; 1000];
    let mut b = vec![2u8; 1000];
    let mut c = vec![0xfe; 1000];
    b.append(&mut a);
    c.append(&mut b);

    client2
        .send(&c, Reliability::ReliableOrdered)
        .await
        .unwrap();

    let buf = client2.recv().await.unwrap();
    assert!(buf == [0xfe, 19, 20, 21]);

    client2
        .send(&[0xfe, 22, 23, 24], Reliability::ReliableSequenced)
        .await
        .unwrap();

    notify.notify_one();
}

#[tokio::test]
async fn test_loss_packet1() {
    let notify = std::sync::Arc::new(tokio::sync::Notify::new());
    let notify2 = notify.clone();
    let mut server = RaknetListener::bind(&"127.0.0.1:0".parse().unwrap())
        .await
        .unwrap();
    let local_addr = server.local_addr().unwrap();
    server.listen().await;
    tokio::spawn(async move {
        let mut client1 = server.accept().await.unwrap();
        
        client1.set_loss_rate(8);

        for i in 0..10 {
            let mut flag = vec![0xfe_u8];
            let mut data = vec![i as u8; 2000];
            flag.append(&mut data);
            client1
                .send(&flag, Reliability::ReliableOrdered)
                .await
                .unwrap();

            let data = client1.recv().await.unwrap();
            assert!(data == flag);
        }

        notify2.notified().await;
    });
    let mut client2 = RaknetSocket::connect(&local_addr).await.unwrap();
    
    client2.set_loss_rate(8);

    for i in 0..10 {
        let mut flag = vec![0xfe_u8];
        let mut data = vec![i as u8; 2000];
        flag.append(&mut data);
        client2
            .send(&flag, Reliability::ReliableOrdered)
            .await
            .unwrap();

        let data = client2.recv().await.unwrap();
        assert!(data == flag);
    }
    notify.notify_one();
}

#[tokio::test]
async fn test_loss_packet2() {
    let notify = std::sync::Arc::new(tokio::sync::Notify::new());
    let notify2 = notify.clone();
    let mut server = RaknetListener::bind(&"127.0.0.1:0".parse().unwrap())
        .await
        .unwrap();
    let local_addr = server.local_addr().unwrap();
    server.listen().await;
    tokio::spawn(async move {
        let mut client1 = server.accept().await.unwrap();
        
        client1.set_loss_rate(8);

        for i in 0..10 {
            let mut flag = vec![0xfe_u8];
            let mut data = vec![i as u8; 2000];
            flag.append(&mut data);
            client1
                .send(&flag, Reliability::ReliableOrdered)
                .await
                .unwrap();
        }

        for i in 0..10 {
            let mut flag = vec![0xfe_u8];
            let mut data = vec![i as u8; 2000];
            flag.append(&mut data);
            let data = client1.recv().await.unwrap();
            assert!(data == flag);
        }
        notify2.notified().await;
    });
    let mut client2 = RaknetSocket::connect(&local_addr).await.unwrap();
    
    client2.set_loss_rate(8);

    for i in 0..10 {
        let mut flag = vec![0xfe_u8];
        let mut data = vec![i as u8; 2000];
        flag.append(&mut data);
        client2
            .send(&flag, Reliability::ReliableOrdered)
            .await
            .unwrap();
    }

    for i in 0..10 {
        let mut flag = vec![0xfe_u8];
        let mut data = vec![i as u8; 2000];
        flag.append(&mut data);
        let data = client2.recv().await.unwrap();
        assert!(data == flag);
    }
    notify.notify_one();
}

#[tokio::test]
async fn test_loss_packet_with_sequenced() {
    let notify = std::sync::Arc::new(tokio::sync::Notify::new());
    let notify2 = notify.clone();
    let mut server = RaknetListener::bind(&"127.0.0.1:0".parse().unwrap())
        .await
        .unwrap();
    let local_addr = server.local_addr().unwrap();
    server.listen().await;
    tokio::spawn(async move {
        let mut client1 = server.accept().await.unwrap();
        
        client1.set_loss_rate(8);

        for i in 0..100 {
            let mut flag = vec![0xfe_u8];
            let mut data = vec![i as u8; 20];
            flag.append(&mut data);
            client1
                .send(&flag, Reliability::ReliableSequenced)
                .await
                .unwrap();
        }

        let mut last = 0;
        for i in 0..50 {
            let mut flag = vec![0xfe_u8];
            let mut data = vec![i as u8; 20];
            flag.append(&mut data);
            let data = client1.recv().await.unwrap();
            assert!(data[1] >= last);
            last = data[1];
        }
        notify2.notified().await;
    });
    let mut client2 = RaknetSocket::connect(&local_addr).await.unwrap();
    
    client2.set_loss_rate(8);

    for i in 0..100 {
        let mut flag = vec![0xfe_u8];
        let mut data = vec![i as u8; 20];
        flag.append(&mut data);
        client2
            .send(&flag, Reliability::ReliableSequenced)
            .await
            .unwrap();
    }

    let mut last = 0;
    for i in 0..50 {
        let mut flag = vec![0xfe_u8];
        let mut data = vec![i as u8; 20];
        flag.append(&mut data);
        let data = client2.recv().await.unwrap();
        assert!(data[1] >= last);
        last = data[1];
    }
    notify.notify_one();
}

#[tokio::test]
async fn test_raknet_server_close() {
    for _ in 0..10 {
        let mut server = RaknetListener::bind(&"127.0.0.1:19132".parse().unwrap())
            .await
            .unwrap();
        server.listen().await;
        let client = RaknetSocket::connect(&"127.0.0.1:19132".parse().unwrap())
            .await
            .unwrap();
        let mut a = vec![3u8; 1000];
        let mut b = vec![2u8; 1000];
        let mut c = vec![0xfe; 1000];
        b.append(&mut a);
        c.append(&mut b);
        client.send(&c, Reliability::ReliableOrdered).await.unwrap();

        let client2 = server.accept().await.unwrap();
        let buf = client2.recv().await.unwrap();
        assert!(buf.len() == 3000);
        assert!(buf[0..1000] == vec![0xfe; 1000]);
        assert!(buf[1000..2000] == vec![2u8; 1000]);
        assert!(buf[2000..3000] == vec![3u8; 1000]);

        server.close().await.unwrap();
        client.close().await.unwrap();
    }

    let mut server1 = RaknetListener::bind(&"127.0.0.1:19132".parse().unwrap())
        .await
        .unwrap();
    server1.listen().await;
    server1.close().await.unwrap();
    let mut server2 = RaknetListener::bind(&"127.0.0.1:19132".parse().unwrap())
        .await
        .unwrap();
    server2.listen().await;
}

#[tokio::test]
async fn test_send_recv_full_packet() {
    let mut server = RaknetListener::bind(&"127.0.0.1:0".parse().unwrap())
        .await
        .unwrap();
    let remote_addr = format!("127.0.0.1:{}", server.local_addr().unwrap().port());
    tokio::spawn(async move {
        server.listen().await;
        let client = server.accept().await.unwrap();

        for _ in 0..50 {
            client
                .send(&vec![0xfe; 1000], Reliability::ReliableSequenced)
                .await
                .unwrap();
        }
        client.flush().await.unwrap();

        client.close().await.unwrap();
        server.close().await.unwrap();
    });

    let client = RaknetSocket::connect(&remote_addr.parse().unwrap())
        .await
        .unwrap();

    for _ in 0..50 {
        let buf = client.recv().await.unwrap();
        assert!(buf == [0xfe; 1000]);
    }
}

#[tokio::test]
async fn test_send_recv_with_flush() {
    let mut server = RaknetListener::bind(&"127.0.0.1:0".parse().unwrap())
        .await
        .unwrap();
    let remote_addr = format!("127.0.0.1:{}", server.local_addr().unwrap().port());
    let s1 = std::sync::Arc::new(tokio::sync::Semaphore::new(1));
    let s2 = s1.clone();
    tokio::spawn(async move {
        server.listen().await;
        let client = server.accept().await.unwrap();

        for _ in 0..50 {
            #[allow(unused_must_use)]
            {
                s1.acquire().await.unwrap();
            }
            client
                .send(&vec![0xfe; 1000], Reliability::ReliableSequenced)
                .await
                .unwrap();
            client.flush().await.unwrap();
        }

        client.close().await.unwrap();
        for _ in 0..5 {
            client
                .send(&vec![0xfe; 1000], Reliability::ReliableSequenced)
                .await
                .unwrap();
            match match client.flush().await {
                Ok(_) => panic!("incorrect return"),
                Err(e) => e,
            } {
                error::RaknetError::ConnectionClosed => {}
                _ => panic!("incorrect return"),
            };
        }
        server.close().await.unwrap();
    });

    let client = RaknetSocket::connect(&remote_addr.parse().unwrap())
        .await
        .unwrap();

    for _ in 0..50 {
        let buf = client.recv().await.unwrap();
        assert!(buf == [0xfe; 1000]);
        s2.add_permits(1);
    }
}

