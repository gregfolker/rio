## rio

## TODO:
- Add flag arguments
  - `--block-size` for size of I/O (Default=4KiB)
  - `--io-engine` for the I/O engine to use
  - `--queue-depth` for the max amount of outstanding I/O to wait for (Default=1)
  - `--buffered` for enabling buffered I/O instead of direct (Default=disabled)
  - `--offset` for the starting offset to begin I/O to in the `--target` (Default=0)
  - `--overwrite` for allowing existing data to be overwritten by new I/O (Default=disabled)
  - `--max-bandwidth` for limiting the total amount of bandwidth the tool is allowed to consume
  - `--sync` for using synchronous I/O instead of asynchronous (Only valid with `--buffered` I/O)
  - `--iterations` for specifying the amount of times to perform this job (Default=1)
  - ~~`--num-threads` for specfiying the number of worker threads to perform this job (Default=1)~~
