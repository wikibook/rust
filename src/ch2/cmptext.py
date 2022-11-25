# 파일 이름 지정 --- (*1)
afile = "./fizzbuzz_python.txt"
bfile = "./fizzbuzz_rust.txt"

# 파일 내용을 읽어들임 --- (*2)
with open(afile, "r") as fp:
    astr = fp.read()
with open(bfile, "r") as fp:
    bstr = fp.read()

# 만약을 위해 불필요한 공백 삭제(trim)
astr = astr.strip()
bstr = bstr.strip()

# 비교 --- (*3)
if astr == bstr:
    print("ok")
else:
    print("ng")