from random import random, randint, choice, choices, sample, gauss

PALETTE = [
    (210, 173, 140, 255),
    (206, 155, 122, 255),
    (220, 194, 122, 255),
]

def get_color():
    return choices(PALETTE, [10, 10, 10], k=1)[0]

def corrode(rect, corrosion_param, p0, p1, corner):
    """ corrode a corner of a rectangle """
    # 01
    # 32
    to_remove = xpr.Pixels()
    if corner == 0:
        for (i, d) in enumerate(corrosion_param):
            for dy in range(d):
                to_remove.push(xpr.Pixel((p0[0]+i, p0[1]+dy), xpr.RED))
    elif corner == 1:
        for (i, d) in enumerate(corrosion_param):
            for dy in range(d):
                to_remove.push(xpr.Pixel((p1[0]-i-1, p0[1]+dy), xpr.RED))
    elif corner == 2:
        for (i, d) in enumerate(corrosion_param):
            for dy in range(d):
                to_remove.push(xpr.Pixel((p1[0]-i-1, p1[1]-dy-1), xpr.RED))
    elif corner == 3:
        for (i, d) in enumerate(corrosion_param):
            for dy in range(d):
                to_remove.push(xpr.Pixel((p0[0]+i, p1[1]-dy-1), xpr.RED))
    rect.sub_(to_remove)
    return rect


def corrosion(cols, dist):
    """list of corrosion distance per column/row"""
    ret = []
    last = dist
    for _ in range(cols):
        d = randint(last//2, last)
        ret.append(d)
        last = d
    # ret = [3,2,1]
    return ret

def underline(pixs, w, h, shading_len, color):
    ret = xpr.Pixels()
    mat = pixs.as_bool_mat(w, h)
    for i in range(h-1):
        for j in range(w-1):
            if mat[i][j] and not mat[i][j+1]:
                for k in range(shading_len):
                    ret.push(xpr.Pixel((i, j+k), xpr.RED))
    return ret.with_color(color)

def to_corners(bits):
    corners = []
    if 0b1000 & bits:
        corners.append(0)
    if 0b0001 & bits:
        corners.append(1)
    if 0b0010 & bits:
        corners.append(2)
    if 0b0100 & bits:
        corners.append(3)
    return corners

def gen_maze(n):
    import random
    imgx = imgy = n
    pixels = [[False for _ in range(imgx)] for _ in range(imgy)]
    mx = imgx; my = imgy # width and height of the maze
    maze = [[0 for x in range(mx)] for y in range(my)]
    dx = [0, 1, 0, -1]; dy = [-1, 0, 1, 0] # 4 directions to move in the maze
    BLACK = (0,0,0); WHITE = (255, 255, 255)
    color = [BLACK, WHITE] # RGB colors of the maze
    # start the maze from a random cell
    stack = [(random.randint(0, mx - 1), random.randint(0, my - 1))]
    while len(stack) > 0:
        (cx, cy) = stack[-1]
        maze[cy][cx] = 1
        # find a new cell to add
        nlst = [] # list of available neighbors
        for i in range(4):
            nx = cx + dx[i]; ny = cy + dy[i]
            if nx >= 0 and nx < mx and ny >= 0 and ny < my:
                if maze[ny][nx] == 0:
                    # of occupied neighbors must be 1
                    ctr = 0
                    for j in range(4):
                        ex = nx + dx[j]; ey = ny + dy[j]
                        if ex >= 0 and ex < mx and ey >= 0 and ey < my:
                            if maze[ey][ex] == 1: ctr += 1
                    if ctr == 1: nlst.append(i)
        # if 1 or more neighbors available then randomly select one and move
        if len(nlst) > 0:
            ir = nlst[random.randint(0, len(nlst) - 1)]
            cx += dx[ir]; cy += dy[ir]
            stack.append((cx, cy))
        else: stack.pop()
    # paint the maze
    for ky in range(imgy):
        for kx in range(imgx):
            pixels[kx][ky] = color[maze[my * ky // imgy][mx * kx // imgx]]
    grid = []
    for ky in range(imgy):
        row = []
        for kx in range(imgx):
            if pixels[kx][ky] == BLACK: # skip
                draw = False
                row.append((draw, 0))
            else:
                draw = True
                corners = 0b1111
                if kx!=0 and pixels[kx-1][ky] == WHITE: # LEFT
                    corners &= 0b0111
                    corners &= 0b1110
                if kx+1!=imgx and pixels[kx+1][ky] == WHITE: # RIGHT
                    corners &= 0b1011
                    corners &= 0b1101
                if ky!=0 and pixels[kx][ky-1] == WHITE: # UP
                    corners &= 0b0111
                    corners &= 0b1011
                if ky+1!=imgy and pixels[kx][ky+1] == WHITE: # DOWN
                    corners &= 0b1101
                    corners &= 0b1110
                row.append((draw, corners))
        grid.append(row)
    return grid

# image.save("Maze_" + str(mx) + "x" + str(my) + ".png", "PNG")