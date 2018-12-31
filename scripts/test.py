# pylint: disable=E0602

assert(xpr.add(1,10) == 11)
assert(xpr.add(-1,10) == 9)

a = xpr.Pixel((1,1), xpr.RED)
b = xpr.Pixel((3,3), xpr.GREEN)
c = xpr.Pixel((3,3), xpr.BLUE)
d = xpr.Pixel((4,4), xpr.BLUE)

e = xpr.Pixel((40,40), xpr.BLUE)
f = xpr.Pixel((90,90), xpr.BLUE)

pixs = xpr.Pixels(a,b)
other = xpr.Pixels(c,d)

pixs @= other
pixs @= xpr.rect(e,f,True).with_color((0,0,255,255))

PIXELS = pixs
