# 암호화 함수
def encrypt(text, shift):
    # 'A'와'Z'의 문자 코드를 취득 --- (*1)
    code_a = ord('A')
    code_z = ord('Z')
    # 결과를 대입할 변수를 준비
    result = ""
    # 한 문자씩 반복 --- (*2)
    for ch in text:
        # 문자 코드로 변환
        code = ord(ch)
        # A~Z 사이인가 --- (*3)
        if code_a <= code <= code_z:
            # shift만큼 뒤의 문자로 변경한다 --- (*4)
            code = (code - code_a + shift) % 26 + code_a
        # 문자 코드를 문자로 변환 --- (*5)
        result += chr(code)
    return result

# 함수 호출
enc = encrypt("I LOVE RUST.", 3)
dec = encrypt(enc, -3)
print(enc, "=>", dec)

