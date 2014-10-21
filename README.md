rust-sudoku
===========

A custom and personal sudoku solver to learn a bit about the language Rust.


Build
-----

    $> cargo build --release
  
  
Run
---

    $> time target/release/sudoku sudokus/sudoku_hard.txt
    1) #############
    079|000|800
    300|005|060
    500|400|203
    -----------
    000|050|000
    040|206|070
    000|090|000
    -----------
    407|001|005
    030|500|007
    008|000|620
    vvvvvvvvvvv
    279|613|854
    314|825|769
    586|479|213
    -----------
    892|754|136
    145|236|978
    763|198|542
    -----------
    427|961|385
    631|582|497
    958|347|621
    
    real    0m0.017s
    user    0m0.013s
    sys     0m0.004s

  
