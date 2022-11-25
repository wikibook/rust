# 소수인지 판단하는 함수 --- (*1)
def is_prime(n):
    for i in range(2, n):
        if n % i == 0:
            return False
    return True

# count만큼 소수를 생성 --- (*2)
def get_primes(count):
    res = []
    i = 2
    while len(res) < count:
        if is_prime(i):
            res.append(i)
        i += 1
    return res

# 생성한 소수를 출력
print(get_primes(100))

