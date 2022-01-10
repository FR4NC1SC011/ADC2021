# Imports 
from collections import Counter

# Input handling
with open('input_day5.txt') as fin:
    data = fin.read().splitlines()

# Part 1
def part1():
    allPoints = []
    for line in data:
        # Parse data into usable contents
        firstPoint, secondPoint = line.split("->")
        x1, y1 = tuple(map(int, firstPoint.split(',')))
        x2, y2 = tuple(map(int, secondPoint.split(',')))

        # Consider vertical & horizontal lines only
        if x1 == x2 or y1 == y2:
            for x in range(min(x1, x2), max(x1, x2)+1):
                for y in range(min(y1, y2), max(y1, y2)+1):
                    allPoints.append((x, y))
    
    
    return len([point for point in Counter(allPoints).values() if point > 1])

# Part 2
def part2():
    allPoints = []
    for line in data:
        # Parse data into usable contents
        firstPoint, secondPoint = line.split("->")
        x1, y1 = tuple(map(int, firstPoint.split(',')))
        x2, y2 = tuple(map(int, secondPoint.split(',')))

        # Consider vertical & horizontal lines only
        if x1 == x2 or y1 == y2:
            for x in range(min(x1, x2), max(x1, x2)+1):
                for y in range(min(y1, y2), max(y1, y2)+1):
                    allPoints.append((x, y))

        # For diagonal lines
        else:
            xinc = 1 if x1 < x2 else -1     # 1 when diagonal is going from left to right
            yinc = 1 if y1 < y2 else -1     # 1 when diagnoal is going from bottom to top
            y = y1                          
            for x in range(x1, x2 + xinc, xinc):    # travel in direction of diagonal
                allPoints.append((x, y))
                y += yinc                           # follow the diagonal with said direction


    return len([point for point in Counter(allPoints).values() if point > 1])



print("Answer to part 1: ", part1())
print("Answer to part 2: ", part2())
