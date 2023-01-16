use gl46::{GlFns, VertexAttribPointerType, GL_ARRAY_BUFFER, GL_FALSE, GL_FLOAT, GL_STATIC_DRAW};
use std::{ffi::c_uint, marker::PhantomData, ptr::null};

#[repr(transparent)]
pub struct VertexAttribute<T: VertexAttributeType, const SIZE: usize>([T; SIZE]);

impl<T: VertexAttributeType, const SIZE: usize> VertexAttribute<T, SIZE> {
    pub const fn new(data: [T; SIZE]) -> Self {
        Self(data)
    }
}

pub struct Vertices<'a, T: VertexAttributeType, const AMOUNT: usize, const SIZE: usize> {
    buffer: c_uint,
    gl: &'a GlFns,
    vao: c_uint,
    _p: PhantomData<T>,
}

impl<'a, T: VertexAttributeType, const AMOUNT: usize, const SIZE: usize>
    Vertices<'a, T, AMOUNT, SIZE>
{
    pub(crate) fn render(&self, layout_index: c_uint) {
        unsafe {
            self.gl.EnableVertexAttribArray(layout_index);
            self.gl.BindBuffer(GL_ARRAY_BUFFER, self.buffer);
            self.gl.VertexAttribPointer(
                layout_index,
                SIZE as _,
                T::GL_TYPE,
                GL_FALSE.0 as _,
                0,
                null(),
            );
        }
    }
    pub(crate) fn new(gl: &'a GlFns, vertex_data: &[VertexAttribute<T, SIZE>; AMOUNT]) -> Self {
        let mut vao = 0;

        unsafe {
            gl.GenVertexArrays(1, &mut vao);
            gl.BindVertexArray(vao);
        }

        let mut buffer = 0;

        unsafe {
            gl.GenBuffers(1, &mut buffer);
            gl.BindBuffer(GL_ARRAY_BUFFER, buffer);
            gl.BufferData(
                GL_ARRAY_BUFFER,
                std::mem::size_of::<[VertexAttribute<T, SIZE>; AMOUNT]>() as _,
                vertex_data.as_ptr() as _,
                GL_STATIC_DRAW,
            );
        }

        Self {
            buffer,
            gl,
            vao,
            _p: PhantomData,
        }
    }
}

impl<T: VertexAttributeType, const AMOUNT: usize, const SIZE: usize> Drop
    for Vertices<'_, T, AMOUNT, SIZE>
{
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteBuffers(1, &self.buffer);
            self.gl.DeleteVertexArrays(1, &self.vao);
        }
    }
}

pub trait VertexAttributeType {
    const GL_TYPE: VertexAttribPointerType;
}

impl VertexAttributeType for f32 {
    const GL_TYPE: VertexAttribPointerType = GL_FLOAT;
}
