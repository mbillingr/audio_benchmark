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
