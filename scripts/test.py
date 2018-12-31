assert(xpr.add(1,10) == 11)
assert(xpr.add(-1,10) == 9)

a = xpr.Pixel((1,1), xpr.RED)
b = xpr.Pixel((3,3), xpr.GREEN)
c = xpr.Pixel((3,3), xpr.BLUE)
d = xpr.Pixel((4,4), xpr.BLUE)

pixs = xpr.Pixels(a,b)
other = xpr.Pixels(c,d)
pixs.extend(other)
print(pixs)

PIXELS = [a, b]