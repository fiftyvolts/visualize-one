extern crate gl;
extern crate sdl2;

use std;
use std::ffi::CString;

pub mod render_gl;

#[derive(Debug,Clone,Copy)]
struct Vertex {
    data: [f32;6]
}

impl Vertex {
    fn new(x: f32, y: f32, z: f32, r: f32, g: f32, b: f32) -> Self {
        Vertex { data: [x, y, z, r, g, b] }
    }

    fn set_pos(&mut self, x: f32, y: f32, z: f32) {
        self.data[0] = x;
        self.data[1] = y;
        self.data[2] = z;
    }

    fn set_color(&mut self, r: f32, g: f32, b: f32) {
        self.data[3] = r;
        self.data[4] = g;
        self.data[5] = b;
    }
}

impl FromIterator<Vertex> for Vec<f32> {
    fn from_iter<T: IntoIterator<Item = Vertex>>(iter: T) -> Self {
        let mut v = vec![];
        for item in iter {
            v.extend_from_slice(&item.data);
        }
        v
    }
}

fn main() {
    let sdl = sdl2::init().unwrap();

    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video_subsystem
        .window("Game", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    let _gl =
        gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    let mut event_pump = sdl.event_pump().unwrap();

    unsafe {
        gl::Viewport(0, 0, 900, 700); // set viewport
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }

    let vert_shader =
        render_gl::Shader::from_vert_source(&CString::new(include_str!("triangle.vert")).unwrap())
            .unwrap();

    let frag_shader =
        render_gl::Shader::from_frag_source(&CString::new(include_str!("triangle.frag")).unwrap())
            .unwrap();

    let shader_program = render_gl::Program::from_shaders(&[vert_shader, frag_shader]).unwrap();

    let vertices = [
        Vertex::new(-0.5, -0.5, 0.0, 1.0, 0.0, 1.0),
        Vertex::new(0.5, -0.5, 0.0, 0.0, 1.0, 0.0),
        Vertex::new(0.0, 0.5, 0.0, 0.0, 0.0, 1.0),
    ];

    let vbuff: Vec<f32> = vertices.iter().cloned().collect();

    let mut vbo: gl::types::GLuint = 0;

    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,                                                       // target
            (vbuff.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr, // size of data in bytes
            vbuff.as_ptr() as *const gl::types::GLvoid, // pointer to data
            gl::STATIC_DRAW,                               // usage
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer
    }

    let mut vao: gl::types::GLuint = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::EnableVertexAttribArray(0); // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            0,         // index of the generic vertex attribute ("layout (location = 0)")
            3,         // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
            std::ptr::null(),                                     // offset of the first component
        );

        gl::EnableVertexAttribArray(1); // this is "layout (location = 0)" in vertex shader
     gl::VertexAttribPointer(
        1, // index of the generic vertex attribute ("layout (location = 0)")
        3, // the number of components per generic vertex attribute
        gl::FLOAT, // data type
        gl::FALSE, // normalized (int-to-float conversion)
        (6 * std::mem::size_of::<f32>()) as gl::types::GLint, // stride (byte offset between consecutive attributes)
        (3 * std::mem::size_of::<f32>()) as *const gl::types::GLvoid // offset of the first component
    );

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }


    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        shader_program.set_used();
        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0,             // starting index in the enabled arrays
                3,             // number of indices to be rendered
            );
        }

        window.gl_swap_window();
    }
}
