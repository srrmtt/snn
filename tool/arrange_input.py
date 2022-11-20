

if __name__ == '__main__':
    with open('./data/inputSpikes.txt') as f:
        lines = f.readlines()
        n_new_lines = len(lines[0])

        new_lines = [""] * n_new_lines

        for line in lines:
            for (i, c) in enumerate(line):
                new_lines[i] += c
        
        with open('./data/inputs.txt', mode='w') as out:
            for line in new_lines:
                out.write(line + "\n")

