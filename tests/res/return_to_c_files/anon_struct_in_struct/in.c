struct STest {
    struct {
        int x;
        int y;
    };
    union {
        char c;
        int a;
    };
};

union UTest {
    struct {
        int x;
        int y;
    };
    union {
        char c;
        int a;
    };
};
