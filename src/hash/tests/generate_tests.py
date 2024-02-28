#!/usr/bin/python3

import os;
import os.path;
import hashlib;

ALGORITHMS = ["sha1", "sha224", "sha256", "sha384", "sha512"]

REPEATED_TESTS = [("a_rep", 1_000_000), ("long_rep", 16_777_216)]

# Generates the test input file name
def generate_input_file(test_name, count):
    if count > 1:
        return "inputs/" + test_name + ".inr"
    else:
        return "inputs/" + test_name + ".in"

# Generates the test output file name
def generate_output_file(test_name, algorithm):
    return "outputs/" + test_name + "." + algorithm

# Generates the test output for `test_name` using `algorithm`, using the input `count` times
def generate_test(name, algorithm, count):
    input_file = generate_input_file(name, count)
    output_file = generate_output_file(name, algorithm)

    contents = open(input_file, "rb").read().rstrip()

    hasher_constructor = "hashlib." + algorithm + "()";
    hasher = eval(hasher_constructor)
    for _ in range(count):
        hasher.update(contents)

    output = hasher.hexdigest()

    contents = open(output_file, "w+").write(output)

# Lists the tests from the `root` directory with `extension`
def list_tests():
    tests = []

    for entry in os.listdir("inputs"):
        if not os.path.isfile(os.path.join("inputs", entry)):
            continue

        if not entry.endswith(".in"):
            continue

        tests.append(os.path.splitext(entry)[0])

    return tests

# The entry point for the program
def main():
    print("Generating tests . . .")#
    simple_tests = list_tests()
    repeated_tests = REPEATED_TESTS

    for algorithm in ALGORITHMS:
        for test in simple_tests:
            generate_test(test, algorithm, 1)
        
        for (test, count) in repeated_tests:
            generate_test(test, algorithm, count)

main()