# Find the sum of multiples of 3 and 5 below 1000
# https://projecteuler.net/problem=1
MIN = 1
MAX = 999
n_sum = 0

for n in range(MIN, MAX+1):
	if not n % 3 or not n % 5: n_sum += n

print(n_sum)
