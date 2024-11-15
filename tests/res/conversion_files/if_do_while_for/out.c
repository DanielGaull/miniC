void main() {
    int x = 0;
    if (x==5) {
        x += 1;
    }else if (x==6) {
        x += 2;
    }else if (x==7) {
        x += 3;
    }else {
        x += 4;
    }
    int i;
    for (i = 0;i<7;i++) {
        x += 1;
        continue;
    }
    while (i<10) {
        i += 1;
        break;
    }
    do {
        i += 1;
    } while (i<100);
    return;
}

