# 파이썬으로 FizzBuzz 문제 풀기
# 1에서 100까지 반복 --- (*1)
for i in range(1, 101):
    # 조건과 일치하는지 확인 --- (*2)
    if i % 3 == 0 and i % 5 == 0:
        print("FizzBuzz")
    elif i % 3 == 0:
        print("Fizz")
    elif i % 5 == 0:
        print("Buzz")
    else:
        print(i)
