#!/bin/bash

if [ "$#" -ne 1 ]
then
    echo 'incorrect number of arguments'
    echo "usage: $0 QUESTIONID"
    exit 1
fi

if [[ ! "$1" =~ ^[0-9]{1,6}$ ]]
then
    echo 'invalid input format'
    echo "usage: $0 QUESTIONID"
    exit 1
fi

id="$(printf %06d $1)"
filename="src/q${id}.rs"

if [ -f $filename ]
then
    echo "file $filename already exists"
    exit 1
fi

cat <<EOF > $filename
// http://codegolf.stackexchange.com/q/$1/3808

pub fn q$id() {
}

#[test]
fn test$id() {
}
EOF

cat <<EOF >> src/lib.rs

// http://codegolf.stackexchange.com/q/$1/3808
mod q$id;
pub use q$id::q$id;
EOF
