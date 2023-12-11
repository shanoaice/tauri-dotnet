use std::mem::ManuallyDrop;

/// This represents a Rust-owned UTF16 string that is safe to be passed to CLR languages
/// and be modified in place, as long as len and capacity is respected.
/// In .NET, you can consume this using `Span<char>` or `ReadOnlySpan<char>`,
/// or you can use `Marshal.PtrToStringUni(IntPtr, int32)` to allocate a new CLR String
/// After consuming this string, you have to either return this string as is,
/// or call `OwnedUtf16String::free` to free the memory by Rust's allocator
#[repr(C)]
pub struct OwnedUtf16String {
    chars: *mut u16,
    len: usize,
    capacity: usize,
    free: extern "C" fn(*mut u16, usize, usize),
}

extern "C" fn free_owned_utf16_string(chars: *mut u16, len: usize, capacity: usize) {
    let char_vec = unsafe { Vec::from_raw_parts(chars, len, capacity) };
    drop(char_vec);
}

impl From<String> for OwnedUtf16String {
    fn from(s: String) -> Self {
        let char_vec = s.encode_utf16().collect::<Vec<u16>>();
        let len = char_vec.len();
        let capacity = char_vec.capacity();
        let chars = char_vec.leak().as_mut_ptr();
        OwnedUtf16String {
            chars,
            len,
            capacity,
            free: free_owned_utf16_string,
        }
    }
}

impl From<OwnedUtf16String> for String {
    fn from(value: OwnedUtf16String) -> Self {
        let s = ManuallyDrop::new(value);
        unsafe {
            let char_vec = Vec::from_raw_parts(s.chars, s.len, s.capacity);
            String::from_utf16_lossy(&char_vec)
        }
    }
}

impl Drop for OwnedUtf16String {
    fn drop(&mut self) {
        let char_vec = unsafe { Vec::from_raw_parts(self.chars, self.len, self.capacity) };
        drop(char_vec);
    }
}

/// This represents a owned Rust string that is safe to be passed to CLR languages.
/// Modification in-place is generally not recommended, as this is UTF-8.
/// In .NET, the best use case is to call `Encoding.UTF8.GetString(byte_array)` to get a new CLR String
/// After consuming this string, you have to either return this string as is,
/// or call `OwnedUtf16String::free` to free the memory by Rust's allocator
///
/// This is the recommended way to pass strings to .NET, since this path only require 1 alloc and 1 encode
/// Whereas passing UTF-16 Strings requires 2 allocs and 1 encode
#[repr(C)]
pub struct OwnedString {
    chars: *mut u8,
    len: usize,
    capacity: usize,
    free: extern "C" fn(*mut u8, usize, usize),
}

extern "C" fn free_owned_string(chars: *mut u8, len: usize, capacity: usize) {
    let char_vec = unsafe { String::from_raw_parts(chars, len, capacity) };
    drop(char_vec);
}

impl From<String> for OwnedString {
    fn from(s: String) -> Self {
        let len = s.len();
        let capacity = s.capacity();
        let chars = s.into_bytes().leak().as_mut_ptr();
        OwnedString {
            chars,
            len,
            capacity,
            free: free_owned_string,
        }
    }
}

impl From<OwnedString> for String {
    fn from(value: OwnedString) -> Self {
        let s = ManuallyDrop::new(value);
        unsafe { String::from_raw_parts(s.chars, s.len, s.capacity) }
    }
}

impl Drop for OwnedString {
    fn drop(&mut self) {
        let self_str = unsafe { String::from_raw_parts(self.chars, self.len, self.capacity) };
        drop(self_str);
    }
}

/// UnownedUtf16String is a wrapper around a read-only pinned .NET managed string
/// This is to avoid allocating a new string everytime when passed from .NET when calling Marshal.StringToHGlobal*()
#[repr(C)]
pub struct UnownedUtf16String {
    chars: *const u16,
    len: i32,
    free: extern "C" fn(*const u16),
}

impl From<UnownedUtf16String> for String {
    fn from(value: UnownedUtf16String) -> Self {
        String::from_utf16_lossy(unsafe {
            std::slice::from_raw_parts(value.chars, value.len as usize)
        })
    }
}

impl Drop for UnownedUtf16String {
    fn drop(&mut self) {
        (self.free)(self.chars);
    }
}
