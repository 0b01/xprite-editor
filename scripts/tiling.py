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
    side = 24
    n = 512 // side
    background = xpr.rect((0,0), (512,512), True).with_color(PALETTE[0])
    for i in tqdm(range(1, n-1)):
        for j in range(1, n-1):
            p0 = (side * i, side * j)
            rect = xpr.rect((0,0), (side, side), True).with_color(get_color())

            corners = sample([0, 1, 2, 3], k=4)
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
    perimeter = ret.perimeter(512, 512).pixel_perfect().with_color(PALETTE[3])
    underlined = underline(ret, 512, 512, side//4 - 1, PALETTE[1])
    return background @ underlined @ ret @ perimeter

PIXELS = run()
