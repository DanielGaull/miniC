typedef struct STest__struct {
    struct {
        int x;
        int y;
    };
    union {
        char c;
        int a;
    };
} STest;
typedef union UTest__union {
    struct {
        int x;
        int y;
    };
    union {
        char c;
        int a;
    };
} UTest;
