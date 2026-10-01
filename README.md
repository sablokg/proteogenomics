# proteogenomics
 - rust crate for hmm domain search, selection and annotation mapper.
 - it reports only chains with atoms for the id and sequence and euclidean distance for all irrespective of chain and atoms.
 - Euclidean distance chain comparsion for two point coordinate protein.
 - Euclidean distance chain coordinate comparsion for all protein atoms of the chain.


 ```
  cargo build
 ```

 ```
 _                                                               _
                        _                                                               _              
  _ __    _ __    ___   | |_    ___    ___     __ _    ___   _ __     ___    _ __ ___   (_)   ___   ___ 
 | '_ \  | '__|  / _ \  | __|  / _ \  / _ \   / _` |  / _ \ | '_ \   / _ \  | '_ ` _ \  | |  / __| / __|
 | |_) | | |    | (_) | | |_  |  __/ | (_) | | (_| | |  __/ | | | | | (_) | | | | | | | | | | (__  \__ \
 | .__/  |_|     \___/   \__|  \___|  \___/   \__, |  \___| |_| |_|  \___/  |_| |_| |_| |_|  \___| |___/
 |_|                                          |___/                                                     

hmm proteome analyze
    ************************************************
    Author Gaurav Sablok,
    Email: gsablok@proton.me
   ************************************************

Usage: proteogenomics <COMMAND>

Commands:
  target-hmm             select the targets from the hmm
  score-hmm              select the scores from the hmm
  evalue-hmm             select the evalues from the hmm
  eval-target            select the targets based on evalue and target
  targetcounts           prepare the unique counts of the domains
  pdb-id                 
  pdb-sequence           extract the sequence of the pdf file
  euclidean-comparative  calculate the euclidean distance bettwen two chain coordinates
  euclidean-all          calculates the euclidean distance for all chain atoms
  help                   Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

 ```
 score-hmm ./sample-file/samplehmmalignment.txt 12.5
 ==> filtered-scores-with-all-targets.txt <==
 HTH_26  63      WP_003872347.1  189     0.00000000000023        37.9    1       60      14      72      14      75      Cro/C1-type
 HTH_24  48      WP_003872375.1  193     0.0000023       13.8    17      37      34      54      33      54      Winged
 HTH_24  48      WP_003872469.1  148     0.00000000000000000054  55.1    1       48      4       51      4       51      Winged
 HTH_24  48      WP_003872548.1  161     0.0000062       13.1    7       47      47      87      43      88      Winged
 HTH_26  63      WP_003872666.1  178     0.00000000058   27.2    2       62      5       64      5       65      Cro/C1-type

 ==> filtered-score.txt <==
 HTH_XRE 56      WP_003872292.1  132     0.00000003      20.9    17      56      35      75      35      75      -
 HTH_XRE 56      WP_003872307.1  249     0.00000048      17.1    2       34      149     187     148     195     -
 HTH_XRE 56      WP_003872347.1  189     0.0000000000000000000022        63.4    1       56      14      69      14      69      -
 HTH_26  63      WP_003872347.1  189     0.00000000000023        37.9    1       60      14      72      14      75      Cro/C1-type
 HTH_24  48      WP_003872375.1  193     0.0000023       13.8    17      37      34      54      33      54      Winged
 ```
 ```
 target-hmm ./sample-file/samplehmmalignment.txt Winged

 head -n 3 filtered-target.txt
 HTH_24  48      WP_003872375.1  193     0.0000023       13.8    17      37      34      54      33      54      Winged
 HTH_24  48      WP_003872375.1  193     0.0000023       -3.9    23      31      180     188     178     189     Winged
 HTH_24  48      WP_003872469.1  148     0.00000000000000000054  55.1    1       48      4       51      4       51      Winged

 evalue-hmm ./sample-file/samplehmmalignment.txt 1.3e-5

==> ./sample-file/evalue-filter-with-all-targets.txt <==
 HTH_XRE 56      210     WP_003872175.1  0.000023        10.9    3       30      10      56      8       84      -
 HTH_XRE 56      210     WP_003872175.1  0.000023        -2.6    14      24      139     149     138     150     -
 HTH_XRE 56      210     WP_003872175.1  0.000023        -3.4    44      53      187     196     185     198     -
 HTH_24  48      253     WP_003872618.1  0.000016        11.5    22      47      43      68      42      69      Winged

==> ./sample-file/evalues-filter-with-target-annotated.txt <==
 HTH_24  48      WP_003872618.1  0.000016        11.5    22      47      43      68      42      69      Winged
 HTH_24  48      WP_003872618.1  0.000016        -3.9    32      39      117     124     116     127     Winged
 HTH_24  48      WP_003872953.1  0.000023        10.1    19      43      47      71      41      72      Winged
 HTH_24  48      WP_003872953.1  0.000023        -1.6    31      42      192     203     191     204     Winged

```
```
 eval-target ./sample-file/samplehmmalignment.txt 1.3e-5 Winged
 head -n 3 evalue-filter-with-all-targets.txt
 HTH_24  48      253     WP_003872618.1  0.000016        11.5    22      47      43      68      42      69      Winged
 HTH_24  48      253     WP_003872618.1  0.000016        -3.9    32      39      117     124     116     127     Winged
 HTH_24  48      245     WP_003872953.1  0.000023        10.1    19      43      47      71      41      72      Winged

 eval-target ./sample-file/samplehmmalignment.txt 1.3e-5 Cro/C1-type
 head -n 1 evalue-filter-with-all-targets.txt
 HTH_26  63      213     WP_003876921.1  0.000033        11.4    10      36      42      67      40      77      Cro/C1-type

```
```
 targetcounts ./sample-file/samplehmmalignment.txt
 The domain aligned targets for the enrichment are: {"Winged": 1, "Cro/C1-type": 1, "Domain": 1}

```
```
 euclidean-comparative sample.pdb A 10 10 N CA
 The eucledian distance between to given coordinates of the same chain is 1.4656264
 euclidean-comparative sample.pdb A 1 1 N CA
 The eucledian distance between to given coordinates of the same chain is 1.4909409
```
```
 euclidean-all sample.pdb A
  The vector containing the eucleadian distance for those chain atoms are:
   [1.4909409, 1.5401275, 1.254727, 3.479588, 1.4994173, 1.4884146 ...]

 and will write a eucledian file as
 32.231  15.281  -13.143 32.184  14.697  -11.772 1.4909409
 32.184  14.697  -11.772 33.438  13.89   -11.387 1.5401275
 33.438  13.89   -11.387 34.102  13.07   -12.066 1.254727
 34.102  13.07   -12.066 30.797  14.065  -11.625 3.479588
 30.797  14.065  -11.625 30.976  12.589  -11.819 1.4994173
 30.976  12.589  -11.819 29.608  12.016  -11.694 1.4884146
 29.608  12.016  -11.694 28.942  12.335  -12.945 1.4526931
 28.942  12.335  -12.945 27.67   12.696  -13.05  1.3263968
 27.67   12.696  -13.05  26.901  12.777  -11.999 1.3048087
 26.901  12.777  -11.999 27.161  12.963  -14.255 2.2785378

```

- To install windows version:
```
rustup component add llvm-tools
rustup target add x86_64-pc-windows-msvc
git clone https://github.com/IBCHgenomic/varlinker.git
cd ensemblcov
cargo xwin build --target x86_64-pc-windows-msvc
```

