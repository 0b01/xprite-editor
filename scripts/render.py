WIDTH = 1000
HEIGHT = 1000

import numpy as np

RED = (255, 0, 0, 255)

def cubicbezier(x0, y0, x1, y1, x2, y2, x3, y3, n=20):
    global RED
    pts = []
    for i in range(n+1):
        t = i / n
        a = (1. - t)**3
        b = 3. * t * (1. - t)**2
        c = 3.0 * t**2 * (1.0 - t)
        d = t**3

        x = int(a * x0 + b * x1 + c * x2 + d * x3)
        y = int(a * y0 + b * y1 + c * y2 + d * y3)
        pts.append( ((x, y), RED) )
    # for i in range(n):
    #     pts += line(pts[i][0], pts[i][1], pts[i+1][0], pts[i+1][1])
    return pts


def test():
    global RED
    pixels = []
    for i in range(100):
        for j in range(100):
            pixels.append(
                ((i,j), RED)
            )
    return pixels

pixels = test()