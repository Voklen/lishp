```
ls
cd src/
cd ..
echo hello world
ls (echo src)
pipe (ls) (cat)
set-env GREETING Hello
get-env GREETING
let place world
echo $place
echo (get-env GREETING) $place
if true (ls) (echo default)
if false (ls) (echo default)
```
