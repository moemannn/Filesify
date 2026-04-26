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
│        └── ui                         # ui rendering
|                ├── 
│                └── 
└── services
    ├── 
    └── 

presentation → services → adapters
                     ↑
                    config + app state