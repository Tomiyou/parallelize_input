# Executes given commands in parallel

* First command line argument is used as shell command
* {} are replaced by input data read from stdin line-per-line
* Shell command is executed for each line of stdin data

### Example:
```
$ ll | parallelize 'echo "Line: |{}|"'
Line: |-rw-rw-r--  1 tomi tomi  245 Jan 15 17:49 README.md|
Line: |total 36|
Line: |drwxrwxr-x  5 tomi tomi 4096 Jan 15 17:47 ./|
Line: |-rw-rw-r--  1 tomi tomi  180 Jan 15 17:43 Cargo.toml|
Line: |-rw-rw-r--  1 tomi tomi  155 Jan 15 17:43 Cargo.lock|
Line: |drwxrwxr-x  7 tomi tomi 4096 Jan 15 17:46 .git/|
Line: |drwxrwxr-x  2 tomi tomi 4096 Jan 15 17:43 src/|
Line: |drwxrwxr-x 14 tomi tomi 4096 Jan 11 19:02 ../|
Line: |drwxrwxr-x  4 tomi tomi 4096 Jan 15 17:50 target/|
Line: |-rw-rw-r--  1 tomi tomi    8 Jan 11 19:02 .gitignore|
```