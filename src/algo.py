import os
import re

results = []
while dirs:
    dir = dirs.pop()
    for entry in os.scandir(dir):
        if entry.is_dir():
            dirs.append(path)
        elif entry.is_file():
            for pattern in patterns:
                if re.match(pattern, entry.name):
                    results.append(pattern)

for result in results:
    print(result.name)