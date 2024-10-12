from ctypes import CDLL

so_file = "./multiply.so"
multiply = CDLL(so_file)

# print(type(multiply))

a = 25
b = 30
c = 35
d = 54
e = 654

print(multiply.multiply(a, b, c, d, e))
