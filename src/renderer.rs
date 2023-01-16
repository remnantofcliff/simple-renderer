use crate::{
    indices::IndexType,
    program::Program,
    vertices::{VertexAttribute, VertexAttributeType},
    Indices, ProgramBuilder, Vertices,
};
use gl46::{GlFns, GL_COLOR_BUFFER_BIT, GL_DEPTH_BUFFER_BIT, GL_TRIANGLES};
use std::{ffi::c_uint, marker::PhantomData, ptr::null};

pub struct Renderer {
    pub(crate) gl: GlFns,
}

impl Renderer {
    pub fn clear(&self) {
        unsafe {
            self.gl.Clear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
        }
    }
    pub fn create_shader_program<'a>(
        &'a self,
        vertex_src: &'a str,
        fragment_src: &'a str,
    ) -> ProgramBuilder {
        ProgramBuilder::new(&self.gl, vertex_src, fragment_src)
    }

    pub fn create_indices<I: IndexType, const SIZE: usize>(
        &self,
        indices: &[I; SIZE],
    ) -> Indices<I, SIZE> {
        Indices::new(&self.gl, indices)
    }

    pub fn create_vertices<T: VertexAttributeType, const AMOUNT: usize, const SIZE: usize>(
        &self,
        data: &[VertexAttribute<T, SIZE>; AMOUNT],
    ) -> Vertices<T, AMOUNT, SIZE> {
        Vertices::new(&self.gl, data)
    }

    pub fn render_vertices(&self, program: &Program) -> VertexStage {
        program.apply();

        VertexStage::new(&self.gl)
    }
    pub fn set_clear_color(&self) {
        unsafe {
            self.gl.ClearColor(0.0, 0.0, 0.0, 1.0);
        }
    }

    pub fn set_viewport(&self, w: i32, h: i32) {
        unsafe {
            self.gl.Viewport(0, 0, w, h);
        }
    }
}

pub struct VertexStage<'a> {
    gl: &'a GlFns,
    layout_index: c_uint,
}

impl<'a> VertexStage<'a> {
    pub fn add_vertices<T: VertexAttributeType, const AMOUNT: usize, const SIZE: usize>(
        mut self,
        vertices: &Vertices<T, AMOUNT, SIZE>,
    ) -> Self {
        vertices.render(self.layout_index);

        self.layout_index += 1;

        self
    }
    pub fn with_indices<I: IndexType, const SIZE: usize>(
        self,
        indices: &Indices<I, SIZE>,
    ) -> IndexStage<'a, I> {
        indices.bind();

        IndexStage {
            gl: self.gl,
            indices_len: indices.len(),
            layout_index: self.layout_index,
            _p: PhantomData,
        }
    }

    fn new(gl: &'a GlFns) -> Self {
        Self {
            gl,
            layout_index: 0,
        }
    }
}

pub struct IndexStage<'a, I: IndexType> {
    gl: &'a GlFns,
    indices_len: usize,
    layout_index: c_uint,
    _p: PhantomData<I>,
}

impl<I: IndexType> IndexStage<'_, I> {
    pub fn finish(self) {
        unsafe {
            self.gl
                .DrawElements(GL_TRIANGLES, self.indices_len as _, I::GL_TYPE, null());

            for i in 0..self.layout_index {
                self.gl.DisableVertexAttribArray(i);
            }
        }
    }
}
