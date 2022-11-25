use std::f32::consts::PI;
use std::io::{Write, Seek};
use hound::WavWriter;
// 상수
const SAMPLE_RATE: f32 = 44100.0;
const BPM: f32 = 122.0; // BPM(템포)

#[allow(unused_variables)]
fn main() {
    // WavWriter 객체 생성  --- (*1)
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut fw = WavWriter::create("melody.wav", spec).unwrap();
    // 음정 정의 --- (*2)
    // 도, 레, 미, 파
    let (c4, d4, e4, f4) = (261.626, 293.665, 329.628, 349.228);
    // 솔, 라, 시, 도(다음 옥타브)
    let (g4, a4, b4, c5) = (391.995, 440.000, 493.883, 523.251);
    // 음 길이 정의 --- (*3)
    let l4 = ((60.0 / BPM) * SAMPLE_RATE) as u32;
    let l2 = l4 * 2;
    // 멜로디 지정 --- (*4)
    write_tone(&mut fw, g4, l4);
    write_tone(&mut fw, g4, l4);
    write_tone(&mut fw, a4, l4);
    write_tone(&mut fw, a4, l4);
    write_tone(&mut fw, g4, l4);
    write_tone(&mut fw, g4, l4);
    write_tone(&mut fw, e4, l2);
    write_tone(&mut fw, g4, l4);
    write_tone(&mut fw, g4, l4);
    write_tone(&mut fw, e4, l4);
    write_tone(&mut fw, e4, l4);
    write_tone(&mut fw, d4, l2);
}

// 지정한 음계를 지정한 길이만큼 쓴다 --- (*5)
fn write_tone<W>(fw: &mut WavWriter<W>, tone: f32, len: u32) 
where W: Write + Seek {
    for t in 0..len {
        let a = t as f32 / SAMPLE_RATE;
        let v = (a * tone * 2.0  * PI).sin();
        fw.write_sample((v * i16::MAX as f32) as i16).unwrap();
    }
}
