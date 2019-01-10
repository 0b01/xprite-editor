from pprint import pprint
import requests as r

url = "https://lospec.com/palette-list/load?colorNumberFilterType=any&colorNumber=8&page={}&tag=&sortingType=default"
for i in range(27):
    u = url.format(i)
    print(u)
    pals = r.get(u).json()
    for pal in pals:
        name = pal['name']
        if "Place" in name:
            continue
        colors = pal['colors']
        with open(name+".hex", 'w') as f:
            f.write("\n".join(colors.split(",")))