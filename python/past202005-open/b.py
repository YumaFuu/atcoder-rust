n, m, q = map(int, input().split())

scores = [[] for _ in range(n)]
questinos = [n] * m

def ab():
    print(22)

for _ in range(q):
    query = list(map(int, input().split()))
    a = query[1] - 1
    ab()
    if query[0] == 1:
        score = 0
        for q in scores[a]:
            score += questinos[q]
        print(score)

    elif query[0] == 2:
        q = query[2] - 1
        questinos[q] -= 1
        scores[a].append(q)
