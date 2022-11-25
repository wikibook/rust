import random
# 1에서 75까지의 수로 이루어진 리스트를 만든다
nums = list(range(1, 75+1))
# 섞기
random.shuffle(nums)
nums[12] = "*" # 와일드카드 지정
# 카드 표시
for y in range(0, 5):
    for x in range(0, 5):
        print("{:>3},".format(nums[y*5+x]), end="")
    print("")

