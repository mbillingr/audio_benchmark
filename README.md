### Results

#### Lenovo Yoga, Arch Linux, i7-4500U CPU @ 1.80GHz

Normal:
```
test bench_dynamic_0 ... bench:   1,442,457 ns/iter (+/- 9,253)
test bench_dynamic_1 ... bench:   1,741,358 ns/iter (+/- 17,446)
test bench_full_0    ... bench:     880,297 ns/iter (+/- 17,153)
test bench_full_1    ... bench:   1,176,340 ns/iter (+/- 18,781)
test bench_static_0  ... bench:   1,441,766 ns/iter (+/- 12,157)
test bench_static_1  ... bench:   1,098,767 ns/iter (+/- 16,836)
```

With weird "optimization":
```
test bench_dynamic_0 ... bench:     802,922 ns/iter (+/- 8,478)
test bench_dynamic_1 ... bench:   1,268,823 ns/iter (+/- 23,551)
test bench_full_0    ... bench:     800,890 ns/iter (+/- 12,268)
test bench_full_1    ... bench:   1,160,340 ns/iter (+/- 17,903)
test bench_static_0  ... bench:     787,459 ns/iter (+/- 25,691)
test bench_static_1  ... bench:   1,190,604 ns/iter (+/- 33,860)
```

#### PC, Arch Linux, i5-3570 CPU @ 3.40GHz

Normal:
```
test bench_dynamic_0 ... bench:   1,359,656 ns/iter (+/- 23,746)
test bench_dynamic_1 ... bench:   1,396,772 ns/iter (+/- 26,782)
test bench_full_0    ... bench:     886,754 ns/iter (+/- 17,633)
test bench_full_1    ... bench:   1,148,865 ns/iter (+/- 33,511)
test bench_static_0  ... bench:   1,344,096 ns/iter (+/- 42,925)
test bench_static_1  ... bench:   1,021,172 ns/iter (+/- 34,216)
```

With weird "optimization":
```
test bench_dynamic_0 ... bench:     898,363 ns/iter (+/- 29,522)
test bench_dynamic_1 ... bench:   1,320,817 ns/iter (+/- 64,063)
test bench_full_0    ... bench:     875,815 ns/iter (+/- 28,680)
test bench_full_1    ... bench:   1,163,893 ns/iter (+/- 29,621)
test bench_static_0  ... bench:     866,500 ns/iter (+/- 27,286)
test bench_static_1  ... bench:   1,246,030 ns/iter (+/- 36,106)
```

