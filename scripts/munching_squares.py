WIDTH = 256
HEIGHT = 256

def munching_squares():
    ret = xpr.Pixels()
    for x in range(256):
        for y in range(256):
            c = x^y
            ret.push(xpr.Pixel((x, y), (c,0,0,255)))
    return ret

PIXELS = munching_squares()
