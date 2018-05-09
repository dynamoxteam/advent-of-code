# Day Day 20: Particle Swarm

## Part 1 solution

```
v[n] = v[n-1] + a
v[n] = v[0] + a * n

x[n] = x[n-1] + v[n]

               n
x[n] = x[0] + Sum(v[k])
              k=1

               n
x[n] = x[0] + Sum(v[0] + a * k)
              k=1

                              n
x[n] = x[0] + n * v[0] + a * Sum(k)
                             k=1

x[n] = x[0] + n * v[0] + a * n * (n + 1) / 2

lim x[n] ≈ 1/2 * a * n^2
 n->inf
```

In the long term, the particle that stays closest to the 0-position is the one
that has the least absolute acceleration.

## Part 2 solution

```
xa[n] = xb[n]
xa[0] + n * va[0] + aa * n * (n + 1) / 2 = xb[0] + n * vb[0] + ab * n * (n + 1) / 2
(aa - ab) * n * (n + 1) / 2 + (va[0] - vb[0]) * n + (xa[0] - xb[0]) = 0

da = aa - ab
dv = va[0] - vb[0]
dx = xa[0] - xb[0]
```

### For `da` == 0:

```
dv * n + dx = 0
n = -dx / dv;
```

A collision of two particles will happen at the tick `n` if all the following conditions are met:

1. `dx` is divisible by `dv`;
2. `n` is greater than or equal to zero.

### For `da` != 0:

```
da * n * (n + 1) / 2 + dv * n + dx = 0
da/2 * n^2 + (dv + da/2) * n + dx = 0

n = [-(dv + da/2) +- sqrt((dv + da/2)^2 - 2*da*dx)] / da
n = [-(2*dv + da) +- sqrt((2*dv + da)^2 - 8*da*dx)] / (2 * da)

delta = (2*dv + da)^2 - 8*da*dx
numerator = -(2*dv + da) +- sqrt(delta)
denominator = 2 * da

n = numerator / denominator
```

A collision of two particles will happen at the tick `n` if all the following conditions are met:

1. `delta` is greater than or equal to zero.
2. `delta` is a perfect square;
3. `numerator` is divisible by `denominator`;
4. `n` is greater than or equal to zero.

## Usage

```shell
cargo run <input file>
```

## Test

```shell
cargo test
```