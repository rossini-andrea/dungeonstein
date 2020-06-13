//! The module expose various OpenGL boilerplate wrappers.
use std::convert::TryInto;
use std::path::Path;
use std::ffi::c_void;
use gl;
use gl::types::{
    GLchar, GLenum, GLuint, GLint, GLfloat,
    GLsizeiptr
};
use sdl2::{
    surface::Surface,
    image::{LoadSurface, LoadTexture},
    render::{Canvas, Texture, TextureCreator},
    video::WindowContext
};

pub trait GlBindable {
    fn bind(&self);
    fn unbind(&self);
}

pub struct Bind<'a> {
    object: &'a dyn GlBindable
}

impl<'a> Bind<'a> {
    pub fn new(object: &'a dyn GlBindable) -> Self {
        object.bind();
        Self { object }
    }
}

impl<'a> Drop for Bind<'a> {
    fn drop(&mut self) {
        self.object.unbind();
    }
}

pub fn texture_from_file<'r, P: AsRef<Path>>(texture_creator: & 'r TextureCreator<WindowContext>, path: P) -> Texture<'r> {
    let mut texture = texture_creator.load_texture(path).unwrap();
    unsafe{
        texture.gl_bind_texture();
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT.try_into().unwrap());
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT.try_into().unwrap());
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST.try_into().unwrap());
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST.try_into().unwrap());
        texture.gl_unbind_texture();
    }

    texture
}
/*
pub struct GlTexture {
    handle: GLuint
}

impl GlTexture {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let image: Surface = match LoadSurface::from_file(path) {
            Ok(s) => s,
            Err(e) => panic!("Failed to load image surface: {}", e.to_string()),
        };

        // Ugly hack
        image.
        let raw_bytes = image.buffer.as_bytes();
        let mut inversion = Vec::with_capacity(image.width * image.height * 4);

        for y in (0..image.height).rev() {
            let start = y * image.width * 4;
            let end = start + image.width * 4;
            inversion.extend_from_slice(&raw_bytes[start..end]);
        }

        Self::from_raw_rgba(image.width, image.height, &inversion)
    }

    pub fn from_raw_rgba(width: usize, height: usize,
        data: &[u8]) -> Result<Self, String> {
        let mut handle: GLuint = 0;

        unsafe {
            gl::GenTextures(1, &mut handle);
        }

        let result = GlTexture { handle };

        {
            let mut _bind = Bind::new(&result);
            unsafe {
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT.try_into().unwrap());
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT.try_into().unwrap());
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST.try_into().unwrap());
                gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST.try_into().unwrap());
                gl::TexImage2D(
                    gl::TEXTURE_2D, 0, gl::RGBA as i32,
                    width as i32, height as i32, 0, gl::RGBA,
                    gl::UNSIGNED_BYTE, data.as_ptr() as *const c_void);
                gl::GenerateMipmap(gl::TEXTURE_2D);
            }
        }

        Ok(result)
    }

    pub fn detach(&mut self) -> GLuint {
        let h = self.handle;
        self.handle = 0;
        h
    }
}*/

/*
impl Drop for GlTexture {
    fn drop(&mut self) {
        if self.handle == 0 { return; }

        unsafe {
            gl::DeleteTextures(1, &self.handle);
        }
        self.handle = 0;
    }
}*/

/// A gl shader.
pub struct GlShader {
    handle: GLuint
}

impl GlShader {
    /// Loads a glsl shader from a text file.
    /// * `path`: source glsl file.
    /// Returns: `Result<GlShader, String>`.
    ///
    /// The type of shader is derived from the heading comment in the source file.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let shader_source = match std::fs::read(path) {
            Ok(s) => s,
            Err(error) => {
                return Err(format!("{}", error));
            }
        };

        let shader_type: GLenum;

        if shader_source.starts_with(b"///totw vertex_shader") {
            shader_type = gl::VERTEX_SHADER;
        } else if shader_source.starts_with(b"///totw fragment_shader") {
            shader_type = gl::FRAGMENT_SHADER;
        } else {
            return Err("Unrecognized shader type.".to_string());
        }

        let handle = unsafe{ gl::CreateShader(shader_type) };

        if handle == 0 {
            let err_num = unsafe { gl::GetError() };
            return Err(format!("OpenGL error {}", err_num));
        }

        // Pass in and compile the shader source.
        unsafe{
            gl::ShaderSource(
                handle,
                1,
                [shader_source.as_ptr()].as_ptr() as *const *const i8,
                [shader_source.len() as GLint].as_ptr()
            );
            gl::CompileShader(handle);
        }

        // Get the compilation status.
        let mut status: GLint = 0;

        unsafe { gl::GetShaderiv(handle, gl::COMPILE_STATUS, &mut status as *mut GLint); }

        // If the compilation failed, delete the shader.
        if status == 0 {
            let mut log_len: GLint = 0;
            let mut out_len: GLint = 0;
            unsafe {gl::GetShaderiv(handle, gl::INFO_LOG_LENGTH, &mut log_len as *mut GLint);}
            let mut log = vec!(0u8; log_len.try_into().unwrap());
            unsafe {
                gl::GetShaderInfoLog(handle, log_len, &mut out_len as *mut GLint, log.as_mut_ptr() as *mut GLchar);
                gl::DeleteShader(handle);
            }
            return Err(format!("{}", std::str::from_utf8(&log).unwrap_or_default()));
        }

        Ok(GlShader { handle })
    }
}

impl Drop for GlShader {
    fn drop(&mut self) {
        if self.handle == 0 { return; }

        unsafe {
            gl::DeleteShader(self.handle);
        }
        self.handle = 0;
    }
}

#[macro_export]
macro_rules! attrib_bindings {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(($x, String::from(stringify!($x)) + "\0"));
            )*
            temp_vec
        }
    };
}

pub struct GlShaderProgram {
    handle: GLuint
}

impl GlShaderProgram {
    pub fn new(shaders: &[GlShader],
        attrib_bindings: &[(GLuint, String)]) -> Result<Self, String> {
        let handle = unsafe { gl::CreateProgram() };

        for shader in shaders {
            unsafe { gl::AttachShader(handle, shader.handle); }
        }

        for (binding, name) in attrib_bindings {
            unsafe { gl::BindAttribLocation(handle, *binding, name.as_ptr() as *const i8); }
        }

        unsafe { gl::LinkProgram(handle); }

        let mut status: GLint = 0;

        unsafe { gl::GetProgramiv(handle, gl::LINK_STATUS, &mut status as *mut GLint); }

        // If the link failed, delete the program.
        if status == 0 {
            let mut log_len: GLint = 0;
            let mut out_len: GLint = 0;
            unsafe {gl::GetProgramiv(handle, gl::INFO_LOG_LENGTH, &mut log_len as *mut GLint);}
            let mut log = vec!(0u8; log_len.try_into().unwrap());
            unsafe {
                gl::GetProgramInfoLog(handle, log_len, &mut out_len as *mut GLint,
                    log.as_mut_ptr() as *mut GLchar);
                gl::DeleteProgram(handle);
            }
            return Err(format!("{}", std::str::from_utf8(&log).unwrap_or_default()));
        }

        for shader in shaders {
            unsafe { gl::DetachShader(handle, shader.handle); }
        }

        Ok(GlShaderProgram { handle })
    }

    /// Retrieves the location for GL uniform
    pub fn uniform_location(&self, name: &str) -> GLint {
        unsafe { gl::GetUniformLocation(self.handle, name.as_ptr() as *const i8) }
    }

    pub fn detach(&mut self) -> GLuint {
        let h = self.handle;
        self.handle = 0;
        h
    }
}

impl GlBindable for GlShaderProgram {
    fn bind(&self) {
        unsafe {
            gl::UseProgram(self.handle);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::UseProgram(0);
        }
    }
}

impl Drop for GlShaderProgram {
    fn drop(&mut self) {
        if self.handle == 0 { return; }

        unsafe {
            gl::DeleteProgram(self.handle);
        }

        self.handle = 0;
    }
}

pub struct GlVertexArray {
    handle: GLuint,
    vertex_buffer_handle: GLuint,
    element_buffer_handle: GLuint
}

impl GlVertexArray {
    /// Creates an OpenGL Vertex Array from a float buffer and attributes
    /// definitions.
    /// * `data`: slice of floats to load.
    /// * `indices`: element array indices
    /// * `attributes`: slice of tuples, where:
    ///   * `attribute`: name of attribute
    ///   * `size`: count of elements in the `data` buffer
    pub fn from_vertex_buffer(
        data: &[GLfloat], indices: &[GLuint], attributes: &[(GLuint, usize)]
    ) -> Self {
        const SIZE_OF_FLOAT: usize = std::mem::size_of::<GLfloat>();
        const SIZE_OF_UINT: usize = std::mem::size_of::<GLuint>();

        let total_vertex_size = attributes
            .iter()
            .fold(0, |acc, (_, y)| acc + y);

        assert_eq!(data.len() % total_vertex_size, 0);

        let mut handle: GLuint = 0;
        let mut vertex_buffer_handle: GLuint = 0;
        let mut element_buffer_handle: GLuint = 0;
        let s: GLsizeiptr = (data.len() * SIZE_OF_FLOAT).try_into().unwrap();

        unsafe {
            gl::GenVertexArrays(1, &mut handle);
            gl::GenBuffers(1, &mut vertex_buffer_handle);
            gl::GenBuffers(1, &mut element_buffer_handle);
            gl::BindVertexArray(handle);
            gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer_handle);
            gl::BufferData(gl::ARRAY_BUFFER, s,
                data.as_ptr() as *const c_void, gl::STATIC_DRAW);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, element_buffer_handle);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() * SIZE_OF_UINT).try_into().unwrap(),
                indices.as_ptr() as *const c_void, gl::STATIC_DRAW);
        }

        let mut pointer = 0;

        for (attribute, size) in attributes {
            unsafe {
                gl::VertexAttribPointer(
                    *attribute, (*size).try_into().unwrap(),
                    gl::FLOAT, gl::FALSE,
                    (total_vertex_size * SIZE_OF_FLOAT)
                        .try_into().unwrap(),
                    (pointer * SIZE_OF_FLOAT) as *const c_void);
                gl::EnableVertexAttribArray(*attribute);
            }

            pointer += size;
        }

        unsafe { gl::BindVertexArray(0); }

        Self { handle, vertex_buffer_handle, element_buffer_handle }
    }
}

impl GlBindable for GlVertexArray {
    fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.handle);
        }
    }

    fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for GlVertexArray {
    fn drop(&mut self) {
        if self.handle == 0 { return; }

        unsafe {
            gl::DeleteVertexArrays(1, &self.handle);
            gl::DeleteBuffers(1, &self.vertex_buffer_handle);
            gl::DeleteBuffers(1, &self.element_buffer_handle);
        }
    }
}
