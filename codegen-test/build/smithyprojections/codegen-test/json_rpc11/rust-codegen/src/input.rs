// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use crate::model::EmptyStruct;
use crate::model::FooEnum;
use crate::model::KitchenSink;
use crate::model::MyUnion;
use crate::model::SimpleStruct;
use crate::model::StructWithLocationName;
use crate::serializer::JsonEnumsInputBody;
use crate::serializer::JsonUnionsInputBody;
use crate::serializer::KitchenSinkOperationInputBody;
use crate::serializer::NullOperationInputBody;
use crate::serializer::OperationWithOptionalInputOutputInputBody;
use crate::serializer::PutAndGetInlineDocumentsInputBody;
use smithy_types::Blob;
use smithy_types::Document;
use smithy_types::Instant;
/// See [`EmptyOperationInput`](crate::input::EmptyOperationInput)
pub mod empty_operation_input {

    use crate::input::EmptyOperationInput;
    use crate::operation::EmptyOperation;
    /// A builder for [`EmptyOperationInput`](crate::input::EmptyOperationInput)
    #[non_exhaustive]
    #[derive(Debug, Clone, Default)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`EmptyOperation`](crate::operation::EmptyOperation)
        pub fn build(self) -> EmptyOperation {
            EmptyOperation::new(EmptyOperationInput {})
        }
    }
}
impl EmptyOperationInput {
    pub fn request_builder_base(&self) -> ::http::request::Builder {
        let builder = ::http::request::Builder::new();
        builder
            .method("POST")
            .header("Content-Type", "application/x-amz-json-1.1")
            .header("X-Amz-Target", "JsonProtocol.EmptyOperation")
    }
    pub fn build_body(&self) -> ::std::vec::Vec<u8> {
        vec![]
    }
    pub fn assemble(
        builder: ::http::request::Builder,
        body: ::std::vec::Vec<u8>,
    ) -> ::http::request::Request<::std::vec::Vec<u8>> {
        builder
            .header(::http::header::CONTENT_LENGTH, body.len())
            .body(body)
            .expect("http request should be valid")
    }
    /// Creates a new builder-style object to manufacture [`EmptyOperationInput`](crate::input::EmptyOperationInput)
    pub fn builder() -> crate::input::empty_operation_input::Builder {
        crate::input::empty_operation_input::Builder::default()
    }
}

/// See [`GreetingWithErrorsInput`](crate::input::GreetingWithErrorsInput)
pub mod greeting_with_errors_input {

    use crate::input::GreetingWithErrorsInput;
    use crate::operation::GreetingWithErrors;
    /// A builder for [`GreetingWithErrorsInput`](crate::input::GreetingWithErrorsInput)
    #[non_exhaustive]
    #[derive(Debug, Clone, Default)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`GreetingWithErrors`](crate::operation::GreetingWithErrors)
        pub fn build(self) -> GreetingWithErrors {
            GreetingWithErrors::new(GreetingWithErrorsInput {})
        }
    }
}
impl GreetingWithErrorsInput {
    pub fn request_builder_base(&self) -> ::http::request::Builder {
        let builder = ::http::request::Builder::new();
        builder
            .method("POST")
            .header("Content-Type", "application/x-amz-json-1.1")
            .header("X-Amz-Target", "JsonProtocol.GreetingWithErrors")
    }
    pub fn build_body(&self) -> ::std::vec::Vec<u8> {
        vec![]
    }
    pub fn assemble(
        builder: ::http::request::Builder,
        body: ::std::vec::Vec<u8>,
    ) -> ::http::request::Request<::std::vec::Vec<u8>> {
        builder
            .header(::http::header::CONTENT_LENGTH, body.len())
            .body(body)
            .expect("http request should be valid")
    }
    /// Creates a new builder-style object to manufacture [`GreetingWithErrorsInput`](crate::input::GreetingWithErrorsInput)
    pub fn builder() -> crate::input::greeting_with_errors_input::Builder {
        crate::input::greeting_with_errors_input::Builder::default()
    }
}

/// See [`JsonEnumsInput`](crate::input::JsonEnumsInput)
pub mod json_enums_input {

    use crate::input::JsonEnumsInput;
    use crate::model::FooEnum;
    use crate::operation::JsonEnums;
    /// A builder for [`JsonEnumsInput`](crate::input::JsonEnumsInput)
    #[non_exhaustive]
    #[derive(Debug, Clone, Default)]
    pub struct Builder {
        foo_enum1: ::std::option::Option<FooEnum>,
        foo_enum2: ::std::option::Option<FooEnum>,
        foo_enum3: ::std::option::Option<FooEnum>,
        foo_enum_list: ::std::option::Option<::std::vec::Vec<FooEnum>>,
        foo_enum_set: ::std::option::Option<::std::collections::BTreeSet<FooEnum>>,
        foo_enum_map:
            ::std::option::Option<::std::collections::HashMap<::std::string::String, FooEnum>>,
    }
    impl Builder {
        pub fn foo_enum1(mut self, inp: FooEnum) -> Self {
            self.foo_enum1 = Some(inp);
            self
        }
        pub fn foo_enum2(mut self, inp: FooEnum) -> Self {
            self.foo_enum2 = Some(inp);
            self
        }
        pub fn foo_enum3(mut self, inp: FooEnum) -> Self {
            self.foo_enum3 = Some(inp);
            self
        }
        pub fn foo_enum_list(mut self, inp: ::std::vec::Vec<FooEnum>) -> Self {
            self.foo_enum_list = Some(inp);
            self
        }
        pub fn foo_enum_set(mut self, inp: ::std::collections::BTreeSet<FooEnum>) -> Self {
            self.foo_enum_set = Some(inp);
            self
        }
        pub fn foo_enum_map(
            mut self,
            inp: ::std::collections::HashMap<::std::string::String, FooEnum>,
        ) -> Self {
            self.foo_enum_map = Some(inp);
            self
        }
        /// Consumes the builder and constructs a [`JsonEnums`](crate::operation::JsonEnums)
        pub fn build(self) -> JsonEnums {
            JsonEnums::new(JsonEnumsInput {
                foo_enum1: self.foo_enum1,
                foo_enum2: self.foo_enum2,
                foo_enum3: self.foo_enum3,
                foo_enum_list: self.foo_enum_list,
                foo_enum_set: self.foo_enum_set,
                foo_enum_map: self.foo_enum_map,
            })
        }
    }
}
impl JsonEnumsInput {
    pub fn request_builder_base(&self) -> ::http::request::Builder {
        let builder = ::http::request::Builder::new();
        builder
            .method("POST")
            .header("Content-Type", "application/x-amz-json-1.1")
            .header("X-Amz-Target", "JsonProtocol.JsonEnums")
    }
    fn body(&self) -> JsonEnumsInputBody {
        JsonEnumsInputBody {
            foo_enum1: &self.foo_enum1,
            foo_enum2: &self.foo_enum2,
            foo_enum3: &self.foo_enum3,
            foo_enum_list: &self.foo_enum_list,
            foo_enum_set: &self.foo_enum_set,
            foo_enum_map: &self.foo_enum_map,
        }
    }
    pub fn build_body(&self) -> ::std::vec::Vec<u8> {
        ::serde_json::to_vec(&self.body()).expect("serialization should succeed")
    }
    pub fn assemble(
        builder: ::http::request::Builder,
        body: ::std::vec::Vec<u8>,
    ) -> ::http::request::Request<::std::vec::Vec<u8>> {
        builder
            .header(::http::header::CONTENT_LENGTH, body.len())
            .body(body)
            .expect("http request should be valid")
    }
    /// Creates a new builder-style object to manufacture [`JsonEnumsInput`](crate::input::JsonEnumsInput)
    pub fn builder() -> crate::input::json_enums_input::Builder {
        crate::input::json_enums_input::Builder::default()
    }
}

/// See [`JsonUnionsInput`](crate::input::JsonUnionsInput)
pub mod json_unions_input {

    use crate::input::JsonUnionsInput;
    use crate::model::MyUnion;
    use crate::operation::JsonUnions;
    /// A builder for [`JsonUnionsInput`](crate::input::JsonUnionsInput)
    #[non_exhaustive]
    #[derive(Debug, Clone, Default)]
    pub struct Builder {
        contents: ::std::option::Option<MyUnion>,
    }
    impl Builder {
        /// A union with a representative set of types for members.
        pub fn contents(mut self, inp: MyUnion) -> Self {
            self.contents = Some(inp);
            self
        }
        /// Consumes the builder and constructs a [`JsonUnions`](crate::operation::JsonUnions)
        pub fn build(self) -> JsonUnions {
            JsonUnions::new(JsonUnionsInput {
                contents: self.contents,
            })
        }
    }
}
impl JsonUnionsInput {
    pub fn request_builder_base(&self) -> ::http::request::Builder {
        let builder = ::http::request::Builder::new();
        builder
            .method("POST")
            .header("Content-Type", "application/x-amz-json-1.1")
            .header("X-Amz-Target", "JsonProtocol.JsonUnions")
    }
    fn body(&self) -> JsonUnionsInputBody {
        JsonUnionsInputBody {
            contents: &self.contents,
        }
    }
    pub fn build_body(&self) -> ::std::vec::Vec<u8> {
        ::serde_json::to_vec(&self.body()).expect("serialization should succeed")
    }
    pub fn assemble(
        builder: ::http::request::Builder,
        body: ::std::vec::Vec<u8>,
    ) -> ::http::request::Request<::std::vec::Vec<u8>> {
        builder
            .header(::http::header::CONTENT_LENGTH, body.len())
            .body(body)
            .expect("http request should be valid")
    }
    /// Creates a new builder-style object to manufacture [`JsonUnionsInput`](crate::input::JsonUnionsInput)
    pub fn builder() -> crate::input::json_unions_input::Builder {
        crate::input::json_unions_input::Builder::default()
    }
}

/// See [`KitchenSinkOperationInput`](crate::input::KitchenSinkOperationInput)
pub mod kitchen_sink_operation_input {

    use crate::input::KitchenSinkOperationInput;
    use crate::model::EmptyStruct;
    use crate::model::KitchenSink;
    use crate::model::SimpleStruct;
    use crate::model::StructWithLocationName;
    use crate::operation::KitchenSinkOperation;
    use smithy_types::Blob;
    use smithy_types::Instant;
    /// A builder for [`KitchenSinkOperationInput`](crate::input::KitchenSinkOperationInput)
    #[non_exhaustive]
    #[derive(Debug, Clone, Default)]
    pub struct Builder {
        blob: ::std::option::Option<Blob>,
        boolean: ::std::option::Option<bool>,
        double: ::std::option::Option<f64>,
        empty_struct: ::std::option::Option<EmptyStruct>,
        float: ::std::option::Option<f32>,
        httpdate_timestamp: ::std::option::Option<Instant>,
        integer: ::std::option::Option<i32>,
        iso8601_timestamp: ::std::option::Option<Instant>,
        json_value: ::std::option::Option<::std::string::String>,
        list_of_lists:
            ::std::option::Option<::std::vec::Vec<::std::vec::Vec<::std::string::String>>>,
        list_of_maps_of_strings: ::std::option::Option<
            ::std::vec::Vec<
                ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            >,
        >,
        list_of_strings: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        list_of_structs: ::std::option::Option<::std::vec::Vec<SimpleStruct>>,
        long: ::std::option::Option<i64>,
        map_of_lists_of_strings: ::std::option::Option<
            ::std::collections::HashMap<
                ::std::string::String,
                ::std::vec::Vec<::std::string::String>,
            >,
        >,
        map_of_maps: ::std::option::Option<
            ::std::collections::HashMap<
                ::std::string::String,
                ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            >,
        >,
        map_of_strings: ::std::option::Option<
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
        map_of_structs:
            ::std::option::Option<::std::collections::HashMap<::std::string::String, SimpleStruct>>,
        recursive_list: ::std::option::Option<::std::vec::Vec<KitchenSink>>,
        recursive_map:
            ::std::option::Option<::std::collections::HashMap<::std::string::String, KitchenSink>>,
        recursive_struct: ::std::option::Option<::std::boxed::Box<KitchenSink>>,
        simple_struct: ::std::option::Option<SimpleStruct>,
        string: ::std::option::Option<::std::string::String>,
        struct_with_location_name: ::std::option::Option<StructWithLocationName>,
        timestamp: ::std::option::Option<Instant>,
        unix_timestamp: ::std::option::Option<Instant>,
    }
    impl Builder {
        pub fn blob(mut self, inp: Blob) -> Self {
            self.blob = Some(inp);
            self
        }
        pub fn boolean(mut self, inp: bool) -> Self {
            self.boolean = Some(inp);
            self
        }
        pub fn double(mut self, inp: f64) -> Self {
            self.double = Some(inp);
            self
        }
        pub fn empty_struct(mut self, inp: EmptyStruct) -> Self {
            self.empty_struct = Some(inp);
            self
        }
        pub fn float(mut self, inp: f32) -> Self {
            self.float = Some(inp);
            self
        }
        pub fn httpdate_timestamp(mut self, inp: Instant) -> Self {
            self.httpdate_timestamp = Some(inp);
            self
        }
        pub fn integer(mut self, inp: i32) -> Self {
            self.integer = Some(inp);
            self
        }
        pub fn iso8601_timestamp(mut self, inp: Instant) -> Self {
            self.iso8601_timestamp = Some(inp);
            self
        }
        pub fn json_value(mut self, inp: impl Into<::std::string::String>) -> Self {
            self.json_value = Some(inp.into());
            self
        }
        pub fn list_of_lists(
            mut self,
            inp: ::std::vec::Vec<::std::vec::Vec<::std::string::String>>,
        ) -> Self {
            self.list_of_lists = Some(inp);
            self
        }
        pub fn list_of_maps_of_strings(
            mut self,
            inp: ::std::vec::Vec<
                ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            >,
        ) -> Self {
            self.list_of_maps_of_strings = Some(inp);
            self
        }
        pub fn list_of_strings(mut self, inp: ::std::vec::Vec<::std::string::String>) -> Self {
            self.list_of_strings = Some(inp);
            self
        }
        pub fn list_of_structs(mut self, inp: ::std::vec::Vec<SimpleStruct>) -> Self {
            self.list_of_structs = Some(inp);
            self
        }
        pub fn long(mut self, inp: i64) -> Self {
            self.long = Some(inp);
            self
        }
        pub fn map_of_lists_of_strings(
            mut self,
            inp: ::std::collections::HashMap<
                ::std::string::String,
                ::std::vec::Vec<::std::string::String>,
            >,
        ) -> Self {
            self.map_of_lists_of_strings = Some(inp);
            self
        }
        pub fn map_of_maps(
            mut self,
            inp: ::std::collections::HashMap<
                ::std::string::String,
                ::std::collections::HashMap<::std::string::String, ::std::string::String>,
            >,
        ) -> Self {
            self.map_of_maps = Some(inp);
            self
        }
        pub fn map_of_strings(
            mut self,
            inp: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        ) -> Self {
            self.map_of_strings = Some(inp);
            self
        }
        pub fn map_of_structs(
            mut self,
            inp: ::std::collections::HashMap<::std::string::String, SimpleStruct>,
        ) -> Self {
            self.map_of_structs = Some(inp);
            self
        }
        pub fn recursive_list(mut self, inp: ::std::vec::Vec<KitchenSink>) -> Self {
            self.recursive_list = Some(inp);
            self
        }
        pub fn recursive_map(
            mut self,
            inp: ::std::collections::HashMap<::std::string::String, KitchenSink>,
        ) -> Self {
            self.recursive_map = Some(inp);
            self
        }
        pub fn recursive_struct(mut self, inp: impl Into<::std::boxed::Box<KitchenSink>>) -> Self {
            self.recursive_struct = Some(inp.into());
            self
        }
        pub fn simple_struct(mut self, inp: SimpleStruct) -> Self {
            self.simple_struct = Some(inp);
            self
        }
        pub fn string(mut self, inp: impl Into<::std::string::String>) -> Self {
            self.string = Some(inp.into());
            self
        }
        pub fn struct_with_location_name(mut self, inp: StructWithLocationName) -> Self {
            self.struct_with_location_name = Some(inp);
            self
        }
        pub fn timestamp(mut self, inp: Instant) -> Self {
            self.timestamp = Some(inp);
            self
        }
        pub fn unix_timestamp(mut self, inp: Instant) -> Self {
            self.unix_timestamp = Some(inp);
            self
        }
        /// Consumes the builder and constructs a [`KitchenSinkOperation`](crate::operation::KitchenSinkOperation)
        pub fn build(self) -> KitchenSinkOperation {
            KitchenSinkOperation::new(KitchenSinkOperationInput {
                blob: self.blob,
                boolean: self.boolean,
                double: self.double,
                empty_struct: self.empty_struct,
                float: self.float,
                httpdate_timestamp: self.httpdate_timestamp,
                integer: self.integer,
                iso8601_timestamp: self.iso8601_timestamp,
                json_value: self.json_value,
                list_of_lists: self.list_of_lists,
                list_of_maps_of_strings: self.list_of_maps_of_strings,
                list_of_strings: self.list_of_strings,
                list_of_structs: self.list_of_structs,
                long: self.long,
                map_of_lists_of_strings: self.map_of_lists_of_strings,
                map_of_maps: self.map_of_maps,
                map_of_strings: self.map_of_strings,
                map_of_structs: self.map_of_structs,
                recursive_list: self.recursive_list,
                recursive_map: self.recursive_map,
                recursive_struct: self.recursive_struct,
                simple_struct: self.simple_struct,
                string: self.string,
                struct_with_location_name: self.struct_with_location_name,
                timestamp: self.timestamp,
                unix_timestamp: self.unix_timestamp,
            })
        }
    }
}
impl KitchenSinkOperationInput {
    pub fn request_builder_base(&self) -> ::http::request::Builder {
        let builder = ::http::request::Builder::new();
        builder
            .method("POST")
            .header("Content-Type", "application/x-amz-json-1.1")
            .header("X-Amz-Target", "JsonProtocol.KitchenSinkOperation")
    }
    fn body(&self) -> KitchenSinkOperationInputBody {
        KitchenSinkOperationInputBody {
            blob: &self.blob,
            boolean: &self.boolean,
            double: &self.double,
            empty_struct: &self.empty_struct,
            float: &self.float,
            httpdate_timestamp: &self.httpdate_timestamp,
            integer: &self.integer,
            iso8601_timestamp: &self.iso8601_timestamp,
            json_value: &self.json_value,
            list_of_lists: &self.list_of_lists,
            list_of_maps_of_strings: &self.list_of_maps_of_strings,
            list_of_strings: &self.list_of_strings,
            list_of_structs: &self.list_of_structs,
            long: &self.long,
            map_of_lists_of_strings: &self.map_of_lists_of_strings,
            map_of_maps: &self.map_of_maps,
            map_of_strings: &self.map_of_strings,
            map_of_structs: &self.map_of_structs,
            recursive_list: &self.recursive_list,
            recursive_map: &self.recursive_map,
            recursive_struct: &self.recursive_struct,
            simple_struct: &self.simple_struct,
            string: &self.string,
            struct_with_location_name: &self.struct_with_location_name,
            timestamp: &self.timestamp,
            unix_timestamp: &self.unix_timestamp,
        }
    }
    pub fn build_body(&self) -> ::std::vec::Vec<u8> {
        ::serde_json::to_vec(&self.body()).expect("serialization should succeed")
    }
    pub fn assemble(
        builder: ::http::request::Builder,
        body: ::std::vec::Vec<u8>,
    ) -> ::http::request::Request<::std::vec::Vec<u8>> {
        builder
            .header(::http::header::CONTENT_LENGTH, body.len())
            .body(body)
            .expect("http request should be valid")
    }
    /// Creates a new builder-style object to manufacture [`KitchenSinkOperationInput`](crate::input::KitchenSinkOperationInput)
    pub fn builder() -> crate::input::kitchen_sink_operation_input::Builder {
        crate::input::kitchen_sink_operation_input::Builder::default()
    }
}

/// See [`NullOperationInput`](crate::input::NullOperationInput)
pub mod null_operation_input {

    use crate::input::NullOperationInput;
    use crate::operation::NullOperation;
    /// A builder for [`NullOperationInput`](crate::input::NullOperationInput)
    #[non_exhaustive]
    #[derive(Debug, Clone, Default)]
    pub struct Builder {
        string: ::std::option::Option<::std::string::String>,
        sparse_string_list:
            ::std::option::Option<::std::vec::Vec<::std::option::Option<::std::string::String>>>,
        sparse_string_map: ::std::option::Option<
            ::std::collections::HashMap<
                ::std::string::String,
                ::std::option::Option<::std::string::String>,
            >,
        >,
    }
    impl Builder {
        pub fn string(mut self, inp: impl Into<::std::string::String>) -> Self {
            self.string = Some(inp.into());
            self
        }
        pub fn sparse_string_list(
            mut self,
            inp: ::std::vec::Vec<::std::option::Option<::std::string::String>>,
        ) -> Self {
            self.sparse_string_list = Some(inp);
            self
        }
        pub fn sparse_string_map(
            mut self,
            inp: ::std::collections::HashMap<
                ::std::string::String,
                ::std::option::Option<::std::string::String>,
            >,
        ) -> Self {
            self.sparse_string_map = Some(inp);
            self
        }
        /// Consumes the builder and constructs a [`NullOperation`](crate::operation::NullOperation)
        pub fn build(self) -> NullOperation {
            NullOperation::new(NullOperationInput {
                string: self.string,
                sparse_string_list: self.sparse_string_list,
                sparse_string_map: self.sparse_string_map,
            })
        }
    }
}
impl NullOperationInput {
    pub fn request_builder_base(&self) -> ::http::request::Builder {
        let builder = ::http::request::Builder::new();
        builder
            .method("POST")
            .header("Content-Type", "application/x-amz-json-1.1")
            .header("X-Amz-Target", "JsonProtocol.NullOperation")
    }
    fn body(&self) -> NullOperationInputBody {
        NullOperationInputBody {
            string: &self.string,
            sparse_string_list: &self.sparse_string_list,
            sparse_string_map: &self.sparse_string_map,
        }
    }
    pub fn build_body(&self) -> ::std::vec::Vec<u8> {
        ::serde_json::to_vec(&self.body()).expect("serialization should succeed")
    }
    pub fn assemble(
        builder: ::http::request::Builder,
        body: ::std::vec::Vec<u8>,
    ) -> ::http::request::Request<::std::vec::Vec<u8>> {
        builder
            .header(::http::header::CONTENT_LENGTH, body.len())
            .body(body)
            .expect("http request should be valid")
    }
    /// Creates a new builder-style object to manufacture [`NullOperationInput`](crate::input::NullOperationInput)
    pub fn builder() -> crate::input::null_operation_input::Builder {
        crate::input::null_operation_input::Builder::default()
    }
}

/// See [`OperationWithOptionalInputOutputInput`](crate::input::OperationWithOptionalInputOutputInput)
pub mod operation_with_optional_input_output_input {

    use crate::input::OperationWithOptionalInputOutputInput;
    use crate::operation::OperationWithOptionalInputOutput;
    /// A builder for [`OperationWithOptionalInputOutputInput`](crate::input::OperationWithOptionalInputOutputInput)
    #[non_exhaustive]
    #[derive(Debug, Clone, Default)]
    pub struct Builder {
        value: ::std::option::Option<::std::string::String>,
    }
    impl Builder {
        pub fn value(mut self, inp: impl Into<::std::string::String>) -> Self {
            self.value = Some(inp.into());
            self
        }
        /// Consumes the builder and constructs a [`OperationWithOptionalInputOutput`](crate::operation::OperationWithOptionalInputOutput)
        pub fn build(self) -> OperationWithOptionalInputOutput {
            OperationWithOptionalInputOutput::new(OperationWithOptionalInputOutputInput {
                value: self.value,
            })
        }
    }
}
impl OperationWithOptionalInputOutputInput {
    pub fn request_builder_base(&self) -> ::http::request::Builder {
        let builder = ::http::request::Builder::new();
        builder
            .method("POST")
            .header("Content-Type", "application/x-amz-json-1.1")
            .header(
                "X-Amz-Target",
                "JsonProtocol.OperationWithOptionalInputOutput",
            )
    }
    fn body(&self) -> OperationWithOptionalInputOutputInputBody {
        OperationWithOptionalInputOutputInputBody { value: &self.value }
    }
    pub fn build_body(&self) -> ::std::vec::Vec<u8> {
        ::serde_json::to_vec(&self.body()).expect("serialization should succeed")
    }
    pub fn assemble(
        builder: ::http::request::Builder,
        body: ::std::vec::Vec<u8>,
    ) -> ::http::request::Request<::std::vec::Vec<u8>> {
        builder
            .header(::http::header::CONTENT_LENGTH, body.len())
            .body(body)
            .expect("http request should be valid")
    }
    /// Creates a new builder-style object to manufacture [`OperationWithOptionalInputOutputInput`](crate::input::OperationWithOptionalInputOutputInput)
    pub fn builder() -> crate::input::operation_with_optional_input_output_input::Builder {
        crate::input::operation_with_optional_input_output_input::Builder::default()
    }
}

/// See [`PutAndGetInlineDocumentsInput`](crate::input::PutAndGetInlineDocumentsInput)
pub mod put_and_get_inline_documents_input {

    use crate::input::PutAndGetInlineDocumentsInput;
    use crate::operation::PutAndGetInlineDocuments;
    use smithy_types::Document;
    /// A builder for [`PutAndGetInlineDocumentsInput`](crate::input::PutAndGetInlineDocumentsInput)
    #[non_exhaustive]
    #[derive(Debug, Clone, Default)]
    pub struct Builder {
        inline_document: ::std::option::Option<Document>,
    }
    impl Builder {
        pub fn inline_document(mut self, inp: Document) -> Self {
            self.inline_document = Some(inp);
            self
        }
        /// Consumes the builder and constructs a [`PutAndGetInlineDocuments`](crate::operation::PutAndGetInlineDocuments)
        pub fn build(self) -> Result<PutAndGetInlineDocuments, String> {
            Ok(PutAndGetInlineDocuments::new(
                PutAndGetInlineDocumentsInput {
                    inline_document: self.inline_document.ok_or(
                        "inline_document is required when building PutAndGetInlineDocumentsInput",
                    )?,
                },
            ))
        }
    }
}
impl PutAndGetInlineDocumentsInput {
    pub fn request_builder_base(&self) -> ::http::request::Builder {
        let builder = ::http::request::Builder::new();
        builder
            .method("POST")
            .header("Content-Type", "application/x-amz-json-1.1")
            .header("X-Amz-Target", "JsonProtocol.PutAndGetInlineDocuments")
    }
    fn body(&self) -> PutAndGetInlineDocumentsInputBody {
        PutAndGetInlineDocumentsInputBody {
            inline_document: &self.inline_document,
        }
    }
    pub fn build_body(&self) -> ::std::vec::Vec<u8> {
        ::serde_json::to_vec(&self.body()).expect("serialization should succeed")
    }
    pub fn assemble(
        builder: ::http::request::Builder,
        body: ::std::vec::Vec<u8>,
    ) -> ::http::request::Request<::std::vec::Vec<u8>> {
        builder
            .header(::http::header::CONTENT_LENGTH, body.len())
            .body(body)
            .expect("http request should be valid")
    }
    /// Creates a new builder-style object to manufacture [`PutAndGetInlineDocumentsInput`](crate::input::PutAndGetInlineDocumentsInput)
    pub fn builder() -> crate::input::put_and_get_inline_documents_input::Builder {
        crate::input::put_and_get_inline_documents_input::Builder::default()
    }
}

/// A shared structure that contains a single union member.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct JsonUnionsInput {
    /// A union with a representative set of types for members.
    pub contents: ::std::option::Option<MyUnion>,
}

#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GreetingWithErrorsInput {}

#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct NullOperationInput {
    pub string: ::std::option::Option<::std::string::String>,
    pub sparse_string_list:
        ::std::option::Option<::std::vec::Vec<::std::option::Option<::std::string::String>>>,
    pub sparse_string_map: ::std::option::Option<
        ::std::collections::HashMap<
            ::std::string::String,
            ::std::option::Option<::std::string::String>,
        >,
    >,
}

#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct JsonEnumsInput {
    pub foo_enum1: ::std::option::Option<FooEnum>,
    pub foo_enum2: ::std::option::Option<FooEnum>,
    pub foo_enum3: ::std::option::Option<FooEnum>,
    pub foo_enum_list: ::std::option::Option<::std::vec::Vec<FooEnum>>,
    pub foo_enum_set: ::std::option::Option<::std::collections::BTreeSet<FooEnum>>,
    pub foo_enum_map:
        ::std::option::Option<::std::collections::HashMap<::std::string::String, FooEnum>>,
}

#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct PutAndGetInlineDocumentsInput {
    pub inline_document: Document,
}

#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct OperationWithOptionalInputOutputInput {
    pub value: ::std::option::Option<::std::string::String>,
}

#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct KitchenSinkOperationInput {
    pub blob: ::std::option::Option<Blob>,
    pub boolean: ::std::option::Option<bool>,
    pub double: ::std::option::Option<f64>,
    pub empty_struct: ::std::option::Option<EmptyStruct>,
    pub float: ::std::option::Option<f32>,
    pub httpdate_timestamp: ::std::option::Option<Instant>,
    pub integer: ::std::option::Option<i32>,
    pub iso8601_timestamp: ::std::option::Option<Instant>,
    pub json_value: ::std::option::Option<::std::string::String>,
    pub list_of_lists:
        ::std::option::Option<::std::vec::Vec<::std::vec::Vec<::std::string::String>>>,
    pub list_of_maps_of_strings: ::std::option::Option<
        ::std::vec::Vec<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    >,
    pub list_of_strings: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub list_of_structs: ::std::option::Option<::std::vec::Vec<SimpleStruct>>,
    pub long: ::std::option::Option<i64>,
    pub map_of_lists_of_strings: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::vec::Vec<::std::string::String>>,
    >,
    pub map_of_maps: ::std::option::Option<
        ::std::collections::HashMap<
            ::std::string::String,
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
        >,
    >,
    pub map_of_strings: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    pub map_of_structs:
        ::std::option::Option<::std::collections::HashMap<::std::string::String, SimpleStruct>>,
    pub recursive_list: ::std::option::Option<::std::vec::Vec<KitchenSink>>,
    pub recursive_map:
        ::std::option::Option<::std::collections::HashMap<::std::string::String, KitchenSink>>,
    pub recursive_struct: ::std::option::Option<::std::boxed::Box<KitchenSink>>,
    pub simple_struct: ::std::option::Option<SimpleStruct>,
    pub string: ::std::option::Option<::std::string::String>,
    pub struct_with_location_name: ::std::option::Option<StructWithLocationName>,
    pub timestamp: ::std::option::Option<Instant>,
    pub unix_timestamp: ::std::option::Option<Instant>,
}

#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct EmptyOperationInput {}
