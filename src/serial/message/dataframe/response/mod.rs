use bytes::{ Bytes };

mod get_id_response;
pub use get_id_response::GetIDResponse as GetIDResponse;

mod get_node_protocol_info_response;
pub use get_node_protocol_info_response::GetNodeProtocolInfoResponse as GetNodeProtocolInfoResponse;

mod get_version_response;
pub use get_version_response::GetVersionResponse as GetVersionResponse;

mod bool_response;
pub use bool_response::BoolResponse as BoolResponse;

mod remove_failed_node_response;
pub use remove_failed_node_response::RemoveFailedNodeResponse as RemoveFailedNodeResponse;

mod replace_failed_node_response;
pub use replace_failed_node_response::ReplaceFailedNodeResponse as ReplaceFailedNodeResponse;

mod serial_api_get_capabilities_response;
pub use serial_api_get_capabilities_response::SerialAPIGetCapabilitiesResponse as SerialAPIGetCapabilitiesResponse;

mod serial_api_get_init_data_response;
pub use serial_api_get_init_data_response::SerialAPIGetInitDataResponse as SerialAPIGetInitDataResponse;

#[derive(Debug, PartialEq, Clone)]
pub enum Response {
    SerialAPIGetInitData(SerialAPIGetInitDataResponse),
    SerialAPIGetCapabilities(SerialAPIGetCapabilitiesResponse),
    GetVersion(GetVersionResponse),
    GetID(GetIDResponse),
    GetNodeProtocolInfo(GetNodeProtocolInfoResponse),
    GetNodeInfo(BoolResponse),
    IsNodeFailed(BoolResponse),
    RemoveFailedNode(RemoveFailedNodeResponse),
    ReplaceFailedNode(ReplaceFailedNodeResponse),
    SendData(BoolResponse),
    Unknown { func_id: u8, data : Bytes },
}