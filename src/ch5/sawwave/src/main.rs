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
        "saw.wav", spec).unwrap();
    // 톱니파로 음을 생성 --- (*2)
    let mut wav: Vec<f32> = vec![];
    let bpm = 120;
    wav.extend(sawtooth_wave(60, calc_len(bpm, 4), 0.5));
    wav.extend(sawtooth_wave(64, calc_len(bpm, 4), 0.5));
    wav.extend(sawtooth_wave(67, calc_len(bpm, 4), 0.5));
    // 파일로 쓰기 --- (*3)
    for v in wav.into_iter() {
        fw.write_sample(v).unwrap();
        println!("{}", v);
    }
}
// 노트 번호를 주파수로 변경 --- (*4)
fn noteno_to_hz(no: i32) -> f32 {
     440.0 * 2.0f32.powf((no-69) as f32 / 12.0)
}
// n 음표 샘플 수를 계산 --- (*5)
fn calc_len(bpm: usize, n: usize) -> usize {
    let base_len = (60.0 / bpm as f32) * SAMPLE_RATE;
    ((4.0 / n as f32) * base_len) as usize
}
// 톱니파 생성 --- (*6)
fn sawtooth_wave(noteno: i32, len: usize, gain: f32) -> Vec<f32> {
    let tone = noteno_to_hz(noteno); // 주파수 얻기
    let form_samples = SAMPLE_RATE / tone; // 주기 얻기
    let mut wav:Vec<f32> = vec![0.0; len];
    for i in 0..len {
        let pif = (i as f32 / form_samples) % 1.0;
        wav[i] = pif * 2.0 - 1.0;
    }
    // 음량 조절 --- (*7)
    wav.into_iter().map(|v| (v * gain) as f32).collect()
}
