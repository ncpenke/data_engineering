from os import mkdir
import json
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

def write_file(i, n):
    csv_file = open(f"./data/rows_{i}_{n}.csv", "w")
    json_file = open(f"./data/rows_{i}_{n}.json", "w")
    
    csv_file.write(",".join(columns))
    csv_file.write("\n")

    json_file.write("[\n")

    while n > 0:
        d = {}
        d[columns[0]] = gen_date()
        d[columns[1]] = gen_string()
        d[columns[2]] = gen_number()
        d[columns[3]] = gen_string()
        d[columns[4]] = gen_number()
        n -= 1
        csv_file.write(",".join(d.values()))
        csv_file.write("\n")
        json_file.write(json.dumps(d))
        if n > 0:
            json_file.write("\n,")
    
    json_file.write("]\n")
        

columns =  [ f"column{i}" for i in range(0,5) ]

try:
    os.system("rm -rf data")
    os.mkdir("./data")
except:
    pass

for i in range(100):
    write_file(i, 1000)
for i in range(200):
    write_file(i, 1e3)
write_file(0, 1e5)
write_file(0, 2e6)