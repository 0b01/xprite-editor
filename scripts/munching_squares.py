WIDTH = 512
HEIGHT = 512

def munching_squares():
    ret = []
    for x in range(512):
        for y in range(512):
            c = x^y
            ret.append(((x, y), (c,0,0,255)))
    return ret

PIXELS = munching_squares()
