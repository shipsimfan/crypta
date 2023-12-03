#!/bin/bash

sha_types=(224 256 384 512)

# Calculates the SHA`num` of stdin and outputs it to stdout
#
# Format:
# `sha num`
sha() {
    command="sha${1}sum"
    
    $command $2 | grep -o '^\S*' | tr -d "\n"
}


# Gets the name of a test from its `input` file
#
# Format:
# `get_test_name input`
get_test_name() {
    filename=$(basename -- "$1")
    echo "${filename%.*}"
}

# Generates a test from the `input` file
#
# Format:
# `test input`
test() {
    test_name=$(get_test_name $1)

    for sha_type in ${sha_types[@]}; do
        cat $1 | sha $sha_type > "${test_name}.${sha_type}"
    done
}

# Lists the tests in the folder and stores the array in `output`
#
# Format:
# `list_tests output`
list_tests() {
    local -n output=$1
    output=($(find . -maxdepth 1 -type f -name "*.in"))
}

main() {
    echo "Generating tests . . ."

    local tests
    list_tests tests in
    
    for test in ${tests[@]}; do
        test $test
    done
}

# Run `main`
main