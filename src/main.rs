use capnp::serialize;
use capnp::schema_capnp::node;
use capnp::schema_capnp::node::struct_;
use capnp::schema_capnp::node::interface;
use capnp::schema_capnp::node::const_;
use capnp::schema_capnp::node::enum_;
use capnp::schema_capnp::node::annotation;
use capnpc::codegen::GeneratorContext;

struct ReadWrapper<R>
where
    R: std::io::Read,
{
    inner: R,
}

impl<R> capnp::io::Read for ReadWrapper<R>
where
    R: std::io::Read,
{
    fn read(&mut self, buf: &mut [u8]) -> capnp::Result<usize> {
        loop {
            match std::io::Read::read(&mut self.inner, buf) {
                Ok(n) => return Ok(n),
                Err(e) if e.kind() == std::io::ErrorKind::Interrupted => {}
                Err(e) => return Err(capnp::Error{description: format!("{e}"), kind: capnp::ErrorKind::Failed}),
            }
        }
    }
}

pub fn main() {
    let message = serialize::read_message(
        ReadWrapper { inner: ::std::io::stdin() },
        capnp::message::ReaderOptions::new(),
    ).unwrap();
    let ctx: GeneratorContext = GeneratorContext::new(&message).unwrap();
    for requested_file in ctx.request.get_requested_files().unwrap() {
        load(&ctx, requested_file.get_id());
    }
}

pub fn load(ctx: &GeneratorContext, node_id: u64) {
    let node_ = ctx.node_map[&node_id];
    match node_.which().unwrap() {
        node::File(_) => {}
        node::Struct(struct_) => {
            handle_struct(ctx, struct_);
        }
        node::Interface(interface_) => {
            handle_interface(ctx, interface_);
        }
        node::Const(const_) => {
            handle_const(ctx, const_);
        }
        node::Enum(enum_) => {
            handle_enum(ctx, enum_);
        }
        node::Annotation(annotation_) => {
            handle_annotation(ctx, annotation_);
        }
    }
    for nested_node in node_.get_nested_nodes().unwrap(){
        load(ctx, nested_node.get_id())
    }
}

pub fn handle_struct(ctx: &GeneratorContext, struct_: struct_::Reader) {
    // 
}

pub fn handle_interface(ctx: &GeneratorContext, interface_: interface::Reader) {
    // 
}

pub fn handle_const(ctx: &GeneratorContext, const_: const_::Reader) {
    // 
}

pub fn handle_enum(ctx: &GeneratorContext, enum_: enum_::Reader) {
    // 
}

pub fn handle_annotation(ctx: &GeneratorContext, annotation_: annotation::Reader) {
    // 
}