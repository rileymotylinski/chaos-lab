import csv
import matplotlib.pyplot as plt



def get_logistic_points():
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

    return (x_points, y_points)

x_points, y_pionts = get_logistic_points()

plt.plot(x_points,y_pionts,"ro")
plt.show()
