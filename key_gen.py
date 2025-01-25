import random
import string
import sys

if len(sys.argv) <= 1:
  print("please pass a command line argument represents key length")
  exit

n = int(sys.argv[1])
if n < 0 or n > 100:
  print("command line argument should be from 0 to 100")
  exit

def generate_random_n_bit_ascii():
    # Create a pool of ASCII characters (printable characters)
    ascii_chars = string.ascii_letters + string.digits + string.punctuation
    # Generate a 32-character long random string
    return ''.join(random.choice(ascii_chars) for _ in range(n))

random_sequence = generate_random_n_bit_ascii()
print(f"Random {n}-bit ASCII sequence:", random_sequence)
