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

int fibonacci(int n) {
  if (n <= 1) {
    return 1;
  }
  return fibonacci(n - 2) + fibonacci(n - 1);
}

int main() {
  int i;
  i = 0;

  while (i < 15) {
    println_number(fibonacci(i));
    i = i + 1;
  }
}
