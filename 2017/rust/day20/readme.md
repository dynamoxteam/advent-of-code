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

## Usage

```shell
cargo run <input file>
```

## Test

```shell
cargo test
```