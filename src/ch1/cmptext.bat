rem === Windows용 ===
rem  fizzbuzz 파일 결과를 저장
python3 fizzbuzz.py > fb_python.txt
rustc fizzbuzz.rs && ./fizzbuzz > fb_rust.txt
rem diff로 파일 내용 차이를 확인
fc fb_python.txt fb_rust.txt
