// This file is generated by rust-protobuf 3.0.0-pre. Do not edit
// .proto file is parsed by protoc --rust-out=...
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]

//! Generated file from `header.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_0_0_PRE;

#[derive(PartialEq,Clone,Default)]
#[derive(Serialize, Deserialize)]
pub struct Header {
    // message fields
    msg_id: ::std::option::Option<u32>,
    // special fields
    #[serde(skip)]
    pub unknown_fields: ::protobuf::UnknownFields,
    #[serde(skip)]
    pub cached_size: ::protobuf::rt::CachedSize,
}

impl<'a> ::std::default::Default for &'a Header {
    fn default() -> &'a Header {
        <Header as ::protobuf::Message>::default_instance()
    }
}

impl Header {
    pub fn new() -> Header {
        ::std::default::Default::default()
    }

    // required uint32 msg_id = 1;

    pub fn get_msg_id(&self) -> u32 {
        self.msg_id.unwrap_or(0)
    }

    pub fn clear_msg_id(&mut self) {
        self.msg_id = ::std::option::Option::None;
    }

    pub fn has_msg_id(&self) -> bool {
        self.msg_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg_id(&mut self, v: u32) {
        self.msg_id = ::std::option::Option::Some(v);
    }
}

impl ::protobuf::Message for Header {
    fn is_initialized(&self) -> bool {
        if self.msg_id.is_none() {
            return false;
        }
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    self.msg_id = ::std::option::Option::Some(is.read_uint32()?);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.msg_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.msg_id {
            os.write_uint32(1, v)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Header {
        Header::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::rt::make_option_get_copy_accessor::<_, ::protobuf::reflect::types::ProtobufTypeUint32, _>(
                "msg_id",
                |m: &Header| { &m.msg_id },
                |m: &mut Header| { &mut m.msg_id },
                Header::get_msg_id,
            ));
            ::protobuf::reflect::MessageDescriptor::new::<Header>(
                "Header",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Header {
        static instance: ::protobuf::rt::Lazy<Header> = ::protobuf::rt::Lazy::INIT;
        instance.get(Header::new)
    }
}

impl ::protobuf::Clear for Header {
    fn clear(&mut self) {
        self.msg_id = ::std::option::Option::None;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Header {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Header {
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0cheader.proto\x1a\x0frustproto.proto\"\x1f\n\x06Header\x12\x15\n\
    \x06msg_id\x18\x01\x20\x02(\rR\x05msgIdB\x04\xb0\xa8\x08\x01J\x91\x01\n\
    \x06\x12\x04\0\0\x07\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\t\n\x02\x03\0\
    \x12\x03\x02\x07\x18\n\x08\n\x01\x08\x12\x03\x03\0+\n\x0b\n\x04\x08\x86\
    \x85\x01\x12\x03\x03\0+\n\n\n\x02\x04\0\x12\x04\x05\0\x07\x01\n\n\n\x03\
    \x04\0\x01\x12\x03\x05\x08\x0e\n\x0b\n\x04\x04\0\x02\0\x12\x03\x06\x04\
    \x1f\n\x0c\n\x05\x04\0\x02\0\x04\x12\x03\x06\x04\x0c\n\x0c\n\x05\x04\0\
    \x02\0\x05\x12\x03\x06\r\x13\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x06\x14\
    \x1a\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x06\x1d\x1e\
";

static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

/// `FileDescriptorProto` object which was a source for this generated file
pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
