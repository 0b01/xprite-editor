WIDTH = 1000
HEIGHT = 1000

def run():
    WIDTH = 1000
    HEIGHT = 1000

    from random import random, randint, choice, choices, sample, gauss
    from tqdm import tqdm
    PALETTE = [
        (210, 173, 140, 255),
        (206, 155, 122, 255),
        (220, 194, 122, 255),
    ]

    def get_color():
        return choices(PALETTE, [10, 10, 10], k=1)[0]

    from collections import namedtuple
    from operator import itemgetter
    from pprint import pformat

    class Node(namedtuple('Node', 'location left_child right_child')):
        def __repr__(self):
            return pformat(tuple(self))

    def kdtree(point_list, depth=0):
        try:
            k = len(point_list[0]) # assumes all points have the same dimension
        except IndexError as e: # if not point_list:
            return None
        # Select axis based on depth so that axis cycles through all valid values
        axis = depth % k

        # Sort point list and choose median as pivot element
        point_list.sort(key=itemgetter(axis))
        median = len(point_list) // 2 # choose median

        # Create node and construct subtrees
        return Node(
            location=point_list[median],
            left_child=kdtree(point_list[:median], depth + 1),
            right_child=kdtree(point_list[median + 1:], depth + 1)
        )

    def to_rects(node, p0, p1, is_vert=True):
        (x0, y0) = p0
        (x1, y1) = p1
        if not hasattr(node, 'location'):
            return [((x0, y0), (x1, y1))]
        (xm, ym) = node.location
        if is_vert:
            left = to_rects(node.left_child, (x0, y0), (xm, y1), False)
            right = to_rects(node.right_child, (xm, y0), (x1, y1), False)
        else:
            left = to_rects(node.left_child, (x0, y0), (x1, ym), True)
            right = to_rects(node.right_child, (x0, ym), (x1, y1), True)
        return left + right

    from random import randint

    def random_point(p0, p1):
        (x0, y0) = p0
        (x1, y1) = p1
        # r0 = int(gauss((x0 + x1) / 2, (x1 - x0) / 8))
        # r1 = int(gauss((y0 + y1) / 2, (y1 - y0) / 8))
        r0 = randint(x0, x1)
        r1 = randint(y0, y1)
        return [r0, r1]

    n = 7
    points = [random_point((10, 10), (990, 990)) for _ in range(n)]

    # points = [(2,3), (5,4), (9,6), (4,7), (8,1), (7,2)]
    tree = kdtree(points)
    # print(tree)
    rects = to_rects(tree, (0,0), (1000,1000))
    print(rects)
    starts = [rect[0][0] for rect in rects]
    print(starts)
    working_set = set()
    working_set.update(set([rect for rect in rects if rect[0][0]==0]))
    print(working_set)

    def rectangle(p0, p1, color):
        ret = []
        for i in range(p0[0], p1[0]):
            for j in range(p0[1], p1[1]):
                ret.append(((i, j), color))
        return ret

    ret = []
    vals = []
    for (p0, p1) in rects[::-1]:
        p0 = (p0[0]+10, p0[1]+10)
        p1 = (p1[0]-10, p1[1]-10)
        pixs = rectangle(p0, p1, get_color())
        val = ((p0, p1), pixs)
        vals.append(val)
        ret += pixs

    return ret

PIXELS = run()
