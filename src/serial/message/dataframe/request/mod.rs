use bytes::{ Bytes };

pub mod add_node_to_network_request_chip;
pub use add_node_to_network_request_chip::AddNodeToNetworkRequestChip as AddNodeToNetworkRequestChip;

pub mod add_node_to_network_request_host;
pub use add_node_to_network_request_host::AddNodeToNetworkRequestHost as AddNodeToNetworkRequestHost;

pub mod application_command_handler_request;
pub use application_command_handler_request::ApplicationCommandHandlerRequest as ApplicationCommandHandlerRequest;

pub mod application_update_request;
pub use application_update_request::ApplicationUpdateRequest as ApplicationUpdateRequest;

pub mod node_id_request;
pub use node_id_request::NodeIDRequest as NodeIDRequest;

pub mod node_id_with_callback_request;
pub use node_id_with_callback_request::NodeIDWithCallbackRequest as NodeIDWithCallbackRequest;

pub mod send_data_request_chip;
pub use send_data_request_chip::SendDataRequestChip as SendDataRequestChip;

pub mod send_data_request_host;
pub use send_data_request_host::SendDataRequestHost as SendDataRequestHost;

#[derive(Debug, PartialEq, Clone)]
pub enum Request {
    SerialAPIGetInitData,
    ApplicationCommandHandler(ApplicationCommandHandlerRequest),
    SerialAPIGetCapabilities,
    SendDataHost(SendDataRequestHost),
    SendDataChip(SendDataRequestChip),
    GetVersion,
    GetID,
    GetNodeProtocolInfo(NodeIDRequest),
    ApplicationUpdate(ApplicationUpdateRequest),
    AddNodeToNetworkChip(AddNodeToNetworkRequestChip),
    AddNodeToNetworkHost(AddNodeToNetworkRequestHost),
    GetNodeInfo(NodeIDRequest),
    IsNodeFailed(NodeIDRequest),
    RemoveFailedNode(NodeIDWithCallbackRequest), // TODO Need to split Chip { callback_id, result }
    ReplaceFailedNode(NodeIDRequest),
    Unknown { func_id: u8, data : Bytes },
}