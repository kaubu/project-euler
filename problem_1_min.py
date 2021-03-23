n_sum = 0
for n in range(1000): n_sum += n if not n % 3 or not n % 5 else 0
print(n_sum)
