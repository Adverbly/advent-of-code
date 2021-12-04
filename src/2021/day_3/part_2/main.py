# This is a sample Python script.

# Press Shift+F10 to execute it or replace it with your code.
# Press Double Shift to search everywhere for classes, files, tool windows, actions, and settings.

from data import INPUT

def print_hi(name):
    up_sum = 0
    right_sum = 0
    forward_sum = 0
    aim_sum = 0
    data = INPUT.split("\n")
    for line in data:
        direction, amount = line.split(" ")
        if direction == 'forward':
            forward_sum += int(amount)
            up_sum += int(amount) * aim_sum
        elif direction == 'down':
            aim_sum += int(amount)
        elif direction == 'up':
            aim_sum -= int(amount)
        elif direction == 'left':
            right_sum -= int(amount)
        elif direction == 'right':
            right_sum += int(amount)
    print(up_sum * forward_sum)



# Press the green button in the gutter to run the script.
if __name__ == '__main__':
    print_hi('PyCharm')

# See PyCharm help at https://www.jetbrains.com/help/pycharm/
