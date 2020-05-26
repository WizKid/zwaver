mod serial;
mod command_class;

use serial::codec;
use serial::message;
use serial::message::dataframe;
use tokio_serial;
use futures::stream::StreamExt;
use futures_util::sink::SinkExt;
use std::time::Duration;
use tokio_util::codec::Framed;
use tokio::time::timeout;

#[tokio::main]
async fn main() {
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

/*
    writer.send(message::Message::DataFrame(
        dataframe::DataFrame::Request(
            dataframe::Request::SendDataHost(
                dataframe::SendDataRequestHost { node_id: 18, data: command_class::CommandClass::Security2(command_class::security2::Security2::KexGet), options: 0x5, callback_id: 52 },
            ),
        ),
    )).await.unwrap();

    wait_message(&mut writer, &mut reader, 500).await;
    */

    writer.send(message::Message::DataFrame(
        dataframe::DataFrame::Request(
            dataframe::Request::SendDataHost(
                dataframe::SendDataRequestHost {
                    node_id: 18,
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

    // wait_message(&mut writer, &mut reader, 500).await;
    while true {
        wait_message(&mut writer, &mut reader, 10000).await;
    }

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
            dataframe::Request::GetNodeInfo(
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
            dataframe::Request::RemoveFailedNode(
                dataframe::NodeIDWithCallbackRequest { node_id: 11, callback: 58 },
            ),
        ),
    )).await.unwrap();

    wait_message(&mut writer, &mut reader, 500).await;
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

    while true {
        wait_message(&mut writer, &mut reader, 10000).await;
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
