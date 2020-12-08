//! # Nagini Internal
//! Uses naga to load shaders to make pipelines

#[derive(Debug)]
pub struct Pipeline {
    pub name: String,
    pub shaders: Vec<ShaderData>,
}

#[derive(Debug)]
pub struct ShaderData {
    pub name: String,
    pub spirv: Vec<u8>,
    pub module: naga::Module,
}