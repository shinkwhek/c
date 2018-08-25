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

try 0 'return 0;'
try 42 'return 42;'
try 21 'return 1+2+3+4+5+6;'
try 23 'return 13+20-10;'
try 2 'return 1+2*3-5;'
try 4 'return 8/2;'
try 18 'return 9*4/2;'

echo ok
