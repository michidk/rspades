use sprot::msg::{
    model::PlayerId,
    msg::{ChatMessage, Msg, StateData, VersionHandshakeInit},
    MessageKind,
};

mod addr;

use anyhow::Context;
use enet::*;
use std::time::{Duration, SystemTime};

use addr::GetSocketAddress;

fn main() -> anyhow::Result<()> {
    let enet = Enet::new().context("could not initialize ENet")?;

    // http://services.buildandshoot.com/serverlist.json
    // Set the target server address
    let server_addr = "aos://1124169524:32887".get_address();
    println!("Connecting to {}:{}", server_addr.ip(), server_addr.port());

    // parameters: https://github.com/JStalnac/SharpSpades/blob/master/SharpSpades/Server.cs#L116
    // not: https://github.com/DryByte/AoS.js/blob/9b8e3559a4d7b97008ec289cfc36dd182f0ead04/src/client/BaseClient.js#L64
    let mut host = enet
        .create_host::<()>(
            None,
            32,
            ChannelLimit::Limited(1),
            BandwidthLimit::Unlimited,
            BandwidthLimit::Unlimited,
        )
        .context("could not create host")?;

    // user data: 3 for v0.75, 4 for v0.76
    host.connect(&server_addr, 1, 3).context("connect failed")?;

    host.compress_with_range_coder();

    let _peer_id = loop {
        let e = host
            .service(Duration::from_secs(1))
            .expect("service failed");

        let e = match e {
            Some(ev) => ev,
            _ => continue,
        };

        println!("[client] event: {:#?}", e);

        match e.kind() {
            EventKind::Connect => break e.peer_id(),
            EventKind::Disconnect { data } => {
                println!(
                    "connection NOT successful, peer: {:?}, reason: {}",
                    e.peer_id(),
                    data
                );
                std::process::exit(0);
            }
            EventKind::Receive { .. } => {
                anyhow::bail!("unexpected Receive-event while waiting for connection")
            }
        };
    };

    // // send a "hello"-like packet
    // let peer = host.peer_mut(peer_id).unwrap();
    // peer.send_packet(
    //     Packet::new(b"harro".to_vec(), PacketMode::ReliableSequenced).unwrap(),
    //     1,
    // )
    // .context("sending packet failed")?;

    // disconnect after all outgoing packets have been sent.
    // peer.disconnect_later(5);
    let mut my_player_id = PlayerId(255);

    loop {
        let e = host
            .service(Duration::from_secs(1))
            .context("service failed");
        if let Ok(Some(mut e)) = e {
            match e.kind() {
                EventKind::Receive { packet, .. } => {
                    println!(">> [{}]", packet.data()[0]);

                    let start = SystemTime::now();
                    let msg = match Msg::parse_server(packet.data()) {
                        Ok(msg) => msg,
                        Err(err) => {
                            eprintln!("FAILED @ {:?}: {:?}", err, packet.data());
                            //std::process::exit(-1);
                            continue;
                        }
                    };
                    let elapsed = start.elapsed();

                    println!("\t> {elapsed:?}");

                    //println!(">> {:?}", msg);

                    match msg {
                        // ChatMessage
                        Msg::ChatMessage(ChatMessage {
                            player_id,
                            kind,
                            message,
                        }) => {
                            println!(
                                "player {:?} sent a message of type {:?}: {}",
                                player_id, kind, message
                            );

                            if message == "Hi" && player_id != my_player_id {
                                let peer = e.peer_mut();

                                println!("Send hi from player {:?}", my_player_id);

                                // ChatMessage
                                peer.send_packet(
                                    Packet::new(
                                        vec![17, my_player_id.0, kind as u8, b'H', b'i', b'!'],
                                        PacketMode::ReliableSequenced,
                                    )
                                    .unwrap(),
                                    0,
                                )
                                .context("sending packet failed")?;
                            }
                        }
                        // https://github.com/piqueserver/aosprotocol/pull/39/files
                        // VersionHandshakeInit
                        Msg::VersionHandshakeInit(VersionHandshakeInit { challenge }) => {
                            println!("Handling handshake, secret number: {}", challenge);

                            let peer = e.peer_mut();

                            // VersionHandshakeResponse
                            let mut v = vec![MessageKind::VersionHandshakeResponse.id()];
                            v.extend(challenge.to_le_bytes());
                            println!("Handshake response: {:?}", v);
                            peer.send_packet(
                                Packet::new(v, PacketMode::ReliableSequenced).unwrap(),
                                0,
                            )
                            .context("sending packet failed")?;
                        }
                        // VersionGet
                        Msg::VersionGet(_) => {
                            println!("VersionGet Request");

                            let peer = e.peer_mut();

                            // VersionResponse
                            let v = vec![
                                34, b'c', 1, 1, 1, b'W', b'i', b'n', b'd', b'o', b'w', b's', b' ',
                                b'1', b'0',
                            ];
                            peer.send_packet(
                                Packet::new(v, PacketMode::ReliableSequenced).unwrap(),
                                0,
                            )
                            .context("sending packet failed")?;
                        }
                        // StateData
                        Msg::StateData(StateData { player_id, .. }) => {
                            let player_id = player_id;
                            my_player_id = player_id;

                            let peer = e.peer_mut();

                            // existing player packet, used to join a team after StateData
                            let data = vec![
                                MessageKind::ExisitingPlayer.id(),
                                player_id.0,
                                -1i8 as u8,
                                0,
                                0,
                                0,
                                0,
                                0,
                                0,
                                0,
                                0,
                                0,
                                b'T',
                                b'e',
                                b's',
                                b't',
                            ];
                            println!("transmitted data: {:?}", data);
                            peer.send_packet(
                                Packet::new(data, PacketMode::ReliableSequenced).unwrap(),
                                0,
                            )
                            .context("sending packet failed")?;

                            // Short CreatePlayer
                            // does not work here
                            // peer.send_packet(
                            //     Packet::new(vec![10, player_id, -0x1i8 as u8, 0],
                            //     PacketMode::ReliableSequenced).unwrap(),
                            //     0,
                            // )
                            // .context("sending packet failed")?;

                            // change team
                            // does not work here
                            // peer.send_packet(
                            //     Packet::new(vec![29, player_id, -0x1i8 as u8],
                            //     PacketMode::ReliableSequenced).unwrap(),
                            //     0,
                            // )
                            // .context("sending packet failed")?;

                            peer.send_packet(
                                Packet::new(
                                    vec![17, player_id.0, 0, b'H', b'i'],
                                    PacketMode::ReliableSequenced,
                                )
                                .unwrap(),
                                0,
                            )
                            .context("sending packet failed")?;
                        }
                        _ => {}
                    }
                }
                EventKind::Disconnect { data } => {
                    println!(
                        "connection closed, peer: {:?}, reason: {}",
                        e.peer_id(),
                        data
                    );
                    std::process::exit(0);
                }
                _ => {}
            }
        }
        // println!("received event: {:#?}", e);
    }
}
