# Dependencies
raylib

## Build instructions
(linux build only atm)
install raylib to your home directory then run the build script by typing `./build.sh` or double clicking it in your file manager. 
there are commandline arguments which are as follows
```txt
	Usage: ./build-linux.sh [-hdusrcqq]
   -h  Show this information"
   -d  Faster builds that have debug symbols, and enable warnings
   -u  Run upx* on the executable after compilation (before -r)
   -s  Run strip on the executable after compilation (before -r)
   -r  Run the executable after compilation
   -c  Remove the temp/(debug|release) directory, ie. full recompile
   -q  Suppress this script's informational prints
   -qq Suppress all prints, complete silence (> /dev/null 2>&1)
```

# Known Bugs
None(at the moment)

# Other things
this is translated directly from rust at the moment, so please forgive any inaccuracies. 

- The only thing implemented right now is player movement and camera movement
- The art is likely not final. 
- the current code is in C because rust's raylib bindings have some issues.

# TODO(things coming in the future)
- animation(probably coming next commit)
- attacking
- simple game loop 
- 
