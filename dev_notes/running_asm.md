# How to Run Assembly

----

## Linux

- Compile to Binary: ```gcc -o <name> <file>.s```
- Run Binary: ```./<name>```
- View Exit Code: ```echo $?```

- Or Run & View Exit Code: ```./<name>; echo $?```
- - Compile, Run, and View Exit Code: ```gcc -o <name> <file>.s; ./<name>; echo $?```

## Windows

- Ensure GCC is Installed
- Compile to Binary: ```gcc <file>.s -o <name>.exe```
- Run Binary: ```.\<name>.exe```
- View Exit Code: ```$LASTEXITCODE```

- Or Run & View Exit Code: ```.\<name>; $LASTEXITCODE```
- Compile, Run, and View Exit Code: ```gcc <file>.s -o <name>.exe; .\<name>; $LASTEXITCODE```