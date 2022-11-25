use std::ffi::CString;
fn main() {
    // Rust 문자열 준비
    let msg = "안녕하세요";
    // C 언어 문자열로 변경
    let msg_cstr = CString::new(msg).unwrap();
    // C 라이브러리 호출
    unsafe {
        // 여기서 C 라이브러리 호출
        // c_lang_lib::print_str(msg_cstr.as_ptr());
    }
}