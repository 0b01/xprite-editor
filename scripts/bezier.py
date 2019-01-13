# pylint: disable=E0602
WIDTH = 480
HEIGHT = 270

def draw_curve(Y, X_o, X_e, dx_o, dx_e, dy_o, dy_e):
    controlPoints = (
        (X_o, Y),
        (X_o + dx_o, Y - dy_o),
        (X_e - dx_e, Y - dy_e),
        (X_e, Y),
    )
    ret = xpr.bezier(
        controlPoints[0],
        controlPoints[1],
        controlPoints[2],
        controlPoints[3],
    )
    return ret.with_color(xpr.BLUE)

MARGIN_X = WIDTH // 8

X_o = MARGIN_X
X_e = WIDTH - MARGIN_X
Y = HEIGHT // 2
dx = HEIGHT // 4
dy = HEIGHT // 4
PIXELS = xpr.rect((0,0),(WIDTH,HEIGHT),True).with_color(xpr.GREEN) \
       @ draw_curve(Y, X_o, X_e, dx, dx, dy, dy) \
       @ draw_curve(Y, X_o, X_e, dx, dx, -dy, -dy)