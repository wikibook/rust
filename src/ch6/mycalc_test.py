# Python에서 동적 라이브러리 이용
import platform, os
from ctypes import *

# OS 확인 --- (*1)
pf = platform.system()
print(pf)

# Windows --- (*2)
if pf == 'Windows': libfile = 'mycalc.dll'
# macOS
elif pf == 'Darwin': libfile = 'libmycalc.dylib'
# Linux
else: libfile = 'libmycalc.so'

# 동적 라이브러리 경로 지정 --- (*3)
libpath = os.path.join(os.path.dirname(__file__), libfile)
print("lib=", libpath)

# 라이브러리 로드 --- (*4)
mycalc = cdll.LoadLibrary(libpath)
# Rust 라이브러리 실행 --- (*5)
print(mycalc.rust_mul(100, 8))
print(mycalc.rust_mul(8, 9))