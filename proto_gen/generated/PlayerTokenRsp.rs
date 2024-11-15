// This file is generated by rust-protobuf 3.7.1. Do not edit
// .proto file is parsed by protoc 29.0-rc1
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `PlayerTokenRsp.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:Mualani.Guide.PlayerTokenRsp)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct PlayerTokenRsp {
    // message fields
    // @@protoc_insertion_point(field:Mualani.Guide.PlayerTokenRsp.PlayerUID)
    pub PlayerUID: ::std::option::Option<u64>,
    // @@protoc_insertion_point(field:Mualani.Guide.PlayerTokenRsp.KeyID)
    pub KeyID: ::std::option::Option<u64>,
    // @@protoc_insertion_point(field:Mualani.Guide.PlayerTokenRsp.ServerSeed)
    pub ServerSeed: ::std::option::Option<::std::string::String>,
    // @@protoc_insertion_point(field:Mualani.Guide.PlayerTokenRsp.RegionCode)
    pub RegionCode: ::std::option::Option<::std::string::String>,
    // @@protoc_insertion_point(field:Mualani.Guide.PlayerTokenRsp.ClientIP)
    pub ClientIP: ::std::option::Option<::std::string::String>,
    // special fields
    // @@protoc_insertion_point(special_field:Mualani.Guide.PlayerTokenRsp.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a PlayerTokenRsp {
    fn default() -> &'a PlayerTokenRsp {
        <PlayerTokenRsp as ::protobuf::Message>::default_instance()
    }
}

impl PlayerTokenRsp {
    pub fn new() -> PlayerTokenRsp {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "PlayerUID",
            |m: &PlayerTokenRsp| { &m.PlayerUID },
            |m: &mut PlayerTokenRsp| { &mut m.PlayerUID },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "KeyID",
            |m: &PlayerTokenRsp| { &m.KeyID },
            |m: &mut PlayerTokenRsp| { &mut m.KeyID },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "ServerSeed",
            |m: &PlayerTokenRsp| { &m.ServerSeed },
            |m: &mut PlayerTokenRsp| { &mut m.ServerSeed },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "RegionCode",
            |m: &PlayerTokenRsp| { &m.RegionCode },
            |m: &mut PlayerTokenRsp| { &mut m.RegionCode },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_option_accessor::<_, _>(
            "ClientIP",
            |m: &PlayerTokenRsp| { &m.ClientIP },
            |m: &mut PlayerTokenRsp| { &mut m.ClientIP },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<PlayerTokenRsp>(
            "PlayerTokenRsp",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for PlayerTokenRsp {
    const NAME: &'static str = "PlayerTokenRsp";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                40 => {
                    self.PlayerUID = ::std::option::Option::Some(is.read_uint64()?);
                },
                10440 => {
                    self.KeyID = ::std::option::Option::Some(is.read_uint64()?);
                },
                5346 => {
                    self.ServerSeed = ::std::option::Option::Some(is.read_string()?);
                },
                4442 => {
                    self.RegionCode = ::std::option::Option::Some(is.read_string()?);
                },
                9618 => {
                    self.ClientIP = ::std::option::Option::Some(is.read_string()?);
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if let Some(v) = self.PlayerUID {
            my_size += ::protobuf::rt::uint64_size(5, v);
        }
        if let Some(v) = self.KeyID {
            my_size += ::protobuf::rt::uint64_size(1305, v);
        }
        if let Some(v) = self.ServerSeed.as_ref() {
            my_size += ::protobuf::rt::string_size(668, &v);
        }
        if let Some(v) = self.RegionCode.as_ref() {
            my_size += ::protobuf::rt::string_size(555, &v);
        }
        if let Some(v) = self.ClientIP.as_ref() {
            my_size += ::protobuf::rt::string_size(1202, &v);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if let Some(v) = self.PlayerUID {
            os.write_uint64(5, v)?;
        }
        if let Some(v) = self.KeyID {
            os.write_uint64(1305, v)?;
        }
        if let Some(v) = self.ServerSeed.as_ref() {
            os.write_string(668, v)?;
        }
        if let Some(v) = self.RegionCode.as_ref() {
            os.write_string(555, v)?;
        }
        if let Some(v) = self.ClientIP.as_ref() {
            os.write_string(1202, v)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> PlayerTokenRsp {
        PlayerTokenRsp::new()
    }

    fn clear(&mut self) {
        self.PlayerUID = ::std::option::Option::None;
        self.KeyID = ::std::option::Option::None;
        self.ServerSeed = ::std::option::Option::None;
        self.RegionCode = ::std::option::Option::None;
        self.ClientIP = ::std::option::Option::None;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static PlayerTokenRsp {
        static instance: PlayerTokenRsp = PlayerTokenRsp {
            PlayerUID: ::std::option::Option::None,
            KeyID: ::std::option::Option::None,
            ServerSeed: ::std::option::Option::None,
            RegionCode: ::std::option::Option::None,
            ClientIP: ::std::option::Option::None,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for PlayerTokenRsp {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("PlayerTokenRsp").unwrap()).clone()
    }
}

impl ::std::fmt::Display for PlayerTokenRsp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PlayerTokenRsp {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x14PlayerTokenRsp.proto\x12\rMualani.Guide\"\x80\x02\n\x0ePlayerToken\
    Rsp\x12!\n\tPlayerUID\x18\x05\x20\x01(\x04H\0R\tPlayerUID\x88\x01\x01\
    \x12\x1a\n\x05KeyID\x18\x99\n\x20\x01(\x04H\x01R\x05KeyID\x88\x01\x01\
    \x12$\n\nServerSeed\x18\x9c\x05\x20\x01(\tH\x02R\nServerSeed\x88\x01\x01\
    \x12$\n\nRegionCode\x18\xab\x04\x20\x01(\tH\x03R\nRegionCode\x88\x01\x01\
    \x12\x20\n\x08ClientIP\x18\xb2\t\x20\x01(\tH\x04R\x08ClientIP\x88\x01\
    \x01B\x0c\n\n_PlayerUIDB\x08\n\x06_KeyIDB\r\n\x0b_ServerSeedB\r\n\x0b_Re\
    gionCodeB\x0b\n\t_ClientIPb\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(1);
            messages.push(PlayerTokenRsp::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(0);
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
