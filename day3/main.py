import regex as re

# mul(5,5)
# search for all these patterns using regex

with open('input.txt', 'r') as f:
    data = f.read()

#print(data)

# Do a regex search for all the patterns
def search_for_patterns(data):
    pattern = r'mul\((\d+),(\d+)\)'
    matches = re.findall(pattern, data)
    return matches

nums = (search_for_patterns(data))

s = 0
for i, j in nums:
    s += int(i) * int(j)

print(s)