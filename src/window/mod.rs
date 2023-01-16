mod event;

use crate::Renderer;
use fermium::{error::*, prelude::*};
use gl46::GlFns;
use gl46::GL_CULL_FACE;
use gl46::GL_DEPTH_TEST;
use gl46::GL_LESS;
use std::ffi::CString;
use std::ptr::NonNull;

pub use event::Event;
pub use event::Key;

#[derive(Debug, thiserror::Error)]
pub enum WindowError {
    #[error("Window system initialization failed: {0}")]
    Initialization(String),
    #[error("Failed to set OpenGL version: {0}")]
    OpenGLVersion(String),
    #[error("Failed ")]
    OpenGLContext(String),
    #[error("Failed to create window: {0}")]
    WindowCreation(String),
}

pub struct Window {
    gl_ctx: NonNull<c_void>,
    window: NonNull<SDL_Window>,
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe {
            SDL_GL_DeleteContext(SDL_GLContext(self.gl_ctx.as_ptr()));
            SDL_DestroyWindow(self.window.as_ptr());
            SDL_Quit();
        }
    }
}

impl Window {
    pub fn create_renderer(&self) -> Result<Renderer, &'static str> {
        let renderer = Renderer {
            gl: unsafe { GlFns::load_from(&|proc| SDL_GL_GetProcAddress(proc.cast()))? },
        };

        unsafe {
            renderer.gl.Enable(GL_CULL_FACE);
            renderer.gl.Enable(GL_DEPTH_TEST);
            renderer.gl.DepthFunc(GL_LESS);
        }

        Ok(renderer)
    }

    pub fn new(title: &str, width: i32, height: i32) -> Result<Self, WindowError> {
        const ERROR_BUFFER_SIZE: c_int = 256;

        let mut error_buffer = String::with_capacity(ERROR_BUFFER_SIZE as usize);
        let title = CString::new(title).unwrap();

        let (window, gl_ctx) = {
            unsafe {
                if SDL_Init(SDL_INIT_VIDEO) != 0 {
                    SDL_GetErrorMsg(error_buffer.as_mut_ptr().cast(), ERROR_BUFFER_SIZE);

                    return Err(WindowError::Initialization(error_buffer));
                }

                if SDL_GL_SetAttribute(SDL_GL_CONTEXT_MAJOR_VERSION, 4)
                    + SDL_GL_SetAttribute(SDL_GL_CONTEXT_MINOR_VERSION, 6)
                    + SDL_GL_SetAttribute(
                        SDL_GL_CONTEXT_PROFILE_MASK,
                        SDL_GL_CONTEXT_PROFILE_CORE.0 as _,
                    )
                    != 0
                {
                    SDL_GetErrorMsg(error_buffer.as_mut_ptr().cast(), ERROR_BUFFER_SIZE);

                    return Err(WindowError::OpenGLVersion(error_buffer));
                }
            }

            let Some(window) = NonNull::new(unsafe {
                SDL_CreateWindow(
                    title.as_ptr().cast(),
                    SDL_WINDOWPOS_CENTERED,
                    SDL_WINDOWPOS_CENTERED,
                    width,
                    height,
                    (SDL_WINDOW_SHOWN | SDL_WINDOW_OPENGL | SDL_WINDOW_RESIZABLE).0,
                )
            })
            else {
                unsafe {
                    SDL_GetErrorMsg(error_buffer.as_mut_ptr().cast(), ERROR_BUFFER_SIZE);
                }

                return Err(WindowError::WindowCreation(error_buffer));
            };

            let Some(gl_ctx) = NonNull::new(unsafe {SDL_GL_CreateContext(window.as_ptr()).0 })
            else {
                unsafe {
                    SDL_GetErrorMsg(error_buffer.as_mut_ptr().cast(), ERROR_BUFFER_SIZE);
                }

                return Err(WindowError::OpenGLContext(error_buffer));
            };

            (window, gl_ctx)
        };

        Ok(Self { gl_ctx, window })
    }
    pub fn next_event(&self) -> Option<Event> {
        let mut event = SDL_Event::default();

        if unsafe { SDL_PollEvent(&mut event) } == 1 {
            Some(Event::from_sdl_event(event))
        } else {
            None
        }
    }

    pub fn size(&mut self) -> (c_int, c_int) {
        let mut w = 0;
        let mut h = 0;

        unsafe {
            SDL_GetWindowSize(self.window.as_mut(), &mut w, &mut h);
        }

        (w, h)
    }

    pub fn swap_buffers(&mut self) {
        unsafe { SDL_GL_SwapWindow(self.window.as_mut()) }
    }
}
