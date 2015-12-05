import os
import subprocess

def get_program_output(prog_name):
    output = subprocess.check_output(prog_name, shell=True)
    output = [x.strip() for x in output.strip().split("\n") if len(x) > 0]
    return output

test_dir = "tests"

errs = []
subdirs = os.listdir(test_dir)
tests_passed = 0
for d in subdirs:
    tests = os.listdir(os.path.join(test_dir, d))
    tests = [t for t in tests if t.endswith(".sc")]
    for t in tests:
        print "Running {0}".format(t)
        # Get the first line of t
        path = os.path.join(test_dir, d, t)
        fd = open(path, "r")
        ln = fd.readline()
        fd.close()

        ln = ln.replace("// ", "").replace(";", "")
        expected_output = ln.strip()

        compiler = "cargo run {0}".format(path)

        # Make sure the compiler gives an error
        if expected_output.startswith("ERROR"):
            output = get_program_output(compiler)
            error_lines = [x for x in output if x.startswith('FAILED')]
            if len(error_lines) == 0:
                errs.append("ERROR at {0}: Expected program to fail, but no"
                            .format(path))
            else:
                error_expected = expected_output.split(' ')[1]
                error_received = error_lines[0].split(' ')[1]
                if error_received != error_expected:
                    errs.append("ERROR at {0}: Expected err: {1} but got {2}"
                                .format(path, error_expected, error_received))
                else:
                    tests_passed += 1
        else:
            # Build our program (do it with os.system() because it blocks until
            # the command has finished. If we don't do this, we might try to build
            # the output of the previous test.
            os.system(compiler)
            os.system("./build.sh")
            # Run it with no buffering on stdout (so we get whatever it prints)
            output = get_program_output("./a.out")

            if expected_output != output[-1]:
                errs.append("ERROR at {0}: Expected {1}. Got {2}"
                            .format(path, expected_output, output[-1]))
            else:
                tests_passed += 1

print "{0} passed".format(tests_passed)
print "There were {0} errors".format(len(errs))
for e in errs:
    print e
