int main() {
    int a[2][3];
    a[0][0] = 11;
    a[0][1] = 22;
    a[0][2] = 33;
    a[1][0] = 1;
    a[1][1] = 2;
    a[1][2] = 3;
    return a[0][0] + a[0][1] + a[0][2] + a[1][0] + a[1][1] + a[1][2];
}
