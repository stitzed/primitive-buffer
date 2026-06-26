use std::ops::Deref;
use std::mem::MaybeUninit;
use std::fmt::Debug;

/// A stack-based buffer with fixed capacity for primitives.
/// 
/// Uses uninitialized memory to avoid the cost of filling the buffer with default values.
/// Only the first `len` elements are guaranteed to be initialized.
/// # Examples
/// ```
/// # use primitive_buffer::Buffer;
/// let mut buffer: Buffer<u8, 8> = Buffer::new();
/// 
/// buffer.push(1);
/// buffer.push(2);
/// 
/// assert_eq!(buffer.len(), 2);
/// 
/// assert_eq!(buffer.pop(), Some(2));
/// assert_eq!(buffer.len(), 1);
/// 
/// buffer.clear();
/// 
/// assert!(buffer.is_empty());
/// ```
pub struct Buffer<T: Copy, const N: usize> {
    buffer: [MaybeUninit<T>; N],
    len: usize
}

impl<T: Copy, const N: usize> Buffer<T, N> {
    /// Creates a new empty buffer on the stack with a fixed capacity.
    /// 
    /// # Examples
    /// ```
    /// # use primitive_buffer::Buffer;
    /// let mut buf: Buffer<u8, 4> = Buffer::new();
    /// assert!(buf.is_empty());
    /// ```
    /// 
    /// # Complexity
    /// `O(1)`
    #[inline(always)]
    pub fn new() -> Self {
        Self { buffer: [MaybeUninit::uninit(); N], len: 0 }
    }
    
    /// Appends an element to the back of the buffer.
    /// 
    /// # Examples
    /// ```
    /// # use primitive_buffer::Buffer;
    /// let mut buf: Buffer<u8, 2> = Buffer::new();
    /// buf.push(10);
    /// buf.push(20);
    /// assert_eq!(buf.as_slice(), &[10, 20]);
    /// ```
    /// 
    /// # Panics
    /// Panics when trying to add an element with full capacity.
    /// 
    /// # Complexity
    /// `O(1)`
    #[inline(always)]
    pub fn push(&mut self, item: T) {
        if self.len >= N {
            panic!("buffer overflow: capacity {} reached", N)
        }

        self.buffer[self.len].write(item);
        self.len += 1;
    }
    
    /// Appends an element to the back of the buffer without checking capacity.
    /// 
    /// # Examples
    /// ```
    /// # use primitive_buffer::Buffer;
    /// let mut buf: Buffer<u8, 2> = Buffer::new();
    /// unsafe {
    ///     buf.push_unchecked(42);
    /// }
    /// assert_eq!(buf.len(), 1);
    /// ```
    /// 
    /// # Safety
    /// The caller must ensure that the buffer is not full. Calling this method
    /// when `len() == N` is undefined behavior.
    /// 
    /// # Complexity
    /// `O(1)`
    #[inline(always)]
    pub unsafe fn push_unchecked(&mut self, item: T) {
        debug_assert!(self.len < N, "buffer overflow: capacity {} reached", N);
        
        unsafe {
            self.buffer.get_unchecked_mut(self.len)
        }.write(item);

        self.len += 1;
    }

    /// Returns the last element, if there is one, and removes it from the buffer.
    /// 
    /// # Examples
    /// ```
    /// # use primitive_buffer::Buffer;
    /// let mut buf: Buffer<u8, 2> = Buffer::new();
    /// buf.push(10);
    /// assert_eq!(buf.pop(), Some(10));
    /// assert_eq!(buf.pop(), None);
    /// ```
    /// 
    /// # Complexity
    /// `O(1)`
    #[inline(always)]
    pub fn pop(&mut self) -> Option<T> {
        if self.len > 0 {
            self.len -= 1;
            Some(unsafe {
                self.buffer[self.len].assume_init()
            })
        } else {
            None
        }
    }

    /// Removes and returns the last element from the buffer without checking if it is empty.
    /// 
    /// # Examples
    /// ```
    /// # use primitive_buffer::Buffer;
    /// let mut buf: Buffer<u8, 2> = Buffer::new();
    /// buf.push(5);
    /// unsafe {
    ///     assert_eq!(buf.pop_unchecked(), 5);
    /// }
    /// ```
    /// 
    /// # Safety
    /// The caller must ensure that the buffer is not empty. Calling this method
    /// when `is_empty()` is true is undefined behavior.
    /// 
    /// # Complexity
    /// `O(1)`
    #[inline(always)]
    pub unsafe fn pop_unchecked(&mut self) -> T {
        debug_assert!(self.len > 0, "buffer is empty");
        
        self.len -= 1;
        unsafe { self.buffer.get_unchecked(self.len).assume_init() }
    }
    
    /// Clears the buffer.
    /// 
    /// # Examples
    /// ```
    /// # use primitive_buffer::Buffer;
    /// let mut buf: Buffer<u8, 3> = Buffer::new();
    /// buf.push(1);
    /// buf.clear();
    /// assert!(buf.is_empty());
    /// ```
    /// 
    /// # Complexity
    /// `O(1)`
    #[inline(always)]
    pub fn clear(&mut self) {
        self.len = 0;
    }

    /// Returns the buffer as a slice.
    /// 
    /// # Examples
    /// ```
    /// # use primitive_buffer::Buffer;
    /// let mut buf: Buffer<u8, 3> = Buffer::new();
    /// buf.push(1);
    /// buf.push(2);
    /// assert_eq!(buf.as_slice(), &[1, 2]);
    /// ```
    /// 
    /// # Complexity
    /// `O(1)`
    #[inline(always)]
    pub fn as_slice(&self) -> &[T] {
        unsafe {
            let arr_ptr: *const T = self.buffer.as_ptr() as *const T;
            std::slice::from_raw_parts(arr_ptr, self.len)
        }
    }
    
    /// Checks if the buffer is empty.
    /// 
    /// # Examples
    /// ```
    /// # use primitive_buffer::Buffer;
    /// let mut buf: Buffer<u8, 2> = Buffer::new();
    /// assert!(buf.is_empty());
    /// buf.push(1);
    /// assert!(!buf.is_empty());
    /// ```
    /// 
    /// # Complexity
    /// `O(1)`
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns `true` if the buffer is full.
    /// 
    /// # Examples
    /// ```
    /// # use primitive_buffer::Buffer;
    /// let mut buf: Buffer<u8, 1> = Buffer::new();
    /// assert!(!buf.is_full());
    /// buf.push(10);
    /// assert!(buf.is_full());
    /// ```
    /// 
    /// # Complexity
    /// `O(1)`
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        self.len == N
    }

    /// Returns the length of the buffer.
    /// 
    /// # Examples
    /// ```
    /// # use primitive_buffer::Buffer;
    /// let mut buf: Buffer<u8, 5> = Buffer::new();
    /// assert_eq!(buf.len(), 0);
    /// buf.push(10);
    /// assert_eq!(buf.len(), 1);
    /// ```
    /// 
    /// # Complexity
    /// `O(1)`
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns the total capacity of the buffer.
    /// 
    /// # Examples
    /// ```
    /// # use primitive_buffer::Buffer;
    /// let buf: Buffer<u8, 5> = Buffer::new();
    /// assert_eq!(buf.capacity(), 5);
    /// ```
    /// 
    /// # Complexity
    /// `O(1)`
    #[inline(always)]
    pub fn capacity(&self) -> usize {
        N
    }
}

impl<T: Copy, const N: usize> Deref for Buffer<T, N> {
    type Target = [T];
    
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T: Copy + Debug, const N: usize> Debug for Buffer<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.as_slice()).finish()
    }
}

/// Creates a [`Buffer`] containing the given arguments.
///
/// `buf!` allows shorthand initialization of a buffer. It supports two syntax 
/// variants:
/// 
/// 1. Specifying the capacity explicitly as a macro argument.
/// 2. Allowing the capacity to be inferred by the compiler.
///
/// # Examples
///
/// Creating a buffer with an explicit fixed capacity via the macro argument:
///
/// ```
/// # use primitive_buffer::{Buffer, buf};
/// let b = buf![10; 1, 2];
/// ```
///
/// Creating a buffer where the capacity and type are inferred from the context:
///
/// ```
/// # use primitive_buffer::{Buffer, buf};
/// let b: Buffer<i32, 5> = buf![1, 2, 3];
/// assert_eq!(b.len(), 3);
/// ```
///
/// # Panics
///
/// This macro will panic at runtime if the number of passed elements exceeds the 
/// allocated capacity.
#[macro_export]
macro_rules! buf {
    ($cap:expr; $($element:expr),*) => {
        {
            let mut buffer = Buffer::<_, $cap>::new();
            
            $(
                buffer.push($element);
            )*
            
            buffer
        }
    };

    ($($element:expr),*) => {
        {
            let mut buffer = Buffer::new();
            
            $(
                buffer.push($element);
            )*
            
            buffer
        }
    };
}