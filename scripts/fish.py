def run():
    WIDTH = 200
    HEIGHT = 200

    import collections
    class OrderedSet(collections.MutableSet):
        def __init__(self, iterable=None):
            self.end = end = []
            end += [None, end, end]         # sentinel node for doubly linked list
            self.map = {}                   # key --> [key, prev, next]
            if iterable is not None:
                self |= iterable
        def __len__(self):
            return len(self.map)
        def __contains__(self, key):
            return key in self.map
        def add(self, key):
            if key not in self.map:
                end = self.end
                curr = end[1]
                curr[2] = end[1] = self.map[key] = [key, curr, end]
        def discard(self, key):
            if key in self.map:
                key, prev, next = self.map.pop(key)
                prev[2] = next
                next[1] = prev
        def __iter__(self):
            end = self.end
            curr = end[2]
            while curr is not end:
                yield curr[0]
                curr = curr[2]
        def __reversed__(self):
            end = self.end
            curr = end[1]
            while curr is not end:
                yield curr[0]
                curr = curr[1]
        def pop(self, last=True):
            if not self:
                raise KeyError('set is empty')
            key = self.end[1][0] if last else self.end[2][0]
            self.discard(key)
            return key
        def __repr__(self):
            if not self:
                return '%s()' % (self.__class__.__name__,)
            return '%s(%r)' % (self.__class__.__name__, list(self))
        def __eq__(self, other):
            if isinstance(other, OrderedSet):
                return len(self) == len(other) and list(self) == list(other)
            return set(self) == set(other)

    import math
    def binomial(i, n):
        """Binomial coefficient"""
        return math.factorial(n) / float(
            math.factorial(i) * math.factorial(n - i))

    def pixel_perfect(path):
        if len(path) == 1 or len(path) == 0:
            return path
        ret = []
        c = 0
        while c < len(path):
            if c > 0 and c+1 < len(path) \
                and (path[c-1][0] == path[c][0] or path[c-1][1] == path[c][1]) \
                and (path[c+1][0] == path[c][0] or path[c+1][1] == path[c][1]) \
                and path[c-1][0] != path[c+1][0] \
                and path[c-1][1] != path[c+1][1]:
                c += 1
            ret.append(path[c]);
            c += 1
        return ret

    def bernstein(t, i, n):
        """Bernstein polynom"""
        return binomial(i, n) * (t ** i) * ((1 - t) ** (n - i))
    def bezier(t, points):
        """Calculate coordinate of a point in the bezier curve"""
        n = len(points) - 1
        x = y = 0
        for i, pos in enumerate(points):
            bern = bernstein(t, i, n)
            x += pos[0] * bern
            y += pos[1] * bern
        return x, y
    def bezier_curve_range(n, points):
        """Range of points in a curve bezier"""
        for i in range(n):
            t = i / float(n - 1)
            yield bezier(t, points)

    def rectangle(p0, p1, color):
        ret = []
        for i in range(p0[0], p1[0]):
            for j in range(p0[1], p1[1]):
                ret.append(((i, j), color))
        return ret
    def draw_curve(Y, X_o, X_e, dx_o, dx_e, dy_o, dy_e):
        ret = OrderedSet()
        steps = 4000
        controlPoints = (
            (X_o, Y),
            (X_o + dx_o, Y - dy_o),
            (X_e - dx_e, Y - dy_e),
            (X_e, Y),
        )
        oldPoint = controlPoints[0]
        for point in bezier_curve_range(steps, controlPoints):
            # print(oldPoint[0], oldPoint[1], point[0], point[1])
            oldPoint = point
            ret.add((int(oldPoint[0]), int(oldPoint[1])))
        # ret.pop()
        # print(len(ret))
        ret = pixel_perfect(list(ret))
        # print(len(ret))
        return [(i,(0,0,0,255)) for i in ret]

    Y = 100
    X_o = 20
    X_e = 180
    dx = 50
    dy = 50
    PIXELS = rectangle((0,0),(WIDTH,HEIGHT),(255,0,0,255)) \
             + draw_curve(Y, X_o, X_e, dx, dx, dy, dy) \
             + draw_curve(Y, X_o, X_e, dx, dx, -dy, -dy)
    return WIDTH, HEIGHT, PIXELS

WIDTH, HEIGHT, PIXELS = run()