use hound;
use rand::prelude::*;
const SAMPLE_RATE: f32 = 44100.0;
fn main() {
    // WavWriter 생성
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE as u32,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut fw = hound::WavWriter::create(
        "noise.wav", spec).unwrap();
    // 노이즈 생성
    let mut wav: Vec<f32> = vec![];
    let bpm = 120;
    // 생성 --- (*1)
    // 풀 레인지(rull range) 노이즈 생성
    wav.extend(noise(2.0, -1.0, calc_len(bpm, 2)));
    // 0.8에서 1.0 사이의 노이즈 생성
    wav.extend(noise(0.2, 0.8, calc_len(bpm, 2)));
    // -1.0에서 -0.2 사이의 노이즈 생성
    wav.extend(noise(0.8, -1.0, calc_len(bpm, 2)));
    // 파일로 저장
    for v in wav.into_iter() {
        fw.write_sample(v).unwrap();
        println!("{}", v);
    }
}
fn calc_len(bpm: usize, n: usize) -> usize {
    ((4.0 / n as f32) * 
     (60.0 / bpm as f32) * SAMPLE_RATE) as usize
}
// 노이즈 생성 함수 --- (*2)
fn noise(range: f32, shift: f32, len: usize) -> Vec<f32> {
    let mut wav:Vec<f32> = vec![0.0; len];
    let mut rng = rand::thread_rng();
    for i in 0..len {
        wav[i] = rng.gen::<f32>() * range + shift;
    }
    // 음량 조절
    let gain = 0.5;
    wav.into_iter().map(|v| (v * gain) as f32).collect()
}
