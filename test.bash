#!/bin/bash

try() {
  expected="$1"
  input="$2"

  ./target/debug/c "$input" > tmp.s
  gcc -static -o tmp tmp.s
  ./tmp
  actual="$?"

  if [ "$actual" == "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input expected, but got $actual"
    exit 1
  fi
} 

try 0 'int main() {return 0;}'
try 42 'int main() {return 42;}'
try 21 'int main() {return 1+2+3+4+5+6;}'
try 23 'int main() {return 13+20-10;}'
try 2 'int main() {return 1+2*3-5;}'
try 4 'int main() {return 8/2;}'
try 18 'int main() {return 9*4/2;}'
try 5 'int f1() {return 2;} int f2() {return f1()*f1()+1;} int main() {return f2();}'

echo ok
