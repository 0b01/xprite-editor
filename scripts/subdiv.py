WIDTH = 1024
HEIGHT = 1024

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

    def rectangle(p0, p1):
        ret = []
        color = get_color()
        for i in range(p0[0], p1[0]):
            for j in range(p0[1], p1[1]):
                ret.append(((i, j), color))
        return ret

    cutoff = 2
    padding = 2
    ret = []

    def draw_block(v0, v1):
        ((x0, y0), (x1, y1)) = (v0, v1)
        if x1 - x0 < cutoff or y1 - y0 < cutoff:
            draw_rect(v0, v1)
            return
        if random() > 0.5:
            draw_rect(v0, v1);
            draw_block((x0 + padding, y0 + padding), (x1 - padding, y1 - padding))
        else:
            split_block(v0, v1);

    def split_block(v0, v1):
        ((x0, y0), (x1, y1)) = (v0, v1)
        cut_dir = get_dir(v0, v1)
        if cut_dir == 'H':
            pivot = get_pos(y0, y1)
            draw_block(v0, (x1, pivot))
            draw_block((x0, pivot), v1)
        else:
            pivot = get_pos(x0, x1)
            draw_block(v0, (pivot, y1))
            draw_block((pivot, y0), v1)

    def get_dir(v0, v1):
        ((x0, y0), (x1, y1)) = (v0, v1)
        if x1 - x0 < y1 - y0:
            return 'H'
        else:
            return 'V'

    def get_pos(p1, p2):
        return int(gauss((p1 + p2) / 2, (p2 - p1) / 8))

    def draw_rect(v0, v1):
        nonlocal ret
        ret += rectangle(v0, v1)

    pad = 3
    side = 64
    n = 1024 // side
    for i in range(n):
        for j in range(n):
            p0 = (i * side+pad, j*side+pad)
            p1 = ((i+1) * side-pad, (j+1)*side-pad)
            print (p0,p1)
            draw_block(p0, p1)
    return ret


PIXELS = run()
