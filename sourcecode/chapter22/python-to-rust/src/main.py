from ctypes import cdll
from sys import platform

if platform == 'darwin':
    prefix = 'lib'
    ext = 'dylib'
elif platform == 'win32':
    prefix = ''
    ext = 'dll'
else:
    prefix = 'lib'
    ext = 'so'

lib = cdll.LoadLibrary('target/debug/{}triple_input.{}'.format(prefix, ext))
triple_input = lib.triple_input

input = 5
output = triple_input(input)
print('{} * 3 = {}'.format(input, output))


