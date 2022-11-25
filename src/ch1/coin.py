# 파이썬으로 거스름돈 조합 계산
# 거스름돈
price = 3950
# 500원 동전의 수 만큼 반복
for i500 in range(0, 11):
    # 100원 동전의 수 만큼 반복
    for i100 in range(0, 4):
        # 50원 동전의 수 만큼 반복
        for i50 in range(0, 11):
            # 동전의 합계를 계산
            total = i50 * 50 + i100 * 100 + i500 * 500
            # 동전의 합계가 거스름돈과 동일한 경우 출력
            if price == total:
                print("500원x{}+100원x{}+원x{}={}"
                        .format(i500,i100,i50,total))
