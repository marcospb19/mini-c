// factorial(8) == 40320

void print_number_rec(int number) {
  if (number == 0) {
    return;
  }

  print_number_rec(number / 10);
  print(number % 10);
}

void print_number(int number) {
  if (number == 0) {
    print(0);
  } else {
    print_number_rec(number);
  }
}

void println_number(int number) {
  print_number(number);
  println();
}

//

int factorial(int n) {
  if (n <= 1) {
    return 1;
  }
  return n * factorial(n - 1);
}

int main() { println_number(factorial(8)); }
