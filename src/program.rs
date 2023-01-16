use gl46::{GlFns, GL_ACTIVE_UNIFORMS, GL_BOOL, GL_FALSE};
use std::{
    collections::HashMap,
    ffi::{c_int, c_uint},
    ptr::null_mut,
    string::FromUtf8Error,
};

#[derive(Debug, thiserror::Error)]
pub enum ProgramError {
    #[error("Failed to convert error message to string: {0}")]
    ErrorMsgConversion(#[from] FromUtf8Error),
    #[error("No uniform with name {0}")]
    NoSuchUniform(String),
    #[error("Failed to link program: {0}")]
    ProgramLinking(String),
    #[error("Failed to create new program")]
    ProgramCreation,
    #[error("Failed to compile shader: {0}")]
    ShaderCompilation(String),
    #[error("Failed to create new shader")]
    ShaderCreation,
}

pub struct ProgramBuilder<'a> {
    fragment_src: &'a str,
    gl: &'a GlFns,
    vertex_src: &'a str,
}

pub struct Program<'a> {
    gl: &'a GlFns,
    id: c_uint,
    uniforms: HashMap<Box<str>, UniformId>,
}

#[derive(Clone, Copy, Debug)]
pub struct UniformId(c_int);

impl<'a> Program<'a> {
    pub(crate) fn apply(&self) {
        self.gl.UseProgram(self.id);
    }

    pub fn set_uniform(&self, name: &str, value: &impl UniformType) -> bool {
        self.uniforms
            .get(name)
            .map(|uniform_id| {
                value.set_uniform(self, *uniform_id);
            })
            .is_some()
    }

    pub fn update_uniforms(&mut self) {
        let mut count = 0;

        unsafe {
            self.gl
                .GetProgramiv(self.id, GL_ACTIVE_UNIFORMS, &mut count);
        }

        let mut uniforms = HashMap::with_capacity(count as _);

        let mut name = [0u8; 128];
        let mut len = 0;
        let mut size = 0;
        let mut type_ = GL_BOOL;

        for i in 0..count as _ {
            unsafe {
                self.gl.GetActiveUniform(
                    self.id,
                    i,
                    128,
                    &mut len,
                    &mut size,
                    &mut type_,
                    &mut name as _,
                );
            }

            let s = String::from_utf8_lossy(&name[..len as _])
                .to_string()
                .into_boxed_str();

            uniforms.insert(
                s,
                UniformId(unsafe { self.gl.GetUniformLocation(self.id, name.as_ptr().cast()) }),
            );
        }
        self.uniforms = uniforms;
    }

    fn new(gl: &'a GlFns) -> Result<Self, ProgramError> {
        let id = gl.CreateProgram();

        let uniforms = HashMap::new();

        (id != 0)
            .then_some(Self { gl, id, uniforms })
            .ok_or(ProgramError::ProgramCreation)
    }
}

impl Drop for Program<'_> {
    fn drop(&mut self) {
        self.gl.DeleteProgram(self.id);
    }
}

pub struct Shader<'a> {
    gl: &'a GlFns,
    id: c_uint,
}

impl Drop for Shader<'_> {
    fn drop(&mut self) {
        self.gl.DeleteShader(self.id);
    }
}

impl<'a> ProgramBuilder<'a> {
    pub fn build(self) -> Result<Program<'a>, ProgramError> {
        let mut program = Program::new(self.gl)?;

        let vertex = Self::compile_shader(self.gl, self.vertex_src, gl46::GL_VERTEX_SHADER)?;
        let fragment = Self::compile_shader(self.gl, self.fragment_src, gl46::GL_FRAGMENT_SHADER)?;

        self.gl.AttachShader(program.id, vertex.id);
        self.gl.AttachShader(program.id, fragment.id);
        self.gl.LinkProgram(program.id);

        let mut compilation_result = gl46::GL_FALSE.0 as c_int;

        unsafe {
            self.gl
                .GetProgramiv(program.id, gl46::GL_LINK_STATUS, &mut compilation_result);
            self.gl.DetachShader(program.id, vertex.id);
            self.gl.DetachShader(program.id, fragment.id);
        }

        if compilation_result == gl46::GL_FALSE.0 as c_int {
            let mut info_len = 0;

            unsafe {
                self.gl
                    .GetProgramiv(program.id, gl46::GL_INFO_LOG_LENGTH, &mut info_len);
            }

            let mut error = vec![0; info_len as usize];

            unsafe {
                self.gl
                    .GetProgramInfoLog(program.id, info_len, null_mut(), error.as_mut_ptr());
            }

            return Err(ProgramError::ProgramLinking(String::from_utf8(error)?));
        }

        program.update_uniforms();

        Ok(program)
    }

    pub fn new(gl: &'a GlFns, vertex_src: &'a str, fragment_src: &'a str) -> Self {
        Self {
            fragment_src,
            gl,
            vertex_src,
        }
    }

    fn compile_shader(
        gl: &'a GlFns,
        src: &str,
        shader_type: gl46::GLenum,
    ) -> Result<Shader<'a>, ProgramError> {
        let shader = gl.CreateShader(shader_type);

        if shader == 0 {
            return Err(ProgramError::ShaderCreation);
        }

        unsafe {
            gl.ShaderSource(shader, 1, &src.as_ptr(), &(src.len() as c_int));
        };

        gl.CompileShader(shader);

        let mut compilation_result = gl46::GL_FALSE.0 as c_int;

        unsafe {
            gl.GetShaderiv(shader, gl46::GL_COMPILE_STATUS, &mut compilation_result);
        }

        if compilation_result == gl46::GL_FALSE.0 as c_int {
            let mut info_len = 0;

            unsafe {
                gl.GetShaderiv(shader, gl46::GL_INFO_LOG_LENGTH, &mut info_len);
            }

            let mut error = vec![0; info_len as usize];

            unsafe {
                gl.GetShaderInfoLog(shader, info_len, null_mut(), error.as_mut_ptr());
            }

            return Err(ProgramError::ShaderCompilation(String::from_utf8(error)?));
        }

        Ok(Shader { gl, id: shader })
    }
}

pub trait UniformType {
    fn set_uniform(&self, program: &Program, uniform_id: UniformId);
}

impl UniformType for f32 {
    fn set_uniform(&self, program: &Program, uniform_id: UniformId) {
        unsafe {
            program.gl.ProgramUniform1f(program.id, uniform_id.0, *self);
        }
    }
}

impl UniformType for glam::Vec2 {
    fn set_uniform(&self, program: &Program, uniform_id: UniformId) {
        unsafe {
            program
                .gl
                .ProgramUniform2fv(program.id, uniform_id.0, 1, self.to_array().as_ptr());
        }
    }
}

impl UniformType for glam::Mat4 {
    fn set_uniform(&self, program: &Program, uniform_id: UniformId) {
        unsafe {
            program.gl.ProgramUniformMatrix4fv(
                program.id,
                uniform_id.0,
                1,
                GL_FALSE.0 as _,
                self.to_cols_array().as_ptr(),
            )
        }
    }
}
