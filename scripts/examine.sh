#!/usr/bin/env bash

# examine disassembly
# $1 -- function name; rest -- object files
# i.e.
# scripts/examine.sh bzero bld/usr/lib/libc.a
# 0000000000000000 <bzero>:
#    0:   48 89 f2                mov    %rsi,%rdx
#    3:   31 f6                   xor    %esi,%esi
#    5:   e9 00 00 00 00          jmpq   a <bzero+0xa>
#    0:

fn=$1; shift 1
exec objdump -d "$@" |
awk " /^[[:xdigit:]].*<$fn>/,/^\$/ { print \$0 }" |
awk -F: -F' '  'NR==1 {  offset=strtonum("0x"$1); print $0; }
                NR!=1 {  split($0,a,":"); rhs=a[2]; n=strtonum("0x"$1); $1=sprintf("%x", n-offset); printf "%4s:%s\n", $1,rhs }'
