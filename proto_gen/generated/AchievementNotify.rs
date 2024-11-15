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

//! Generated file from `AchievementNotify.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_7_1;

// @@protoc_insertion_point(message:Mualani.Guide.AchievementInfo)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AchievementInfo {
    // message fields
    // @@protoc_insertion_point(field:Mualani.Guide.AchievementInfo.id)
    pub id: u32,
    // @@protoc_insertion_point(field:Mualani.Guide.AchievementInfo.status)
    pub status: ::protobuf::EnumOrUnknown<Status>,
    // @@protoc_insertion_point(field:Mualani.Guide.AchievementInfo.fin_prog)
    pub fin_prog: u32,
    // @@protoc_insertion_point(field:Mualani.Guide.AchievementInfo.cur_prog)
    pub cur_prog: u32,
    // @@protoc_insertion_point(field:Mualani.Guide.AchievementInfo.finish_time)
    pub finish_time: u32,
    // special fields
    // @@protoc_insertion_point(special_field:Mualani.Guide.AchievementInfo.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AchievementInfo {
    fn default() -> &'a AchievementInfo {
        <AchievementInfo as ::protobuf::Message>::default_instance()
    }
}

impl AchievementInfo {
    pub fn new() -> AchievementInfo {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "id",
            |m: &AchievementInfo| { &m.id },
            |m: &mut AchievementInfo| { &mut m.id },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "status",
            |m: &AchievementInfo| { &m.status },
            |m: &mut AchievementInfo| { &mut m.status },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "fin_prog",
            |m: &AchievementInfo| { &m.fin_prog },
            |m: &mut AchievementInfo| { &mut m.fin_prog },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "cur_prog",
            |m: &AchievementInfo| { &m.cur_prog },
            |m: &mut AchievementInfo| { &mut m.cur_prog },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "finish_time",
            |m: &AchievementInfo| { &m.finish_time },
            |m: &mut AchievementInfo| { &mut m.finish_time },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AchievementInfo>(
            "AchievementInfo",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AchievementInfo {
    const NAME: &'static str = "AchievementInfo";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                16 => {
                    self.id = is.read_uint32()?;
                },
                56 => {
                    self.status = is.read_enum_or_unknown()?;
                },
                120 => {
                    self.fin_prog = is.read_uint32()?;
                },
                40 => {
                    self.cur_prog = is.read_uint32()?;
                },
                88 => {
                    self.finish_time = is.read_uint32()?;
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
        if self.id != 0 {
            my_size += ::protobuf::rt::uint32_size(2, self.id);
        }
        if self.status != ::protobuf::EnumOrUnknown::new(Status::STATUS_INVALID) {
            my_size += ::protobuf::rt::int32_size(7, self.status.value());
        }
        if self.fin_prog != 0 {
            my_size += ::protobuf::rt::uint32_size(15, self.fin_prog);
        }
        if self.cur_prog != 0 {
            my_size += ::protobuf::rt::uint32_size(5, self.cur_prog);
        }
        if self.finish_time != 0 {
            my_size += ::protobuf::rt::uint32_size(11, self.finish_time);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.id != 0 {
            os.write_uint32(2, self.id)?;
        }
        if self.status != ::protobuf::EnumOrUnknown::new(Status::STATUS_INVALID) {
            os.write_enum(7, ::protobuf::EnumOrUnknown::value(&self.status))?;
        }
        if self.fin_prog != 0 {
            os.write_uint32(15, self.fin_prog)?;
        }
        if self.cur_prog != 0 {
            os.write_uint32(5, self.cur_prog)?;
        }
        if self.finish_time != 0 {
            os.write_uint32(11, self.finish_time)?;
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

    fn new() -> AchievementInfo {
        AchievementInfo::new()
    }

    fn clear(&mut self) {
        self.id = 0;
        self.status = ::protobuf::EnumOrUnknown::new(Status::STATUS_INVALID);
        self.fin_prog = 0;
        self.cur_prog = 0;
        self.finish_time = 0;
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AchievementInfo {
        static instance: AchievementInfo = AchievementInfo {
            id: 0,
            status: ::protobuf::EnumOrUnknown::from_i32(0),
            fin_prog: 0,
            cur_prog: 0,
            finish_time: 0,
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AchievementInfo {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AchievementInfo").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AchievementInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AchievementInfo {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:Mualani.Guide.AchievementNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AchievementNotify {
    // message fields
    // @@protoc_insertion_point(field:Mualani.Guide.AchievementNotify.reward_taken_goal_id_list)
    pub reward_taken_goal_id_list: ::std::vec::Vec<u32>,
    // @@protoc_insertion_point(field:Mualani.Guide.AchievementNotify.achievements)
    pub achievements: ::std::vec::Vec<AchievementInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:Mualani.Guide.AchievementNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AchievementNotify {
    fn default() -> &'a AchievementNotify {
        <AchievementNotify as ::protobuf::Message>::default_instance()
    }
}

impl AchievementNotify {
    pub fn new() -> AchievementNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "reward_taken_goal_id_list",
            |m: &AchievementNotify| { &m.reward_taken_goal_id_list },
            |m: &mut AchievementNotify| { &mut m.reward_taken_goal_id_list },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "achievements",
            |m: &AchievementNotify| { &m.achievements },
            |m: &mut AchievementNotify| { &mut m.achievements },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AchievementNotify>(
            "AchievementNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AchievementNotify {
    const NAME: &'static str = "AchievementNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                42 => {
                    is.read_repeated_packed_uint32_into(&mut self.reward_taken_goal_id_list)?;
                },
                40 => {
                    self.reward_taken_goal_id_list.push(is.read_uint32()?);
                },
                34 => {
                    self.achievements.push(is.read_message()?);
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
        my_size += ::protobuf::rt::vec_packed_uint32_size(5, &self.reward_taken_goal_id_list);
        for value in &self.achievements {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        os.write_repeated_packed_uint32(5, &self.reward_taken_goal_id_list)?;
        for v in &self.achievements {
            ::protobuf::rt::write_message_field_with_cached_size(4, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> AchievementNotify {
        AchievementNotify::new()
    }

    fn clear(&mut self) {
        self.reward_taken_goal_id_list.clear();
        self.achievements.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AchievementNotify {
        static instance: AchievementNotify = AchievementNotify {
            reward_taken_goal_id_list: ::std::vec::Vec::new(),
            achievements: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AchievementNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AchievementNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AchievementNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AchievementNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

// @@protoc_insertion_point(message:Mualani.Guide.AchievementUpdateNotify)
#[derive(PartialEq,Clone,Default,Debug)]
pub struct AchievementUpdateNotify {
    // message fields
    // @@protoc_insertion_point(field:Mualani.Guide.AchievementUpdateNotify.achievements)
    pub achievements: ::std::vec::Vec<AchievementInfo>,
    // special fields
    // @@protoc_insertion_point(special_field:Mualani.Guide.AchievementUpdateNotify.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a AchievementUpdateNotify {
    fn default() -> &'a AchievementUpdateNotify {
        <AchievementUpdateNotify as ::protobuf::Message>::default_instance()
    }
}

impl AchievementUpdateNotify {
    pub fn new() -> AchievementUpdateNotify {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(1);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "achievements",
            |m: &AchievementUpdateNotify| { &m.achievements },
            |m: &mut AchievementUpdateNotify| { &mut m.achievements },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<AchievementUpdateNotify>(
            "AchievementUpdateNotify",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for AchievementUpdateNotify {
    const NAME: &'static str = "AchievementUpdateNotify";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                66 => {
                    self.achievements.push(is.read_message()?);
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
        for value in &self.achievements {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        for v in &self.achievements {
            ::protobuf::rt::write_message_field_with_cached_size(8, v, os)?;
        };
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> AchievementUpdateNotify {
        AchievementUpdateNotify::new()
    }

    fn clear(&mut self) {
        self.achievements.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static AchievementUpdateNotify {
        static instance: AchievementUpdateNotify = AchievementUpdateNotify {
            achievements: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for AchievementUpdateNotify {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("AchievementUpdateNotify").unwrap()).clone()
    }
}

impl ::std::fmt::Display for AchievementUpdateNotify {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AchievementUpdateNotify {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:Mualani.Guide.Status)
pub enum Status {
    // @@protoc_insertion_point(enum_value:Mualani.Guide.Status.STATUS_INVALID)
    STATUS_INVALID = 0,
    // @@protoc_insertion_point(enum_value:Mualani.Guide.Status.STATUS_UNFINISHED)
    STATUS_UNFINISHED = 1,
    // @@protoc_insertion_point(enum_value:Mualani.Guide.Status.STATUS_FINISHED)
    STATUS_FINISHED = 2,
    // @@protoc_insertion_point(enum_value:Mualani.Guide.Status.STATUS_REWARD_TAKEN)
    STATUS_REWARD_TAKEN = 3,
}

impl ::protobuf::Enum for Status {
    const NAME: &'static str = "Status";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Status> {
        match value {
            0 => ::std::option::Option::Some(Status::STATUS_INVALID),
            1 => ::std::option::Option::Some(Status::STATUS_UNFINISHED),
            2 => ::std::option::Option::Some(Status::STATUS_FINISHED),
            3 => ::std::option::Option::Some(Status::STATUS_REWARD_TAKEN),
            _ => ::std::option::Option::None
        }
    }

    fn from_str(str: &str) -> ::std::option::Option<Status> {
        match str {
            "STATUS_INVALID" => ::std::option::Option::Some(Status::STATUS_INVALID),
            "STATUS_UNFINISHED" => ::std::option::Option::Some(Status::STATUS_UNFINISHED),
            "STATUS_FINISHED" => ::std::option::Option::Some(Status::STATUS_FINISHED),
            "STATUS_REWARD_TAKEN" => ::std::option::Option::Some(Status::STATUS_REWARD_TAKEN),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [Status] = &[
        Status::STATUS_INVALID,
        Status::STATUS_UNFINISHED,
        Status::STATUS_FINISHED,
        Status::STATUS_REWARD_TAKEN,
    ];
}

impl ::protobuf::EnumFull for Status {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("Status").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for Status {
    fn default() -> Self {
        Status::STATUS_INVALID
    }
}

impl Status {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<Status>("Status")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x17AchievementNotify.proto\x12\rMualani.Guide\"\xa7\x01\n\x0fAchievem\
    entInfo\x12\x0e\n\x02id\x18\x02\x20\x01(\rR\x02id\x12-\n\x06status\x18\
    \x07\x20\x01(\x0e2\x15.Mualani.Guide.StatusR\x06status\x12\x19\n\x08fin_\
    prog\x18\x0f\x20\x01(\rR\x07finProg\x12\x19\n\x08cur_prog\x18\x05\x20\
    \x01(\rR\x07curProg\x12\x1f\n\x0bfinish_time\x18\x0b\x20\x01(\rR\nfinish\
    Time\"\x91\x01\n\x11AchievementNotify\x128\n\x19reward_taken_goal_id_lis\
    t\x18\x05\x20\x03(\rR\x15rewardTakenGoalIdList\x12B\n\x0cachievements\
    \x18\x04\x20\x03(\x0b2\x1e.Mualani.Guide.AchievementInfoR\x0cachievement\
    s\"]\n\x17AchievementUpdateNotify\x12B\n\x0cachievements\x18\x08\x20\x03\
    (\x0b2\x1e.Mualani.Guide.AchievementInfoR\x0cachievements*a\n\x06Status\
    \x12\x12\n\x0eSTATUS_INVALID\x10\0\x12\x15\n\x11STATUS_UNFINISHED\x10\
    \x01\x12\x13\n\x0fSTATUS_FINISHED\x10\x02\x12\x17\n\x13STATUS_REWARD_TAK\
    EN\x10\x03b\x06proto3\
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
            let mut messages = ::std::vec::Vec::with_capacity(3);
            messages.push(AchievementInfo::generated_message_descriptor_data());
            messages.push(AchievementNotify::generated_message_descriptor_data());
            messages.push(AchievementUpdateNotify::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(Status::generated_enum_descriptor_data());
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
