use cranelift_codegen::{settings};
use cranelift_module::{ModuleError, default_libcall_names};
use cranelift_object::{ObjectBuilder, ObjectModule};
use target_lexicon::Triple;
use std::{fs};

pub struct AotCompiler {
    pub module: ObjectModule,
}
impl AotCompiler {
    pub fn new() -> Result<Self, ModuleError> {
        let isa_builder = cranelift_codegen::isa::lookup(Triple::host()).unwrap();
        let isa = isa_builder.finish(settings::Flags::new(settings::builder()))?;
        let obj_builder = ObjectBuilder::new(isa, "prog", default_libcall_names())?;
        let module = ObjectModule::new(obj_builder);
        Ok(Self { module })
    }
    pub fn write(self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let product = self.module.finish();
        let bytes = product.emit()?;
        fs::write(path, bytes)?;
        Ok(())
    }
}
