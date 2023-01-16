use gl46::{
    DrawElementsType, GlFns, GL_ELEMENT_ARRAY_BUFFER, GL_STATIC_DRAW, GL_UNSIGNED_BYTE,
    GL_UNSIGNED_INT, GL_UNSIGNED_SHORT,
};
use std::{ffi::c_uint, marker::PhantomData};

pub struct Indices<'a, I: IndexType, const SIZE: usize> {
    buffer: c_uint,
    gl: &'a GlFns,
    len: usize,
    _p: PhantomData<I>,
}

impl<'a, I: IndexType, const SIZE: usize> Indices<'a, I, SIZE> {
    pub(crate) fn bind(&self) {
        unsafe {
            self.gl.BindBuffer(GL_ELEMENT_ARRAY_BUFFER, self.buffer);
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.len
    }

    pub(crate) fn new(gl: &'a GlFns, indices: &[I; SIZE]) -> Self {
        let mut buffer = 0;

        unsafe {
            gl.GenBuffers(1, &mut buffer);
            gl.BindBuffer(GL_ELEMENT_ARRAY_BUFFER, buffer);
            gl.BufferData(
                GL_ELEMENT_ARRAY_BUFFER,
                (std::mem::size_of::<I>() * indices.len()) as _,
                indices.as_ptr() as _,
                GL_STATIC_DRAW,
            )
        }

        Self {
            buffer,
            gl,
            len: indices.len(),
            _p: PhantomData,
        }
    }
}

impl<I: IndexType, const SIZE: usize> Drop for Indices<'_, I, SIZE> {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteBuffers(1, &self.buffer);
        }
    }
}

pub trait IndexType: sealed::Sealed {
    const GL_TYPE: DrawElementsType;
}

impl IndexType for u8 {
    const GL_TYPE: DrawElementsType = GL_UNSIGNED_BYTE;
}

impl IndexType for u16 {
    const GL_TYPE: DrawElementsType = GL_UNSIGNED_SHORT;
}

impl IndexType for u32 {
    const GL_TYPE: DrawElementsType = GL_UNSIGNED_INT;
}

mod sealed {
    pub trait Sealed {}

    impl Sealed for u8 {}
    impl Sealed for u16 {}
    impl Sealed for u32 {}
}
