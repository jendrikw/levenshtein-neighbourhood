from collections import deque


def countFixedDistance(s: str, k: int) -> int:
    n = int('1' + s, 2)
    distances: dict[int, int] = {n: 0}
    queue = deque(((n, len(s)),))
    valid: set[int] = set()
    while len(queue) > 0:
        current, bit_count = queue.popleft()
        next_dist = distances[current] + 1
        for e in range(bit_count):
            left = current >> e
            left_1 = left << 1
            pow = 1 << e
            right = current % pow
            bp1 = bit_count + 1
            for next, next_bit_count in (
                    ((left >> 1 << e) | right, bit_count - 1),
                    (current ^ pow, bit_count),
                    ((left_1 << e) | right, bp1),
                    (((left_1 | 1) << e) | right, bp1),
            ):
                if next in distances: continue
                distances[next] = next_dist
                if next_dist < k:
                    queue.append((next, next_bit_count))
                else:
                    valid.add(next)
        for next in (
                pow << 2 | current,
                3 << bit_count ^ current,
        ):
            if next in distances: continue
            distances[next] = next_dist
            if next_dist < k:
                queue.append((next, bit_count + 1))
            else:
                valid.add(next)
    return len(valid)


s = '0'
v = 0
while True:
    for k in range(1, len(s)+1):
        print((len(s),k), countFixedDistance(s,k), flush=True)
    s += '1100'[v%4]
    v += 1
    print(s)
#print(sorted(countFixedDistance("011", 2)))#