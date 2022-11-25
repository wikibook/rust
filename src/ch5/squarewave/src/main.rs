use hound;
const SAMPLE_RATE: f32 = 44100.0;
fn main() {
    // WavWriter 생성 --- (*1)
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut fw = hound::WavWriter::create(
        "sq.wav", spec).unwrap();
    // 방형파로 음 생성 --- (*2)
    let mut wav: Vec<f32> = vec![];
    let bpm = 120;
    // 멜로디 생성 --- (*3)
    [60,64,67,64, 60,64,67,72].iter().for_each(|no| {
        wav.extend(square_wave(*no, calc_len(bpm, 8), 0.5));
    });
    // 파일로 쓰기
    for v in wav.into_iter() {
        fw.write_sample(v).unwrap();
        println!("{}", v);
    }
}
fn noteno_to_hz(no: i32) -> f32 {
     440.0 * 2.0f32.powf((no-69) as f32 / 12.0)
}
fn calc_len(bpm: usize, n: usize) -> usize {
    let base_len = (60.0 / bpm as f32) * SAMPLE_RATE;
    ((4.0 / n as f32) * base_len) as usize
}
// 방형파 생성 함수 --- (*4)
fn square_wave(noteno: i32, len: usize, gain: f32) -> Vec<f32> {
    let tone = noteno_to_hz(noteno); // 주파수
    let form_samples = SAMPLE_RATE / tone; // 주기
    let mut wav:Vec<f32> = vec![0.0; len];
    // 방형파 주기 반올림
    let half_fs = (form_samples / 2.0) as usize;
    for i in 0..len {
        let hl = (i / half_fs) % 2;
        wav[i] = if hl == 0 { -1.0 } else { 1.0 };
    }
    // 음량 조절
    wav.into_iter().map(|v| (v * gain) as f32).collect()
}
