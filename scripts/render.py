def test():
    pixels = []
    for i in range(100):
        for j in range(100):
            pixels.append(
                ((i,j), (100+i,100+j,i+j,255))
            )
    return pixels

pixels = test()
