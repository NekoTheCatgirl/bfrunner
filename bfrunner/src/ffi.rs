use super::{ parse, exec, Tape };
use std::io::{ Cursor, Write };
use std::os::raw::{ c_char, c_int };
use std::slice;

#[unsafe(no_mangle)]
pub extern "C" fn brainfuck_run(
    source: *const c_char,
    input: *const c_char,
    output: *mut c_char,
    output_capacity: usize
) -> c_int {
    let source = unsafe {
        if source.is_null() {
            return -1;
        }
        std::ffi::CStr::from_ptr(source).to_str().unwrap_or("")
    };

    let input_data = unsafe {
        if input.is_null() { "" } else { std::ffi::CStr::from_ptr(input).to_str().unwrap_or("") }
    };

    let mut state = Tape::default();
    let mut input_cursor = Cursor::new(input_data.as_bytes());

    let mut output_buf = unsafe {
        if output.is_null() {
            return -2;
        }
        slice::from_raw_parts_mut(output as *mut u8, output_capacity)
    };

    struct SliceWriter<'a> {
        buf: &'a mut [u8],
        pos: usize,
    }

    impl<'a> Write for SliceWriter<'a> {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            let remaining = self.buf.len().saturating_sub(self.pos);
            let to_write = remaining.min(buf.len());
            self.buf[self.pos..self.pos + to_write].copy_from_slice(&buf[..to_write]);
            self.pos += to_write;
            Ok(to_write)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    let mut writer = SliceWriter { buf: &mut output_buf, pos: 0 };

    let ast = match parse(source) {
        Ok(a) => a,
        Err(_) => {
            return -3;
        }
    };

    if let Err(_) = exec(&ast, &mut state, &mut input_cursor, &mut writer) {
        return -4;
    }

    writer.pos as c_int
}
