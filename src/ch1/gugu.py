# 파이썬으로 구구단 만들기
for y in range(1, 10):
    for x in range(1, 10):
        print("{:3},".format(y * x), end="")
    print("")

