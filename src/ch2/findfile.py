import sys, os

# 명령줄 인수 확인
if len(sys.argv) < 3:
    print("findfile.py (path) (keyword)")
    quit()
# 명령줄 인수 얻기
target_dir = sys.argv[1]
keyword = sys.argv[2]

# 지정한 디렉토리 검색
for dirname, dirs, files in os.walk(target_dir):
    for file in files:
        if keyword in file:
            fullpath = os.path.join(dirname, file)
            print(fullpath)