import sys
if __name__ == '__main__':
    if len(sys.argv) != 2:
        print(f"USAGE: {__file__} file_to_count.")
        exit(-1)
    path = sys.argv[1]
    with open(path) as fin:
        total = sum(list(map(lambda l : int(l), fin.readlines())))
        print(f"File: {path}\n\t\t-- total spikes emitted: [{total}]")