import os
import subprocess

test_dir = "tests"

errs = []
subdirs = os.listdir(test_dir)
for d in subdirs:
    tests = os.listdir(os.path.join(test_dir, d))
    for t in tests:
        print "Building {0}".format(t)
        # Get the first line of t
        path = os.path.join(test_dir, d, t)
        fd = open(path, "r")
        ln = fd.readline()
        fd.close()

        ln = ln.replace("// ", "").replace(";", "")
        val = ln.strip()

        print "Running {0}".format(t)
        
        # Build our program
        os.system("cargo run {0}".format(path))
        os.system("./build.sh")
        #os.system("./build.sh {0}".format(path))
        # Run it with no buffering on stdout (so we get whatever it prints)
        output = subprocess.check_output("./a.out", shell=True)
        output = [x.strip() for x in output.strip().split("\n") if len(x) > 0]

        if val != output[-1]:
            errs.append(path)
            errs.append("ERROR: Expected {0}. Got {1}".format(val, output[-1]))

print "There were {0} errors".format(len(errs))
for e in errs:
    print e
