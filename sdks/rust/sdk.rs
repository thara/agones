// This file is generated. Do not edit
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
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct Empty {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Empty {}

impl Empty {
    pub fn new() -> Empty {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Empty {
        static mut instance: ::protobuf::lazy::Lazy<Empty> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Empty,
        };
        unsafe {
            instance.get(Empty::new)
        }
    }
}

impl ::protobuf::Message for Empty {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Empty {
    fn new() -> Empty {
        Empty::new()
    }

    fn descriptor_static(_: ::std::option::Option<Empty>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<Empty>(
                    "Empty",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Empty {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Empty {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Empty {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\tsdk.proto\x12\x15stable.agones.dev.sdk\"\x07\n\x05Empty2\xe0\x01\n\
    \x03SDK\x12E\n\x05Ready\x12\x1c.stable.agones.dev.sdk.Empty\x1a\x1c.stab\
    le.agones.dev.sdk.Empty\"\0\x12H\n\x08Shutdown\x12\x1c.stable.agones.dev\
    .sdk.Empty\x1a\x1c.stable.agones.dev.sdk.Empty\"\0\x12H\n\x06Health\x12\
    \x1c.stable.agones.dev.sdk.Empty\x1a\x1c.stable.agones.dev.sdk.Empty\"\0\
    (\x01B\x05Z\x03sdkJ\xff\x08\n\x06\x12\x04\x0e\0!\x01\n\xd2\x04\n\x01\x0c\
    \x12\x03\x0e\0\x122\xc7\x04\x20Copyright\x202017\x20Google\x20Inc.\x20Al\
    l\x20Rights\x20Reserved.\n\n\x20Licensed\x20under\x20the\x20Apache\x20Li\
    cense,\x20Version\x202.0\x20(the\x20\"License\");\n\x20you\x20may\x20not\
    \x20use\x20this\x20file\x20except\x20in\x20compliance\x20with\x20the\x20\
    License.\n\x20You\x20may\x20obtain\x20a\x20copy\x20of\x20the\x20License\
    \x20at\n\n\x20\x20\x20\x20\x20http://www.apache.org/licenses/LICENSE-2.0\
    \n\n\x20Unless\x20required\x20by\x20applicable\x20law\x20or\x20agreed\
    \x20to\x20in\x20writing,\x20software\n\x20distributed\x20under\x20the\
    \x20License\x20is\x20distributed\x20on\x20an\x20\"AS\x20IS\"\x20BASIS,\n\
    \x20WITHOUT\x20WARRANTIES\x20OR\x20CONDITIONS\x20OF\x20ANY\x20KIND,\x20e\
    ither\x20express\x20or\x20implied.\n\x20See\x20the\x20License\x20for\x20\
    the\x20specific\x20language\x20governing\x20permissions\x20and\n\x20limi\
    tations\x20under\x20the\x20License.\n\n\x08\n\x01\x02\x12\x03\x10\x08\
    \x1d\n\x08\n\x01\x08\x12\x03\x11\0\x1a\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\
    \x11\0\x1a\n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\x11\x07\x11\n\r\n\x06\
    \x08\xe7\x07\0\x02\0\x12\x03\x11\x07\x11\n\x0e\n\x07\x08\xe7\x07\0\x02\0\
    \x01\x12\x03\x11\x07\x11\n\x0c\n\x05\x08\xe7\x07\0\x07\x12\x03\x11\x14\
    \x19\nM\n\x02\x06\0\x12\x04\x14\0\x1e\x01\x1aA\x20SDK\x20service\x20to\
    \x20be\x20used\x20in\x20the\x20GameServer\x20SDK\x20to\x20the\x20Pod\x20\
    Sidecar\n\n\n\n\x03\x06\0\x01\x12\x03\x14\x08\x0b\n1\n\x04\x06\0\x02\0\
    \x12\x04\x16\x04\x17\x05\x1a#\x20Call\x20when\x20the\x20GameServer\x20is\
    \x20ready\n\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03\x16\x08\r\n\x0c\n\x05\
    \x06\0\x02\0\x02\x12\x03\x16\x0f\x14\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03\
    \x16\x1f$\n9\n\x04\x06\0\x02\x01\x12\x04\x19\x04\x1a\x05\x1a+\x20Call\
    \x20when\x20the\x20GmaeServer\x20is\x20shutting\x20down\n\n\x0c\n\x05\
    \x06\0\x02\x01\x01\x12\x03\x19\x08\x10\n\x0c\n\x05\x06\0\x02\x01\x02\x12\
    \x03\x19\x12\x17\n\x0c\n\x05\x06\0\x02\x01\x03\x12\x03\x19\"'\nW\n\x04\
    \x06\0\x02\x02\x12\x04\x1c\x04\x1d\x05\x1aI\x20Send\x20a\x20Empty\x20eve\
    ry\x20d\x20Duration\x20to\x20declare\x20that\x20this\x20GameSever\x20is\
    \x20healthy\n\n\x0c\n\x05\x06\0\x02\x02\x01\x12\x03\x1c\x08\x0e\n\x0c\n\
    \x05\x06\0\x02\x02\x05\x12\x03\x1c\x10\x16\n\x0c\n\x05\x06\0\x02\x02\x02\
    \x12\x03\x1c\x17\x1c\n\x0c\n\x05\x06\0\x02\x02\x03\x12\x03\x1c',\n\n\n\
    \x02\x04\0\x12\x04\x20\0!\x01\n\n\n\x03\x04\0\x01\x12\x03\x20\x08\rb\x06\
    proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
