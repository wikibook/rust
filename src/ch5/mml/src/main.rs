mod mml_parser;
mod wav_writer;

fn main() {
    // 산토끼 --- (*1)
    let mml = format!("{}{}",
            "o4 l4 g l8 ee g e l4 cd l8 edce l4 gs",
            "o5 l8 c o4 l8 g o5 l8 c o4 g o5 c o4 g l4 e g l8 d fed l4 c");
    // 한 음씩 Vec<Note>에 추가 --- (*2)
    let notes = mml_parser::parse(mml);
    // WAV 파일로 저장 --- (*3)
    let bpm = 120.0;
    wav_writer::write("rabbit.wav", notes, bpm);

    // 작은별 --- (*4)
    let mml = format!("{}{}{}",
        "o5 l4 ccgg aal2g l4 ffee ddl2c",
        "l4 ggff eel2d l4 ggff eel2d",
        "l4 ccgg aal2g l4 ffee ddl2c"
    );
    let notes = mml_parser::parse(mml);
    let bpm = 120.0;
    wav_writer::write("twinkle_star.wav", notes, bpm);
}
