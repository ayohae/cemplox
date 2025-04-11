improve and make big wordlists by applying transformations to words. 

Usage: cemplox.exe [OPTIONS] --file <FILE> [COMMAND]

Commands:

      length
      
      count
      help    Print this message or the help of the given subcommand(s)

Options:

      -f, --file <FILE>                path to a input wordlist file
      
      -o, --out-file <OUT_FILE>        path to the output file. if not provided, output is written stdout
      
      -b, --batch-size <BATCH_SIZE>    number of input words processed at one time. (smaller batches lower RAM usage) [default: 5]
      
      -s, --sanitize                   sanitize the wordlist (trim and remove special chars), leaving a copy of the original
      
      -l, --leet                       apply leetspeak transforms
      
      -c, --case                       apply case transforms (lower to upper, upper to lower)
      
      -C, --chars <CHARS>              character set used for additional character transforms [default: "1234567890!@#$%^&*()-_=+[]{} "]
      
      -t, --tempfile-mode              enable tempfiles to reduce RAM usage
      
          --max-threads <MAX_THREADS>  max number of threads for parallel processing, # of cpu cores by default [default: 20]
          
          --dry-run                    dryrun mode. estimate counts without running
          
      -L, --log-level <LOG_LEVEL>      log level (error, warn, info, debug, trace) [default: info]
      
      -h, --help                       Print help
      
      -V, --version                    Print version


Length Options:

      -m, --min <MIN>  min length of output words [default: 1]
      
      -M, --max <MAX>  max length of output words [default: 16]
      
      -a, --append     append mode
      
      -p, --prepend    prepend mode
      
      -i, --insert     insert mode (very expensive, use with caution)
      
      -h, --help       Print help


Count Options:

      -a, --append <APPEND>    append this number of characters [default: 0]
      
      -p, --prepend <PREPEND>  prepend this number of characters [default: 0]
      
      -i, --insert <INSERT>    insert this number of characters (very expensive, use with caution) [default: 0]
      
      -h, --help               Print help




example usage:

this takes in a wordlist file and produces a sanitized wordlist with all possible leetspeak transformations and saves it to the outfile:

    cemplox --file [input wordlist file] --out-file [output wordlist file] --leet --sanitize

--------------------------

this takes in a wordlist file and appends 1 digit/special character to each of them and prints to stdout

    cemplox --file [input wordlist file] count --append 1

--------------------------

this takes in a wordlist file, sanitizes it, does all leetspeak transformations, then prepends characters until a maximum length of 8, and prints to stdout all words between length 1-8.

    cemplox --file [input wordlist file] -sl length --prepend -m 1 -M 8

--------------------------
  
installation:

install cargo if not installed

    curl https://sh.rustup.rs -sSf | sh

clone repo:

    git clone https://github.com/ayohae/cemplox

build with: 

    cargo build --release

run inside repo directory with: 

    cargo run --release

OR

install to PATH with

    cargo install --path /path/to/repo_directory 

then run as a normal command line utility: 

    cemplox --file [path/to/word/list]
