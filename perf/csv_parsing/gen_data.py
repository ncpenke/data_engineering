from os import mkdir
import string
import random
import os

def gen_string():
    letters = string.ascii_uppercase
    return ''.join(random.choice(letters) for i in range(10))

def gen_number():
    numbers = string.digits
    i = ''.join(random.choice(numbers) for i in range(5))
    fraction = ''.join(random.choice(numbers) for i in range(2))
    return f"{i}.{fraction}"

def gen_date():
    y = random.choice(range(1900, 2000))
    m = random.choice(range(1, 12))
    d = random.choice(range(1, 31))
    return f"{y}-{m}-{d}"

def write_rows(n):
    with open(f"./data/rows_{n}.csv", "w") as f:
        f.write(",".join(columns))
        f.write("\n")
        while n > 0:
            n -= 1
            f.write(f"{gen_date()},{gen_string()},{gen_number()},{gen_string()},{gen_number()}\n")

columns =  [ f"column{i}" for i in range(0,5) ]

try:
    os.system("rm -rf data")
    os.mkdir("./data")
except:
    pass

write_rows(1000)
write_rows(1e3)
write_rows(1e5)
write_rows(2e6)