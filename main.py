import os
import sys
import argparse
from os import environ
from lohash import StringLoHash 

if not os.environ.get('PYTHONHASHSEED'):
    os.environ['PYTHONHASHSEED'] = '1234'
    os.execv(sys.executable, ['python3'] + sys.argv)
hashseed = os.getenv('PYTHONHASHSEED')

if __name__ == '__main__':
    print("Hash Seed:  {}".format(hashseed))
    parser=argparse.ArgumentParser()
    parser.add_argument("--mode", help="Lohash mode, currently supporting test, interactive, and production.")
    parser.add_argument("--input", help="CSV File to process.")
    parser.add_argument("--output", help="CSV File to output if needed.")
    parser.add_argument("--threshold", help="Threshold for comparison between groups 0<threshold<1")
    parser.add_argument("--k", help="Threshold for comparison between groups 0<k<4")
    parser.add_argument("--writetype", default="default", help="A selection for how the output is generated")
    parser.add_argument("--verbose", default=False , help="Print steps during generation")
    args=parser.parse_args()

    os.environ['PYTHONHASHSEED'] = '1234'
    match args.mode:
        case "test":
            print("Running Test Workload...")
            print("Hash Test: ",hash("TEST OBJECT"))
            lsh = StringLoHash("./test/test.csv", "./test/test.csv", saveHash=True, threshold=float(args.threshold), k=int(args.k))
            lsh.start()
            lsh.display()
            print("Done...")

            if args.output:
                print("Writing Output...")
                lsh.writeGroups()


        case "interactive":
            print("Running Interactive Mode...")

        case _:
            print("Running Prod Workload...")
            lsh = StringLoHash(args.input, args.output, saveHash=True,  threshold=float(args.threshold), k=int(args.k), writeType= args.writetype, verbose=args.verbose)
            lsh.start()
            # lsh.display()
            if args.output and (args.writetype != "csv"):
                print("Writing Output...")
                lsh.writeGroups()

            print("Done...")