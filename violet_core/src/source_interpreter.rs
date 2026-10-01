use std::slice;

use xpans_render::prelude::InterpretSource;

use crate::Source;

/// Evaluates an audio source's properties into values used by a sample processor.
pub trait SourceInterpreter<T>
where
    Self: InterpretSource<Source<T>>,
{
}
impl<T, S> SourceInterpreter<S> for T where T: ?Sized + InterpretSource<Source<S>> {}

pub trait GetInterpretation<T> {
    // fn get(&self, source: usize, index: usize) -> &T;
    // fn get_mut(&mut self, source: usize, index: usize) -> &mut T;
    // fn set(&mut self, source: usize, index: usize, value: T);
    fn get_slice(&self) -> &[T];
    fn get_slice_mut(&mut self) -> &mut [T];
}

pub struct TypedStorage<T> {
    data: Box<[T]>,
}

pub struct ErasedStorage {
    data: Box<[u8]>,
    // layout: core::alloc::Layout,
    // length: usize,
}

impl ErasedStorage {
    pub fn new<T>(sources: usize, length: usize) -> Self
    where
        T: Sized + Default + Clone,
    {
        let t_size = size_of::<T>();
        // let t_align = size_of::<T>();
        // let layout = unsafe { core::alloc::Layout::from_size_align_unchecked(t_size, t_align) };
        // let byte_length = t_size * sources * length;
        let typed_vec = vec![T::default(); sources * length];
        let (v_pointer, v_length, v_capacity) = typed_vec.into_raw_parts();
        let data = unsafe {
            Vec::<u8>::from_raw_parts(v_pointer as *mut u8, v_length * t_size, v_capacity * t_size)
                .into_boxed_slice()
        };
        Self {
            data,
            // layout,
            // length,
        }
    }
    fn typed_slice<T: Sized>(&self) -> &[T] {
        let ptr = self.data.as_ptr() as *const T;
        let len = size_of::<T>() / self.data.len();
        unsafe { slice::from_raw_parts(ptr, len) }
    }
    fn typed_slice_mut<T: Sized>(&mut self) -> &mut [T] {
        let ptr = self.data.as_mut_ptr() as *mut T;
        let len = size_of::<T>() / self.data.len();
        unsafe { slice::from_raw_parts_mut(ptr, len) }
    }
    // fn index<T: Sized>(&self, source: usize, index: usize) -> usize {
    //     // Simplified (source * (length * size)) + (index * size):
    //     self.layout.size() * ((source * self.length) + index)
    // }
    // fn source(&self, source: usize) -> usize {
    //     self.layout.size() * (source * self.length)
    // }
}

impl<T: Sized> GetInterpretation<T> for ErasedStorage {
    // fn get(&self, source: usize, index: usize) -> &T {
    //     let byte_index = self.index(source, index);
    //     let refr = unsafe { self.data.get_unchecked(byte_index) };
    //     let p = ptr::from_ref(refr) as *const T;
    //     unsafe { &*p }
    // }
    //
    //     fn get_mut(&mut self, source: usize, index: usize) -> &mut T {
    //         let byte_index = self.index(source, index);
    //         let refr = unsafe { self.data.get_unchecked_mut(byte_index) };
    //         let p = ptr::from_mut(refr) as *mut T;
    //         unsafe { &mut *p }
    //     }
    //
    //     fn set(&mut self, source: usize, index: usize, value: T) {
    //         *self.get_mut(source, index) = value;
    //     }

    fn get_slice(&self) -> &[T] {
        self.typed_slice()
    }

    fn get_slice_mut(&mut self) -> &mut [T] {
        self.typed_slice_mut()
    }
}
