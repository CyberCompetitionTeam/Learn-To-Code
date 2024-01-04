def main():
    string = ""
    for i in range(1, 101):
        if i % 3 == 0:
            string += "Fizz"
        if i % 5 == 0:
            string += "Buzz"
        if not i%3 == 0 and not i%5 == 0:
            string += str(i)
        string += " "
    print(string)
if __name__ == "__main__":
    main()