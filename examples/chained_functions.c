int h() { return i() + i(); }
int g() { return h() + h(); }
int f() { return g() + g(); }

void print_something() {
  print(1);
  print(2);
}

int main() {
  print_something();
  print(f());
  print_something();
}
