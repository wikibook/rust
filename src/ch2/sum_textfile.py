import sys
total = 0
# 명령줄에 지정한 파일을 처리
for i, v in enumerate(sys.argv):
    if i == 0: continue
    # 텍스트 파일을 읽어들임
    with open(v, "rt") as fp:
        text = fp.read()
        # 한 줄씩 읽어들여 계산
        for line in text.split("\n"):
            try:
                total += float(line)
            except ValueError:
                pass;
# 결과 표시
print(total)