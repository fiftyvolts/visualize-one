use gl;
use std;
use std::ffi::{CStr, CString};
use std::iter::repeat;
use std::path::Path;

use crate::resources::Resources;

pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    pub fn from_source(
        source: &CStr, // modified
        kind: gl::types::GLenum,
    ) -> Result<Self, String> {
        let id = unsafe { gl::CreateShader(kind) };
        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetShaderInfoLog(
                    id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }
        Ok(Self { id })
    }

    pub fn from_vert_source(source: &CStr) -> Result<Self, String> {
        Self::from_source(source, gl::VERTEX_SHADER)
    }

    pub fn from_frag_source(source: &CStr) -> Result<Self, String> {
        Self::from_source(source, gl::FRAGMENT_SHADER)
    }

    pub fn from_res(res: &Resources, name: &Path) -> Result<Shader, String> {
        let source = res
            .load_cstring(name)
            .map_err(|e| format!("Could not load {:?}: {:?}", name, e))?;

        match name.extension().unwrap().to_str() {
            Some("vert") => Self::from_vert_source(&source),
            Some("frag") => Self::from_frag_source(&source),
            _ => Err(format!("Unable to load from {:?}", name)),
        }
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}

fn create_whitespace_cstring_with_len(len: usize) -> CString {
    unsafe { CString::from_vec_unchecked(repeat(b' ').take(len).collect()) }
}

pub struct Program {
    id: gl::types::GLuint,
}

impl Program {
    pub fn from_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe {
                gl::AttachShader(program_id, shader.id());
            }
        }

        unsafe {
            gl::LinkProgram(program_id);
        }

        let mut success: gl::types::GLint = 1;

        unsafe {
            gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl::GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        // continue with error handling here

        for shader in shaders {
            unsafe {
                gl::DetachShader(program_id, shader.id());
            }
        }

        Ok(Program { id: program_id })
    }

    pub fn from_res(res: &Resources,  name: &Path) -> Result<Program, String> {
        const EXT_TYPES: [&str; 2] = [
            "vert",
            "frag"
        ];

        let shaders = EXT_TYPES.iter()
        .map(|ext| Shader::from_res(res, &name.with_extension(ext)))
        .collect::<Result<Vec<Shader>, String>>()?;

        Program::from_shaders(&shaders[..])
    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn set_used(&self) {
        unsafe {
            gl::UseProgram(self.id());
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
