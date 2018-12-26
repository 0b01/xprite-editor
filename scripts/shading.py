WIDTH = 100
HEIGHT = 100

def run():
    from random import random, randint, choice, choices, sample, gauss
    from tqdm import tqdm
    PALETTE = [
        (210, 173, 140, 255),
        (206, 155, 122, 255),
        (220, 194, 122, 255),
    ]

    def get_color():
        return choices(PALETTE, [10, 10, 10], k=1)[0]

    def corrode(rect, corrosion_param, p0, p1, corner):
        """ corrode a corner of a rectangle """
        to_remove = set()
        if corner == 0:
            for (i, d) in enumerate(corrosion_param):
                for dy in range(d):
                    to_remove.add((p0[0]+i, p0[1]+dy))
        elif corner == 1:
            for (i, d) in enumerate(corrosion_param):
                for dy in range(d):
                    to_remove.add((p1[0]-i, p0[1]+dy))
        elif corner == 2:
            for (i, d) in enumerate(corrosion_param):
                for dy in range(d):
                    to_remove.add((p1[0]-i, p1[1]-dy))
        elif corner == 3:
            for (i, d) in enumerate(corrosion_param):
                for dy in range(d):
                    to_remove.add((p0[0]+i, p1[1]-dy))
        return [(a,b) for (a,b) in rect if a not in to_remove]


    def corrosion(cols, dist):
        """list of corrosion distance per column/row"""
        ret = []
        last = dist
        for _ in range(cols):
            d = randint(last//2, last)
            ret.append(d)
            last = d
        return ret

    ret = []
    def rectangle(p0, p1, color):
        ret = []
        for i in range(p0[0], p1[0]):
            for j in range(p0[1], p1[1]):
                ret.append(((i, j), color))
        return ret

    def underline(shape, n, color):
        from collections import defaultdict
        d = defaultdict(lambda: 0)
        for ((x,y),_) in shape:
            d[x] = max(d[x], y)
        ret = []
        for (x, y) in d.items():
            for i in range(n):
                ret.append(((x, y+i+1), color))
        return ret

    p0 = (10,10)
    p1 = (45, 30)
    rect = rectangle(p0, p1, PALETTE[2])
    corners = sample([0, 1, 2, 3], k=4)
    for corner in corners:
        cols = dist = 5
        c = corrosion(cols, dist)
        rect = corrode(rect, c, p0, p1, corner)
    shading = underline(rect, 2, PALETTE[1])
    ret += rect
    ret += shading

    return ret


PIXELS = run()
