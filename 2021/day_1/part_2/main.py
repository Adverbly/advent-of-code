# This is a sample Python script.

# Press Shift+F10 to execute it or replace it with your code.
# Press Double Shift to search everywhere for classes, files, tool windows, actions, and settings.

from data import INPUT

def print_hi(name):
    # Use a breakpoint in the code line below to debug your script.
    data = INPUT.split("\n")
    head = data[:-2]
    middle = data[1:-1]
    tail = data[2:]
    sums = [int(head[i]) + int(middle[i]) + int(tail[i]) for i in range(len(head))]


    prev = sums[:-1]
    next = sums[1:]
    count = 0
    for i in range(len(prev)):
        if int(next[i]) > int(prev[i]):
            count += 1
    print(len(prev), len(sums), count)



# Press the green button in the gutter to run the script.
if __name__ == '__main__':
    print_hi('PyCharm')

# See PyCharm help at https://www.jetbrains.com/help/pycharm/
