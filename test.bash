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

try 0 0
try 42 42
try 21 1+2+3+4+5+6
try 23 13+20-10
try 2 1+2*3-5

echo ok
