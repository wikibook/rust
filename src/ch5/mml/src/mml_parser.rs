use crate::wav_writer::Note; // --- (*1)

// MML을 해석해 Vec<Note> 타입으로 변환 --- (*2)
pub fn parse(src: String) -> Vec<Note> {
    // 해석 결과를 담을 변수 준비
    let mut result = vec![];
    // 옥타브와 음 길이 초기값 지정 --- (*3)
    let mut octave = 5;
    let mut length = 4;
    // 문자열에서 한 문자씩 읽어들임 --- (*4)
    let mut it = src.chars();
    while let Some(ch) = it.next() {
        // MML 코드에서 음, 길이 등을 분기 처리 --- (*5)
        match ch {
            'a'..='g' => { // 노트 --- (*6)
                let note = match ch {
                    'c' => 0, 'd' => 2, 'e' => 4, 'f' => 5, 
                    'g' => 7, 'a' => 9, 'b' => 11, _ => 0,
                };
                let no = note + octave * 12;
                result.push(Note{no, len: length});
            },
            // 쉼표
            'r' => result.push(Note{no: -1, len: length}),
            'o' => { // 옥타브 --- (*7)
                let v = it.next().expect("o 뒤에 숫자를 지정");
                let o = v as i32 - '0' as i32;
                if o >= 0 && o < 9 { octave = o; }
            },
            'l' => { // 음 길이 --- (*8)
                let v = it.next().expect("l 뒤에 숫자를 지정");
                let l = v as i32 - '0' as i32;
                if l >= 1 && l <= 9 { length = l; }
            },
            _ => {}, // 해당하지 않는 문자는 아무 것도 하지 않음
        };
    }
    result
}

// 테스트 --- (*9)
#[cfg(test)]
mod mml_parser_test {
    use super::*; // 상위 요소 이용 선언
    #[test]
    fn parse_test() {
        let mml = "l2 o5 cde".to_string();
        let notes = parse(mml);
        assert_eq!(notes[0].no, 60);
        assert_eq!(notes[0].len, 2);
        assert_eq!(notes[1].no, 62);
        assert_eq!(notes[2].no, 64);
    }
}
