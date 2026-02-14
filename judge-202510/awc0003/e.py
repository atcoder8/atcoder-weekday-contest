n, m = map(int, input().split())
ww = list(map(int, input().split()))
cc = sorted(map(int, input().split()), reverse=True)

dp = [(m, 0)] * (1 << n)
dp[0] = (0, 0)
for bits in range(1 << n):
    curr_track, sum_weight = dp[bits]

    if curr_track == m:
        continue

    for i, w in enumerate(ww):
        if bits >> i & 1 == 1:
            continue

        next_bits = bits | (1 << i)
        if sum_weight + w <= cc[curr_track]:
            dp[next_bits] = min(dp[next_bits], (curr_track, sum_weight + w))
        elif curr_track + 1 < m and w <= cc[curr_track + 1]:
            dp[next_bits] = min(dp[next_bits], (curr_track + 1, w))

print("Yes" if dp[(1 << n) - 1][0] != m else "No")
