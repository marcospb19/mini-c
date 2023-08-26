int main() {
  print(f());

  print_some_stuff();
}

int f() { return g() + g(); }

int g() { return h() + h(); }

int h() { return 1; }

void print_some_stuff() {
  print(1);
  print(2);
  print(3);
}
