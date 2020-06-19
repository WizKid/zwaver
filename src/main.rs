mod serial;
mod command_class;

use x25519_dalek::StaticSecret;
use x25519_dalek::PublicKey;
use rand::prelude::*;
use rand::distributions::Standard;
use serial::codec;
use serial::message;
use serial::message::dataframe;
use tokio_serial;
use futures::stream::StreamExt;
use futures_util::sink::SinkExt;
use std::time::Duration;
use tokio_util::codec::Framed;
use tokio::time::timeout;
use bytes::{ Bytes, Buf };
use aes::Aes128;
use cmac::{Cmac, Mac, NewMac};
use generic_array::{
    GenericArray,
    typenum::{U8, U13, U32},
};
use rand_ctr_drbg::CtrDrbg;
use aes_ccm::{
    Aes128Ccm,
    aead::{Aead, NewAead, Payload},
};


#[tokio::main]
async fn main() {
    /*
    let my_secret = StaticSecret::from([160, 231, 49, 90, 76, 88, 118, 104, 3, 28, 199, 66, 204, 133, 175, 65, 138, 254, 191, 195, 165, 244, 7, 133, 234, 157, 96, 181, 201, 106, 154, 125]);
    let my_public = PublicKey::from(&my_secret);

    println!("my public {:?}", my_public);

    // keypad public: [0, 0, 151, 127, 0, 66, 118, 241, 65, 157, 237, 87, 251, 232, 242, 237, 129, 33, 226, 38, 254, 211, 18, 101, 185, 150, 94, 6, 162, 125, 195, 24]
    let their_public = PublicKey::from([255, 13, 151, 127, 0, 66, 118, 241, 65, 157, 237, 87, 251, 232, 242, 237, 129, 33, 226, 38, 254, 211, 18, 101, 185, 150, 94, 6, 162, 125, 195, 24]);

    println!("their public {:?}", their_public);

    let shared_secret = my_secret.diffie_hellman(&their_public);

    println!("shared secret {:?}", shared_secret.as_bytes());

    let dataframe_bytes = b"\0\x04\0\"$\x9f\x03B\x01\x12A)v\xb0#@x)\x95NGPT\xd7\xa5\x13:\x0e\x98\x01\xb7\xab\x12g\xba\x8b\xde\xbf&]\x89~\0";

    let bytes = Bytes::from(&dataframe_bytes[..]);
    let dataframe = codec::Codec::decode_dataframe(&bytes, false);

    println!("dataframe: {:?}", dataframe);

    let constant_prk: [u8; 16] = [0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33, 0x33];

    let mut mac = Cmac::<Aes128>::new_varkey(&constant_prk).unwrap();
    mac.update(shared_secret.as_bytes());
    mac.update(my_public.as_bytes());
    mac.update(their_public.as_bytes());
    let result = mac.finalize();

    let result_bytes = result.into_bytes();
    println!("prk {:?}", result_bytes);

    let mut mac2 = Cmac::<Aes128>::new_varkey(&result_bytes).unwrap();
    let constant_te: [u8; 16] = [0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x01];
    mac2.update(&constant_te);

    let result = mac2.finalize();
    let result_bytes = result.into_bytes();
    println!("temp_key_ccm {:?}", result_bytes);
    */

    // encrupted message: 9F 03 42 01 12 41 29 76 B0 23 40 78 29 95 4E 47 50 54 D7 A5 13 3A 0E 98 01 B7 AB 12 67 BA 8B DE BF 26 5D 89 7E 00
    // their first:       8E 13 21 B1 78 7E A6 D4 1D 12 2D 29 35 BA F3 4E
    // my first:          96 8E 7F 2E FC 97 81 96 9B 24 A1 5B 5B 95 D1 6F
    // prk my first       4E 34 5F F6 79 A1 06 5D 43 78 AC 23 A3 80 A5 B7
    // prk their first    27 B5 B2 55 32 F3 C6 12 BC 8E 99 37 85 E9 4A 23

    // home id 3374324898
    // my node_id = 1
    // their node_id = 34
    // sequence number 65
    // alice_secret: [160, 231, 49, 90, 76, 88, 118, 104, 3, 28, 199, 66, 204, 133, 175, 65, 138, 254, 191, 195, 165, 244, 7, 133, 234, 157, 96, 181, 201, 106, 154, 125]
    // alice_public: [20, 157, 159, 255, 207, 164, 78, 211, 88, 166, 76, 171, 169, 167, 154, 252, 131, 191, 106, 220, 63, 0, 155, 140, 4, 150, 145, 5, 242, 32, 55, 39] 
    // keypad public: [0, 0, 151, 127, 0, 66, 118, 241, 65, 157, 237, 87, 251, 232, 242, 237, 129, 33, 226, 38, 254, 211, 18, 101, 185, 150, 94, 6, 162, 125, 195, 24]
    // my nonce: [217, 43, 67, 254, 215, 59, 151, 229, 155, 45, 167, 181, 251, 235, 108, 185]
    // their nonce: [41, 118, 176, 35, 64, 120, 41, 149, 78, 71, 80, 84, 215, 165, 19, 58]
    // DataFrame(Request(ApplicationCommandHandler(ApplicationCommandHandlerRequest { status: 0, node_id: 34, data: Security2(KexReport(KexData { request_csa: false, echo: false, kex_schemes: 2, ecdh_profiles: 1, keys: 132 })) })))
    // DataFrame: b"\x01,\0\x04\0\"$\x9f\x03B\x01\x12A)v\xb0#@x)\x95NGPT\xd7\xa5\x13:\x0e\x98\x01\xb7\xab\x12g\xba\x8b\xde\xbf&]\x89~\0a"
    // decode_dataframe: b"\0\x04\0\"$\x9f\x03B\x01\x12A)v\xb0#@x)\x95NGPT\xd7\xa5\x13:\x0e\x98\x01\xb7\xab\x12g\xba\x8b\xde\xbf&]\x89~\0"

    let sender_ei = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0a\x0b\x0c\x0d\x0e\x0f";
    let reciever_ei = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0a\x0b\x0c\x0d\x0e\x0f";

    let mut mac = Cmac::<Aes128>::new_varkey(b"\x26\x26\x26\x26\x26\x26\x26\x26\x26\x26\x26\x26\x26\x26\x26\x26").unwrap();
    mac.update(sender_ei);
    mac.update(reciever_ei);
    let nonce_prk = mac.finalize().into_bytes();
    println!("nonce_prk {:x?}", nonce_prk);

    let mut mac = Cmac::<Aes128>::new_varkey(&nonce_prk[..]).unwrap();
    mac.update(b"\x88\x88\x88\x88\x88\x88\x88\x88\x88\x88\x88\x88\x88\x88\x88\x00");
    mac.update(b"\x88\x88\x88\x88\x88\x88\x88\x88\x88\x88\x88\x88\x88\x88\x88\x01");
    let mei1 = mac.finalize().into_bytes();
    println!("mei1 {:x?}", mei1);
    
    let mut mac = Cmac::<Aes128>::new_varkey(&nonce_prk[..]).unwrap();
    mac.update(&mei1[..]);
    mac.update(b"\x88\x88\x88\x88\x88\x88\x88\x88\x88\x88\x88\x88\x88\x88\x88\x02");
    let mei2 = mac.finalize().into_bytes();
    println!("mei2 {:x?}", mei2);

    let mut mei = GenericArray::<u8, U32>::default();
    mei[0..16].clone_from_slice(&mei1);
    mei[16..32].clone_from_slice(&mei2);
    println!("mei {:x?}", mei);

    let prk = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0a\x0b\x0c\x0d\x0e\x0f";
    let mut mac = Cmac::<Aes128>::new_varkey(&prk[..]).unwrap();
    mac.update(b"\x88\x88\x88\x88\x88\x88\x88\x88\x88\x88\x88\x88\x88\x88\x88\x01");
    let temp_key_ccm = mac.finalize().into_bytes();
    println!("temp_key_ccm {:x?}", temp_key_ccm);

    let mut mac = Cmac::<Aes128>::new_varkey(&prk[..]).unwrap();
    mac.update(&temp_key_ccm);
    mac.update(b"\x88\x88\x88\x88\x88\x88\x88\x88\x88\x88\x88\x88\x88\x88\x88\x02");
    let temp_personalization_string1 = mac.finalize().into_bytes();
    println!("temp_personalization_string1 {:x?}", temp_personalization_string1);

    let mut mac = Cmac::<Aes128>::new_varkey(&prk[..]).unwrap();
    mac.update(&temp_personalization_string1);
    mac.update(b"\x88\x88\x88\x88\x88\x88\x88\x88\x88\x88\x88\x88\x88\x88\x88\x03");
    let temp_personalization_string2 = mac.finalize().into_bytes();
    println!("temp_personalization_string2 {:x?}", temp_personalization_string2);

    let mut temp_personalization_string = GenericArray::<u8, U32>::default();
    temp_personalization_string[0..16].clone_from_slice(&temp_personalization_string1);
    temp_personalization_string[16..32].clone_from_slice(&temp_personalization_string2);

    println!("temp_personalization_string {:x?}", temp_personalization_string);

    let mut ctr_drbg = CtrDrbg::<Aes128>::new(&mei, &temp_personalization_string);
    let mut span = GenericArray::<u8, U13>::default();
    ctr_drbg.fill_bytes(&mut span);

    println!("span {:x?}", span);

    let associated_data = b"\x01\x02\x01\x01\x01\x01\x00\x01\x01\x00";
    let msg = b"\x00";

    let aes_ccm = Aes128Ccm::<U8>::new(&temp_key_ccm);
    let ciphertext = aes_ccm
        .encrypt(
            &span.into(),
            Payload {
                aad: associated_data,
                msg: msg,
            },
        )
        .unwrap();

    println!("ciphertext {:x?}", ciphertext);

    return;

    let mut alice_csprng = rand::thread_rng();
    let alice_secret = StaticSecret::new(&mut alice_csprng);
    let alice_public = PublicKey::from(&alice_secret);

    println!("alice_secret: {:?}", alice_secret.to_bytes());
    println!("alice_public: {:?}", alice_public.as_bytes());

    let settings = tokio_serial::SerialPortSettings {
        baud_rate: 115200,
        data_bits: tokio_serial::DataBits::Eight,
        flow_control: tokio_serial::FlowControl::None,
        parity: tokio_serial::Parity::None,
        stop_bits: tokio_serial::StopBits::One,
        timeout: Duration::new(1, 0),
    };
    let serial_port = tokio_serial::Serial::from_path("COM5", &settings).unwrap();

    let (mut writer, mut reader) = Framed::new(serial_port, codec::Codec).split();

    println!("Nak");
    writer.send(message::Message::Nak).await.unwrap();

    clear_messages(&mut writer, &mut reader).await;
    println!("Clear");

    writer.send(message::Message::DataFrame(
        dataframe::DataFrame::Request(
            dataframe::Request::GetVersion,
        ),
    )).await.unwrap();

    wait_message(&mut writer, &mut reader, 500).await;

    writer.send(message::Message::DataFrame(
        dataframe::DataFrame::Request(
            dataframe::Request::GetID,
        ),
    )).await.unwrap();

    wait_message(&mut writer, &mut reader, 500).await;

    writer.send(message::Message::DataFrame(
        dataframe::DataFrame::Request(
            dataframe::Request::SerialAPIGetCapabilities,
        ),
    )).await.unwrap();

    wait_message(&mut writer, &mut reader, 500).await;

    writer.send(message::Message::DataFrame(
        dataframe::DataFrame::Request(
            dataframe::Request::SerialAPIGetInitData,
        ),
    )).await.unwrap();

    wait_message(&mut writer, &mut reader, 500).await;

    /*
    writer.send(message::Message::DataFrame(
        dataframe::DataFrame::Request(
            dataframe::Request::GetNodeProtocolInfo(
                dataframe::NodeIDRequest { node_id: 15 },
            ),
        ),
    )).await.unwrap();

    wait_message(&mut writer, &mut reader, 500).await;

    writer.send(message::Message::DataFrame(
        dataframe::DataFrame::Request(
            dataframe::Request::IsNodeFailed(
                dataframe::NodeIDRequest { node_id: 15 },
            ),
        ),
    )).await.unwrap();

    wait_message(&mut writer, &mut reader, 500).await;
    */

    /*
    writer.send(message::Message::DataFrame(
        dataframe::DataFrame::Request(
            dataframe::Request::SendDataHost(
                dataframe::SendDataRequestHost { node_id: 14, data: command_class::CommandClass::Security2(command_class::security2::Security2::KexGet), options: 0x5, callback_id: 52 },
            ),
        ),
    )).await.unwrap();

    wait_message(&mut writer, &mut reader, 500).await;
    */

    /*
    writer.send(message::Message::DataFrame(
        dataframe::DataFrame::Request(
            dataframe::Request::RemoveFailedNode(
                dataframe::NodeIDWithCallbackRequest { node_id: 14, callback: 57 },
            ),
        ),
    )).await.unwrap();

    wait_message(&mut writer, &mut reader, 500).await;
    */

    /*
    writer.send(message::Message::DataFrame(
        dataframe::DataFrame::Request(
            dataframe::Request::GetNodeInfo(
                dataframe::NodeIDRequest { node_id: 29 },
            ),
        ),
    )).await.unwrap();

    wait_message_until(&mut writer, &mut reader, 10000,
        |dataframe| {
            if let message::dataframe::DataFrame::Request(
                message::dataframe::request::Request::ApplicationUpdate(
                    message::dataframe::request::application_update_request::ApplicationUpdateRequest::NodeInfoRequestFailed
                )
            ) = dataframe {
                true
            } else {
                false
            }
        }
    ).await;

    writer.send(message::Message::DataFrame(
        dataframe::DataFrame::Request(
            dataframe::Request::IsNodeFailed(
                dataframe::NodeIDRequest { node_id: 29 },
            ),
        ),
    )).await.unwrap();

    wait_message(&mut writer, &mut reader, 500).await;

    writer.send(message::Message::DataFrame(
        dataframe::DataFrame::Request(
            dataframe::Request::RemoveFailedNode(
                dataframe::NodeIDWithCallbackRequest { node_id: 29, callback: 59 },
            ),
        ),
    )).await.unwrap();

    // wait_message(&mut writer, &mut reader, 500).await;
    while true {
        wait_message(&mut writer, &mut reader, 10000).await;
    }
    */

    /*
    writer.send(message::Message::DataFrame(
        dataframe::DataFrame::Request(
            dataframe::Request::ReplaceFailedNode(
                dataframe::NodeIDRequest { node_id: 10 },
            ),
        ),
    )).await.unwrap();

    wait_message(&mut writer, &mut reader, 500).await;

    writer.send(message::Message::DataFrame(
        dataframe::DataFrame::Request(
            dataframe::Request::ReplaceFailedNode(
                dataframe::NodeIDRequest { node_id: 11 },
            ),
        ),
    )).await.unwrap();

    wait_message(&mut writer, &mut reader, 500).await;
    */

    /*
    while true {
        wait_message(&mut writer, &mut reader, 10000).await;
    }
    */

    writer.send(message::Message::DataFrame(
        dataframe::DataFrame::Request(
            dataframe::Request::AddNodeToNetworkHost(
                dataframe::AddNodeToNetworkRequestHost {
                    type_ : dataframe::add_node_to_network_request_host::Type::Any,
                    high_power: true,
                    network_wide: true,
                },
            ),
        ),
    )).await.unwrap();

    println!("Add Node state");

    let mut node_id = 0;

    println!("Waiting for ProtocolDone");

    wait_message_until(&mut writer, &mut reader, 10000,
        |dataframe| {
            if let message::dataframe::DataFrame::Request(
                message::dataframe::request::Request::AddNodeToNetworkChip(
                    message::dataframe::request::add_node_to_network_request_chip::AddNodeToNetworkRequestChip::ProtocolDone( data )  // Parse node id!
                )
            ) = dataframe {
                node_id = data.node_id;
                true
            } else {
                false
            }
        }
    ).await;

    assert_ne!(node_id, 0);
    println!("Send KexGet");

    writer.send(message::Message::DataFrame(
        dataframe::DataFrame::Request(
            dataframe::Request::SendDataHost(
                dataframe::SendDataRequestHost { node_id, data: command_class::CommandClass::Security2(command_class::security2::Security2::KexGet), options: 0x5, callback_id: 52 },
            ),
        ),
    )).await.unwrap();

    println!("Waiting for KexReport");

    wait_message_until(&mut writer, &mut reader, 10000,
        |data| {
            if let message::dataframe::DataFrame::Request(
                message::dataframe::request::Request::ApplicationCommandHandler(
                    message::dataframe::request::application_command_handler_request::ApplicationCommandHandlerRequest {
                        status: _,
                        node_id: _,
                        data: command_class::CommandClass::Security2(
                            command_class::security2::Security2::KexReport( _ )
                        ),
                    }
                )
            ) = data {
                true
            } else {
                false
            }
        }
    ).await;

    println!("Send KexSet");

    writer.send(message::Message::DataFrame(
        dataframe::DataFrame::Request(
            dataframe::Request::SendDataHost(
                dataframe::SendDataRequestHost {
                    node_id,
                    data: command_class::CommandClass::Security2(
                        command_class::security2::Security2::KexSet(
                            command_class::security2::KexData {
                                request_csa: false,
                                echo: false,
                                kex_schemes: 2,
                                ecdh_profiles: 1,
                                keys: 132,
                            }
                        )
                    ),
                    options: 0x5,
                    callback_id: 53
                },
            ),
        ),
    )).await.unwrap();

    println!("Waiting for Public Key Report");

    wait_message_until(&mut writer, &mut reader, 10000,
        |data| {
            if let message::dataframe::DataFrame::Request(
                message::dataframe::request::Request::ApplicationCommandHandler(
                    message::dataframe::request::application_command_handler_request::ApplicationCommandHandlerRequest {
                        status: _,
                        node_id: _,
                        data: command_class::CommandClass::Security2(
                            command_class::security2::Security2::PublicKeyReport( _ )
                        ),
                    }
                )
            ) = data {
                true
            } else {
                false
            }
        }
    ).await;

    println!("Send PublicKeyReportData");

    writer.send(message::Message::DataFrame(
        dataframe::DataFrame::Request(
            dataframe::Request::SendDataHost(
                dataframe::SendDataRequestHost {
                    node_id,
                    data: command_class::CommandClass::Security2(
                        command_class::security2::Security2::PublicKeyReport(
                            command_class::security2::PublicKeyReportData {
                                including_node: true,
                                key: *alice_public.as_bytes(),
                            }
                        )
                    ),
                    options: 0x5,
                    callback_id: 54
                },
            ),
        ),
    )).await.unwrap();


    println!("Waiting for NonceGet");

    wait_message_until(&mut writer, &mut reader, 10000,
        |data| {
            if let message::dataframe::DataFrame::Request(
                message::dataframe::request::Request::ApplicationCommandHandler(
                    message::dataframe::request::application_command_handler_request::ApplicationCommandHandlerRequest {
                        status: _,
                        node_id: _,
                        data: command_class::CommandClass::Security2(
                            command_class::security2::Security2::NonceGet( _ ),
                        ),
                    }
                )
            ) = data {
                true
            } else {
                false
            }
        }
    ).await;

    let csprng = rand::thread_rng();
    let nonce: Vec<u8> = csprng.sample_iter(Standard).take(16).collect();

    println!("Send NonceReport {:?}", nonce);

    writer.send(message::Message::DataFrame(
        dataframe::DataFrame::Request(
            dataframe::Request::SendDataHost(
                dataframe::SendDataRequestHost {
                    node_id,
                    data: command_class::CommandClass::Security2(
                        command_class::security2::Security2::NonceReport(
                            command_class::security2::NonceReportData {
                                sequence_number: 1,
                                mos: false,
                                sos: true,
                                entropy_input: Some(Bytes::from(nonce)),
                            }
                        )
                    ),
                    options: 0x5,
                    callback_id: 55
                },
            ),
        ),
    )).await.unwrap();

    println!("Waiting forever");

    // wait_message(&mut writer, &mut reader, 500).await;
    while true {
        wait_message(&mut writer, &mut reader, 10000).await;
    }
}

async fn wait_message_until<T>(
    writer: &mut futures_util::stream::SplitSink<tokio_util::codec::Framed<tokio_serial::Serial, serial::codec::Codec>, serial::message::Message>,
    reader: &mut futures_util::stream::SplitStream<tokio_util::codec::Framed<tokio_serial::Serial, serial::codec::Codec>>,
    millis: u64,
    mut lambda: T,
) where T: FnMut(message::dataframe::DataFrame) -> bool {
    while true {
        let timed_out_message = timeout(Duration::from_millis(millis), reader.next()).await;
        if let Ok(msg_result) = timed_out_message {
            if let Some(msg_result) = msg_result {
                let msg = msg_result.expect("Failed to read line");
                println!("{:?}", msg);
        
                match msg {
                    message::Message::Ack => {},
                    message::Message::Nak => {},
                    message::Message::Can => {},
                    message::Message::DataFrame( data) => {
                        writer.send(message::Message::Ack).await.unwrap();

                        if lambda(data) {
                            break;
                        }
                    },
                }
            }
        } else {
            println!("timed out");
        }
    }
}

async fn wait_message(
    writer: &mut futures_util::stream::SplitSink<tokio_util::codec::Framed<tokio_serial::Serial, serial::codec::Codec>, serial::message::Message>,
    reader: &mut futures_util::stream::SplitStream<tokio_util::codec::Framed<tokio_serial::Serial, serial::codec::Codec>>,
    millis: u64,
) {
    while let Ok(msg_result) = timeout(Duration::from_millis(millis), reader.next()).await {
        if let Some(msg_result) = msg_result {
            let msg = msg_result.expect("Failed to read line");
            println!("{:?}", msg);
    
            match msg {
                message::Message::Ack => {},
                message::Message::Nak => {},
                message::Message::Can => {},
                message::Message::DataFrame( _ ) => {
                    writer.send(message::Message::Ack).await.unwrap();
                    break;
                },
            }
        } else {
            break;
        }
    }
}

async fn clear_messages(
    writer: &mut futures_util::stream::SplitSink<tokio_util::codec::Framed<tokio_serial::Serial, serial::codec::Codec>, serial::message::Message>,
    reader: &mut futures_util::stream::SplitStream<tokio_util::codec::Framed<tokio_serial::Serial, serial::codec::Codec>>,
) {
    while let Ok(msg_result) = timeout(Duration::from_millis(1000), reader.next()).await {
        if let Some(msg_result) = msg_result {
            let msg = msg_result.expect("Failed to read line");
            println!("{:?}", msg);
    
            match msg {
                message::Message::Ack => {},
                message::Message::Nak => {},
                message::Message::Can => {},
                message::Message::DataFrame( _ ) => {
                    writer.send(message::Message::Ack).await.unwrap();
                },
            }
        }
    }
}
