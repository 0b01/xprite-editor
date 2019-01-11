import builtins
builtins.xpr = xpr
import sys
sys.path.append(".")

WIDTH = 512
HEIGHT = 512

def run():
    from scripts.common import PALETTE, sample, corrosion, corrode, underline, get_color
    import random
    from tqdm import tqdm

    def gen_blob(side):
        rect = xpr.rect((0,0), (side, side), True).with_color(PALETTE["dust2"])
        corners = sample([0, 1, 2, 3], k=4)
        for corner in corners:
            cols = dist = side
            rect = corrode(
                rect,
                corrosion(cols, dist),
                (0,0),
                (side,side),
                corner
            )
        return rect

    ret = xpr.Pixels()
    side = 24
    n = 512 // side
    background = xpr.rect((0,0), (512,512), True).with_color(PALETTE["bg"])

    # for i in tqdm(range(1, n-1)):
    #     for j in range(1, n-1):
    #         p0 = (side * i, side * j)
    #         ret @= gen_blob(side).shift(p0)

    for _ in range(250):
        ret @= gen_blob(random.randint(15, 30)).shift((random.randint(-30, 500), random.randint(-30, 500)))
    ccs = ret.connected_components(512, 512)
    print(len(ccs))
    underlined = xpr.Pixels()
    perimeter = xpr.Pixels()
    ret_pixs = xpr.Pixels()
    for cc in ccs:
        ret_pixs @= cc.with_color(get_color())
        perimeter @= cc.perimeter(512, 512).with_color(PALETTE["outline"])
        underlined @= underline(cc, 512, 512, 3, PALETTE["outline"])
    return background @ underlined @ ret_pixs @ perimeter

PIXELS = run()
