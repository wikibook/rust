use std::f32::consts::PI;
use hound::WavWriter;
use std::io::{Write, Seek};

const SAMPLE_RATE: f32 = 44100.0;

// 노트 번호(음계)와 길이를 정의한 구조체 --- (*1)
#[derive(Debug)]
pub struct Note {
    pub no: i32,
    pub len: i32,
}

// Vec<Note>를 WAV 파일로 저장하는 함수 --- (*2)
pub fn write(filename: &str, notes: Vec<Note>, bpm: f32) {
    // WAV 파일 포맷을 지정
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut fw = WavWriter::create(filename, spec).unwrap();
    // 반복문을 이용해 음을 생성 --- (*3)
    for note in notes.into_iter() {
        // 음의 길이 계산
        let len = (4.0 / note.len as f32 * 
                   (60.0 / bpm) * SAMPLE_RATE) as u32;
        // 주파수 계산 --- (*4)
        let tone = if note.no >= 0 {
            440.0 * 2.0f32.powf((note.no - 69) as f32 / 12.0)
        } else { 0.0 };
        write_tone(&mut fw, tone, len);
    }
}

// 사인 파형을 파일로 저장 --- (*5)
fn write_tone<W>(fw: &mut WavWriter<W>, tone: f32, len: u32) 
where W: Write + Seek {
    for t in 0..len {
        let a = t as f32 / SAMPLE_RATE;
        let v = (a * tone * 2.0  * PI).sin();
        fw.write_sample((v * i16::MAX as f32) as i16).unwrap();
    }
}

// 라이브러리 테스트 --- (*6)
#[cfg(test)]
mod wav_writer_test {
    use super::*; // 상위 요소 이용 선언
    #[test]
    fn notes_test() {
        // 노트 목록 생성
        let notes: Vec<Note> = vec![
            Note{ no: 60, len: 4},
            Note{ no: 62, len: 4},
            Note{ no: 64, len: 4},
        ];
        // WAV 파일로 저장
        write("test.wav", notes, 120.0);
    }
}
