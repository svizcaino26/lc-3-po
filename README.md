# LC-3-PO
An LC-3 spec compliant virtual machine implemented in Rust.

```

                                     /~\                           
                                    |o o)      We're doomed!       
                                    _\=/_                          
                    ___        #   /  _  \   #                     
                   /() \        \\//|/.\|\\//                      
                 _|_____|_       \/  \_/  \/                       
                | | === | |         |\ /|                          
                |_|  O  |_|         \_ _/                          
                 ||  O  ||          | | |                          
                 ||__*__||          | | |                          
                |~ \___/ ~|         []|[]                          
                /=\ /=\ /=\         | | |                          
________________[_]_[_]_[_]________/_]_[_\_________________________
```

## Running an LC-3 program
The VM runs compiled `.obj` files written in assembly for the LC-3 arquitecture.

Examples are provided in the `examples` directory, and more programs can be placed in the directory
to make them available through the CLI for execution.

Just run
```
cargo run --bin cli
```
