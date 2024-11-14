make big wordlists by applying transformations to words. best with one word or small wordlists.

Usage: cemplox [OPTIONS] --file <FILE>


Options:

  -f, --file <FILE>    path to a file containing words to transform
  
  -m, --min <MIN>      minimum length of final words [default: 1]
  
  -M, --max <MAX>      maximum length of final words [default: 8]
  
  -l, --leet           do leet transformations
  
  -c, --case           do case transformations
  
  -a, --append         append characters option
  
  -p, --prepend        prepend characters option
  
  -i, --insert         insert characters option
  
  -C, --chars <CHARS>  character set to use for app/pre/ins [default: "1234567890!@#$%^&*()-_=+[]{} "]
  
  -h, --help           Print help
  
  -V, --version        Print version



  example usage: cemplox --file words.txt -m 8 -M 10 --case --prepend (this applies a case transformation and prepends symbols, returning all combinations between 8 and 10 characters)
  

  installation:

  install cargo if not installed
  
  clone repo ( git clone https://github.com/ayohae/cemplox )
  
  build with: cargo build --release
  
  run inside repo directory with: cargo run --release
  
  OR
  
  install to path with cargo install --path /path/to/repo_directory 
  
  then run as a normal command line utility: cemplox --file <file>

  OR

  download executable from releases
