# 인기 투표 집계
# 투표 데이터 --- (*1)
V_DATA = "C,C,A,A,A,B,C,C,B,B,B,C,B,C,B,A,C,C,B,C,C,C"
# 집계용 사전 타입 데이터 초기화 --- (*2)
c_dic = {"A": 0, "B": 0, "C": 0}
# 투표 데이터 카운트 --- (*3)
for w in V_DATA.split(","):
    c_dic[w] += 1
# 집계 후 결과 표시 --- (*4)
for key in ["A", "B", "C"]:
    print("{}: {:2d}".format(key, c_dic[key]))