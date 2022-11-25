# Counter 클래스 정의 --- (*1)
class Counter:
    value = 0
    # 값을 1 증가시키는 메서드
    def inc(self):
        self.value += 1
        print("value=", self.value)

# Counter 클래스를 인수로 하는 함수 --- (*2)
def count(counter):
    counter.inc()

# 올바르게 이용되는 경우 --- (*3)
a = Counter()
count(a)
count(a)

# 문제가 발생하는 경우 --- (*4)
a = None
count(a)
