def run():
    WIDTH = 100
    HEIGHT = 100
    from random import randint
    ret = xpr.Pixels()
    pix = (WIDTH//2, HEIGHT-1)
    trunk = []
    d = 20
    bg = xpr.rect((0,0), (WIDTH, HEIGHT), True).with_color((255,255,255,255))
    for _ in range(10):
        ret.push(xpr.Pixel(pix, xpr.RED))
        trunk.append(pix)
        dx = randint(-5, 5)
        dy = randint(-d, -d + 5)
        pix = (pix[0] + dx, pix[1] + dy)
        d -= 1
    ############## Generate left, right ##############
    d = 10
    left = []
    right = []
    origin = trunk[0]
    left.append((origin[0]-d, origin[1]))
    right.append((origin[0]+d, origin[1]))
    for (x,y) in trunk[1:]:
        dx = d
        dy = randint(1,2)
        left.append((x-dx, y+dy))
        right.append((x+dx, y-dy))
        d -= 1
    # for (l,r) in zip(left, right):
    #     ret @= xpr.line(l, r).with_color(xpr.BLUE)
    for (p0, p1) in zip(left, left[1:]):
        ret @= xpr.line(p0, p1).with_color(xpr.BLUE)
    for (p0, p1) in zip(right, right[1:]):
        ret @= xpr.line(p0, p1).with_color(xpr.BLUE)

    return WIDTH, HEIGHT, bg @ ret

WIDTH, HEIGHT, PIXELS = run()