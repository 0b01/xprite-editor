import builtins
builtins.xpr = xpr
import sys
sys.path.append(".")


def run():
    from scripts.common import PALETTE, sample, corrosion, corrode, underline, get_color, gen_maze, to_corners
    from tqdm import tqdm
    WIDTH = HEIGHT = 1024

    ret = xpr.Pixels()
    side = 5
    n = WIDTH
    maze = gen_maze(WIDTH//side-1)

    background = xpr.rect((0,0), (WIDTH, HEIGHT), True).with_color(PALETTE[0])
    for i, row in tqdm(enumerate(maze)):
        for j, (needs_draw, corner_bits) in enumerate(row):
            if not needs_draw:
                continue
            p0 = (side * (i+1), side * (j+1))
            rect = xpr.rect((0,0), (side, side), True).with_color(PALETTE[2])
            corners = to_corners(corner_bits)
            for corner in corners:
                cols = dist = side // 2
                rect = corrode(
                    rect,
                    corrosion(cols, dist),
                    (0,0),
                    (side,side),
                    corner
                )
            ret @= rect.shift(p0)
    underlined = underline(ret, WIDTH, HEIGHT, side//2 + 2, PALETTE[1])
    return WIDTH, HEIGHT, background @ underlined @ ret

WIDTH, HEIGHT, PIXELS = run()
