# /usr/bin/env bash

for i in *.png; do
    pngquant 32 -f --strip --speed 1 -o ../html/img/interface/$i -- $i
done
