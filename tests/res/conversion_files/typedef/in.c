typedef int bool_t;

typedef union {
    int x;
    char y;
} au_t;

typedef struct {
    int x;
    char y;
} as_t;

typedef enum {
    ONE,
    TWO
} ae_t;

typedef union u {
    int x;
    char y;
} u_t;

typedef struct s {
    int x;
    char y;
} s_t;

typedef enum e {
    ONE,
    TWO
} e_t;

module mc {
    typedef char bool;
}
