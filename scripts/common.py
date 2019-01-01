from random import random, randint, choice, choices, sample, gauss

PALETTE = [
    (210, 173, 140, 255),
    (206, 155, 122, 255),
    (220, 194, 122, 255),
]

def get_color():
    return choices(PALETTE, [10, 10, 10], k=1)[0]

def corrode(rect, corrosion_param, p0, p1, corner):
    """ corrode a corner of a rectangle """
    to_remove = xpr.Pixels()
    if corner == 0:
        for (i, d) in enumerate(corrosion_param):
            for dy in range(d):
                to_remove.push(xpr.Pixel((p0[0]+i, p0[1]+dy), xpr.RED))
    elif corner == 1:
        for (i, d) in enumerate(corrosion_param):
            for dy in range(d):
                to_remove.push(xpr.Pixel((p1[0]-i, p0[1]+dy), xpr.RED))
    elif corner == 2:
        for (i, d) in enumerate(corrosion_param):
            for dy in range(d):
                to_remove.push(xpr.Pixel((p1[0]-i, p1[1]-dy), xpr.RED))
    elif corner == 3:
        for (i, d) in enumerate(corrosion_param):
            for dy in range(d):
                to_remove.push(xpr.Pixel((p0[0]+i, p1[1]-dy), xpr.RED))
    rect.sub_(to_remove)
    return rect


def corrosion(cols, dist):
    """list of corrosion distance per column/row"""
    ret = []
    last = dist
    for _ in range(cols):
        d = randint(last//2, last)
        ret.append(d)
        last = d
    return ret

def underline(pixs, w, h, shading_len, color):
    ret = xpr.Pixels()
    mat = pixs.as_bool_mat(w, h)
    for i in range(h-1):
        for j in range(w-1):
            if mat[i][j] and not mat[i][j+1]:
                for k in range(shading_len):
                    ret.push(xpr.Pixel((i, j+k), xpr.RED))
    return ret.with_color(color)