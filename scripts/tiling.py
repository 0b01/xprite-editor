import builtins
builtins.xpr = xpr
import sys
sys.path.append(".")

WIDTH = 512
HEIGHT = 512

def run():
    from scripts.common import PALETTE, sample, corrosion, corrode, underline, get_color
    from tqdm import tqdm

    ret = xpr.Pixels()
    side = 9
    n = 512 // side
    background = xpr.rect((0,0), (512,512), True).with_color(PALETTE[0])
    for i in tqdm(range(n)):
        for j in range(n):
            p0 = (side * i, side * j)
            rect = xpr.rect((0,0), (side, side), True).with_color(PALETTE[2])

            corners = sample([0, 1, 2, 3], k=4)
            for corner in corners:
                cols = dist = 5
                rect = corrode(
                    rect,
                    corrosion(cols, dist),
                    (0,0),
                    (side,side),
                    corner
                )
            ret @= rect.shift(p0)
    underlined = underline(ret, 512, 512, 2, PALETTE[1])
    return background @ ret @ underlined

PIXELS = run()
