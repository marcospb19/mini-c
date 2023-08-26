bool truth() { return true; }

int main() {
  int x;

  if (truth()) {
    x = 100;
  } else {
    x = 50;
  }

  print(x);
}
