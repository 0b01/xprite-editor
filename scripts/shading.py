import builtins
builtins.xpr = xpr
import sys
sys.path.append(".")

WIDTH = 100
HEIGHT = 100
def run():
    from scripts.common import PALETTE, sample, corrosion, corrode, underline

    ret = xpr.Pixels()
    p0 = (10,10)
    p1 = (45, 30)
    rect = xpr.rect(p0, p1, True).with_color(PALETTE[2])
    corners = sample([0, 1, 2, 3], k=4)
    for corner in corners:
        cols = dist = 5
        c = corrosion(cols, dist)
        rect = corrode(rect, c, p0, p1, corner)
    shading = underline(rect, 100, 100, 2, PALETTE[1])
    ret @= rect
    ret @= shading
    return ret

PIXELS = run()
