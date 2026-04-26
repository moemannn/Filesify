[]Filestify/src[] tree
.
├── main.rs
├── adapters/                           # system / distro intergration
│        ├── 
│        └── 
├── app/                                # runtime application state (current session only)
│        └── state.rs                   # runtime state 
├── config/                             # app configuration
│        ├── 
│        └── package_managers.rs        # source definitions / commands mapping
├── presentation                        # presentation
│        ├── input                      # input handling
|        │       ├── 
│        │       └── 
│        └── ui                         # tui rendering
|                ├── 
│                └── 
└── services
    ├── 
    └── 

[]Filestify[] tree 
.
├── Cargo.lock                          # dependency lock file
├── Cargo.toml                          # project config
├── .gitignore                          # git ignore
├── README.md                           # read me
├── src/                                # source code
├── tests/                              # integration tests
└── target/                             # compiled build output


presentation → services → adapters
                     ↑
                    config + app state