#include <iostream>

using namespace std;

bool check_mo3(int num){
  if (num % 3 == 0){
    return true;
  } else {
    return false;
  }
}

bool check_mo5(int num){
  if (num % 5 == 0){
    return true;
  } else {
    return false;
  }
}

void check(int num){
 if ( check_mo3(num) && check_mo5(num) ){
   cout << "FizzBuzz" << endl;
 } else if ( check_mo3(num) ){
   cout << "Fizz" << endl;
 } else if ( check_mo5(num) ){
   cout << "Buzz" << endl;
 } else {
   cout << num << endl;
 }
}

int main(){
  for (int i = 1; i <= 100; i++){
    check(i);
  }

  return 0;
}
