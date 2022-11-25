import sys
dicfile = "dict.txt"

# 인수 확인
if len(sys.argv) < 2:
    print("[USAGE] dictionary.py word")
    quit()
# 인수로 넘어온 단어
word = sys.argv[1]

# 사전 데이터를 한 줄씩 비교해 일치하는 것이 있으면 출력
with open(dicfile, "rt", encoding="utf-8") as fp:
    while True:
        line = fp.readline()
        if not line: break
        if word in line:
            print(line.strip())