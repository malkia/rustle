# Exploring rust through bazel

## To start with, just try running it:
```
malkia@penguin:~/p/rustle$ bazel run :hello -c opt
INFO: Analyzed target //:hello (0 packages loaded, 0 targets configured).
INFO: Found 1 target...
Target //:hello up-to-date:
  bazel-bin/hello
INFO: Elapsed time: 0.258s, Critical Path: 0.00s
INFO: 0 processes.
INFO: Build completed successfully, 1 total action
INFO: Build completed successfully, 1 total action
hello
```

## Note:
On first run, it'll take significantly more,
as the rust toolchain (compiler+library) needs
to be downlaoded. Subsequent runs, even after ```bazel clean``` would be still fast as the downloaded
artifacts are globally cached.

You can issue ```bazel fetch``` when you have
good connection, and then can work offline, once
the toolchain is there.

## Getting it running on ***Windows***:
Things are not working perfectly on windows right now,
seems like the rule is expecting the binary file (hello.exe)
to be built without extension, but it does have one (as expected)
hence it's reporting: "not all outputs were created or valid"
look in the ```WORKSPACE``` file for more info

A crude workaround, is to:
```
c:\p\rustle> bazel build :hello & "bazel-bin\hello"
```
