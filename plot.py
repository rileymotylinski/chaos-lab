import csv
import matplotlib.pyplot as plt



def logistic_points():
    x_points = []
    y_points = []
    with open('test.csv', newline='') as csvfile:

        spamreader = csv.reader(csvfile, delimiter=',')
        for row in spamreader:
            # hack-y solution for now, just skips headers. not really planning to display w/ python long term
            if spamreader.line_num == 1:
                continue

            # skipping r column
            for column in row[1:]:
                x_points.append(float(row[0]))
                y_points.append(float(column))
    plt.plot(x_points,y_points,"ro")
    plt.show()

def lorenz_points():
    x_points = []
    y_points = []
    z_points = []
    with open('lorenz.csv', newline='') as csvfile:

        spamreader = csv.reader(csvfile, delimiter=',')
        for row in spamreader:
            # hack-y solution for skiping headers. not really planning to display w/ python long term
            if spamreader.line_num == 1:
                continue
            # definitely prettier way to do this
            x_points.append(float(row[0]))
            y_points.append(float(row[1]))
            z_points.append(float(row[2]))

    

    fig = plt.figure()
    ax = fig.add_subplot(projection='3d')
    ax.scatter(x_points,y_points,z_points, marker="o")

    ax.set_xlabel('X Label')
    ax.set_ylabel('Y Label')
    ax.set_zlabel('Z Label')

    plt.show()

lorenz_points()

  



