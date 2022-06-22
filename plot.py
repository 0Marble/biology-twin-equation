import matplotlib.pyplot as plt
import csv
import sys

def plot_csv(file_path, color):
    file = open(file_path)
    contents = csv.reader(file)
    x = []
    y = []

    for row in contents:
        x.append(float(row[0]))
        y.append(float(row[1]))

    plt.plot(x, y, c=color)

def plot_method(
    name, 
    numeric_file_path, 
    actual_file_path, 
    difference_file_path
):
    
    plt.subplot(121)
    plt.title('red = actual, blue = ' + name)
    plot_csv(numeric_file_path, 'r')
    plot_csv(actual_file_path, 'b')
    plt.axis([0, 15, 1, 2])

    plt.subplot(122)
    plt.title('Difference in %')
    plot_csv(difference_file_path, 'r')
    plt.axis([0, 15, 0, 2])

dir = sys.argv[1];
name = sys.argv[2];
prefix = sys.argv[3];

plot_method(
    name,
    dir + "/" + prefix + "_" + name + ".csv", 
    dir + "/" + prefix + "_actual.csv",
    dir + "/" + prefix + "_" + name + "_diff.csv"
)
plt.show()