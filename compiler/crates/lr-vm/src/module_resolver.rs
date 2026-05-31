use lr_bytecode::Chunk;

pub trait ModuleResolver {
    fn resolve_and_compile(&mut self, import_path: &str, current_file: &str) -> Result<Chunk, String>;
}
