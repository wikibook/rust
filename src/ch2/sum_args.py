import sys
total = 0
# 명령줄 인수를 순서대로 더한다
for i, v in enumerate(sys.argv):
    if i == 0: continue # 0번째는 명령어(프로그램) 자신이므로 무시 
    try:
        # 문자열을 숫자로 변환
        total += float(v)
    except ValueError:
        pass;
# 결과 표시
print(total)