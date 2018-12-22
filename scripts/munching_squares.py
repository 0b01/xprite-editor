WIDTH = 256
HEIGHT = 256

def munching_sq():
    ret = []
    for x in range(256):
        for y in range(256):
            c = x^y
            ret.append(((x, y), (c,c,0, 255)))
    return ret

pixels = munching_sq()
