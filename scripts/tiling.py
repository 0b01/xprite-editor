WIDTH = 512
HEIGHT = 512

def run():
    from random import random, randint, choice, choices, sample
    from tqdm import tqdm
    PALETTE = [
        (210, 173, 140, 255),
        (206, 155, 122, 255),
        (220, 194, 122, 255),
    ]

    def corrosion(cols, dist):
        """list of corrosion distance per column/row"""
        ret = []
        last = dist
        for _ in range(cols):
            d = randint(last//2, last)
            ret.append(d)
            last = d
        return ret

    def get_color():
        return choices(PALETTE, [30, 1, 1], k=1)[0]

    def rectangle(p0, p1, color):
        ret = []
        for i in range(p0[0], p1[0]):
            for j in range(p0[1], p1[1]):
                ret.append(((i, j), color))
        return ret

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

    ret = []
    side = 9
    n = 512 // side
    ret += rectangle((0,0), (512,512), PALETTE[0])
    for i in tqdm(range(n)):
        for j in range(n):
            p0 = (side * i, side * j)
            p1 = (side * (i+1), side * (j+1))
            rect = rectangle(p0, p1, get_color())

            corners = sample([0, 1, 2, 3], k=4)
            for corner in corners:
                cols = dist = 5
                c = corrosion(cols, dist)
                rect = corrode(rect, c, p0, p1, corner)
            ret += rect
    return ret

PIXELS = run()
