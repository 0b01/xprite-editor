import builtins
builtins.xpr = xpr
import sys
sys.path.append(".")

WIDTH = 100
HEIGHT = 100
def run():
    from scripts.common import PALETTE, sample, corrosion, corrode, underline

    p0 = (10,10)
    p1 = (45, 30)

    background = xpr.rect((0,0), (512,512), True).with_color(PALETTE["bg"])
    print(background.as_mat(100, 100))
    rect = xpr.rect(p0, p1, True).with_color(PALETTE["dust2"])
    shading = underline(rect, 100, 100, 2, PALETTE["shadow"])
    return background @ rect @ shading

PIXELS = run()
