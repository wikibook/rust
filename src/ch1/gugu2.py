# 파이썬으로 구구단 만들기(맨 끝의 쉼표 삭제)
for y in range(1, 10):
    a = ["{:3}".format(x * y) for x in range(1, 10)]
    print(",".join(a))


