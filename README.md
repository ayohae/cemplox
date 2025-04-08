improve and make big wordlists by applying transformations to words. 

Usage: cemplox.exe [OPTIONS] --file <FILE> [COMMAND]

Commands:
  length
  count
  help    Print this message or the help of the given subcommand(s)

Options:
  -f, --file <FILE>              path to a file containing words to transform
  
  -o, --out-file <OUT_FILE>      path to the output file
  
  -b, --batch-size <BATCH_SIZE>  how many words from the wordlist are processed at one time. lower if RAM consumption is too high. increase to increase processing time. you can safely raise this if you aren't doing many operations/transformations at the same time [default: 5]
  -s, --sanitize                 sanitize the wordlist
  -l, --leet                     do leet transformations
  -c, --case                     do case transformations
  -C, --chars <CHARS>            character set to use for app/pre/ins [default: "1234567890!@#$%^&*()-_=+[]{} "]
  -h, --help                     Print help
  -V, --version                  Print version

  Length Options:
    -m, --min <MIN>  minimum length of final words [default: 1]
    -M, --max <MAX>  maximum length of final words [default: 16]
    -a, --append     append characters option
    -p, --prepend    prepend characters option
    -i, --insert     insert characters option
    -h, --help       Print help

  Count Options:
    -a, --append <APPEND>    append characters option [default: 0]
    -p, --prepend <PREPEND>  prepend characters option [default: 0]
    -i, --insert <INSERT>    insert characters option [default: 0]
    -h, --help               Print help



example usage:

cemplox --file [input wordlist file] --out-file [output wordlist file] --leet --sanitize : this takes in a wordlist file and produces a sanitized wordlist with all possible leetspeak transformations and saves it to the outfile.

cemplox --file [input wordlist file] count --append 1 : this takes in a wordlist file and appends 1 digit/special character to each of them and prints to stdout

cemplox --file [input wordlist file] -sl length --prepend -m 1 -M 8 : this takes in a wordlist file, sanitizes it, does all leetspeak transformations, then prepends characters until a maximum length of 8, and prints to stdout all words between length 1-8.

  installation:

  install cargo if not installed
  
  clone repo ( git clone https://github.com/ayohae/cemplox )
  
  build with: cargo build --release
  
  run inside repo directory with: cargo run --release
  
  OR
  
  install to path with cargo install --path /path/to/repo_directory 
  
  then run as a normal command line utility: cemplox --file [path/to/word/list]

  OR

  download executable from releases
