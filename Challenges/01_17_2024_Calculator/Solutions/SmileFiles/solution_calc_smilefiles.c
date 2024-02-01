#include <stdio.h>
#include <stdlib.h>

void print()
{
    printf("\nUsage: .\\calc <first_operand> <arithmetic_operation> <second_operand>\nValid Operations: + - '*' / %%\n");
}float valid_input(char * string)
{
    char * end;
    float f;
    f = strtof(string, &end);
    return f;
}
int main(int argc, char * argv[])   // argc is number or arguments, argv is an array of the argument values. The name of the program is an argument too, the first one
{
    if (argc != 4)
    {
        print();
        return -1;
    }
    char * first = argv[1];
    float f = valid_input(argv[1]); // argv[0] = "calc"
    char * operation = argv[2];
    char * second = argv[3];
    float s = valid_input(argv[3]);
    char again = 'n';
    printf("%s %s %s", argv[1], argv[2], argv[3]);
    while (1)
    {
        switch( *operation)
        {
            case '+':
                printf("\n %f + %f = %f\n", f, s, f + s);
                break;
            case '-':
                printf("\n %f - %f = %f\n", f, s, f - s);
                break;
            case '*':
                printf("\n %f * %f = %f\n", f, s, f * s);
                break;
            case '/':
                printf("\n %f / %f = %f\n", f, s, f / s);
                break;
            case '%':
                printf("\n %f %% %f = %f\n", f, s, (int) f % (int) s); // floats cannot %
                break;
            default:
                print();
        }
        printf("Enter another computation? Type 'n' to quit: ");
        scanf(" %c", &again);
        if (again =='n')        // anything but 'n' will keep the program running
        {
            break;
        } else
        {
            printf("Enter the first operator: ");
            scanf("%s", first);
            f = valid_input(first);
            printf("Enter the second operator: ");
            scanf("%s", second);
            s = valid_input(second);
            printf("What operation do you want to perform (+ - * / %%)? ");
            scanf(" %c", operation);
        }
    }
    return 0;
}