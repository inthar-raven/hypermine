#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Meta {
    /// Number of voxels along the edge of a chunk
    #[prost(uint32, tag = "1")]
    pub chunk_size: u32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Character {
    /// Graph edges to traverse from the origin to find the node containing the character's entity
    #[prost(uint32, repeated, tag = "1")]
    pub path: ::prost::alloc::vec::Vec<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EntityNode {
    /// Entities whose origins lie within this node, each encoded as:
    /// { entity: u64, component_count: varint, components: \[{ type: varint, length: varint, data: [u8\] }] }
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub entities: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VoxelNode {
    /// Voxel data for each modified chunk
    #[prost(message, repeated, tag = "1")]
    pub chunks: ::prost::alloc::vec::Vec<Chunk>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Chunk {
    /// Which dodecahedron vertex is associated with this chunk
    #[prost(uint32, tag = "1")]
    pub vertex: u32,
    /// Dense 3D array of 16-bit material tags for all voxels in this chunk
    #[prost(bytes = "vec", tag = "2")]
    pub voxels: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ComponentType {
    /// 4x4 matrix of f32s
    Position = 0,
    /// UTF-8 text
    Name = 1,
}
impl ComponentType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ComponentType::Position => "POSITION",
            ComponentType::Name => "NAME",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "POSITION" => Some(Self::Position),
            "NAME" => Some(Self::Name),
            _ => None,
        }
    }
}
