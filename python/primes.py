from time import time

now = time()

primes = []

for num in range(2, 100001):
    prime = True
    for p in primes:
        if num % p == 0:
            prime = False
            break
    if prime:
        primes.append(num)

elapsed_time = round(time() - now, 3)

print("Found", len(primes), "prime numbers")
print("Should be 9592")
print("Took", elapsed_time, "seconds")
