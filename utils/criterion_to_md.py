# cargo install cargo-criterion
# ~/.cargo/bin/cargo criterion --message-format=json > test.json
# cat test.json | python3 criterion.py

import json
import sys

def format_time(t):
    return round(t / 1e6, 2)

print("""|benchmark|estimate (ms) |lower (ms)|upper (ms)|
|---------|--------|-----|-----|""")

for s in sys.stdin:
    j = json.loads(s)
    if "id" in j:
        typical = j["typical"]
        assert(typical["unit"] == "ns")
        print(f'|{j["id"]}|{format_time(typical["estimate"])}|{format_time(typical["lower_bound"])}|{format_time(typical["upper_bound"])}|')
