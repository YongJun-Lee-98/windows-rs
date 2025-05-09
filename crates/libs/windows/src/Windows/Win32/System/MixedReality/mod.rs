pub const PERCEPTIONFIELD_StateStream_TimeStamps: windows_core::GUID = windows_core::GUID::from_u128(0xaa886119_f32f_49bf_92ca_f9ddf784d297);
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PERCEPTION_PAYLOAD_FIELD {
    pub FieldId: windows_core::GUID,
    pub OffsetInBytes: u32,
    pub SizeInBytes: u32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PERCEPTION_STATE_STREAM_TIMESTAMPS {
    pub InputTimestampInQpcCounts: i64,
    pub AvailableTimestampInQpcCounts: i64,
}
