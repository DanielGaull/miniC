void mod__mc__helper();
void* mod__mc__malloc(usize sz) {
    return malloc(sz);
}

typedef enum Color__enum {
    RED,
    GREEN,
    YELLOW
} mod__mc__Color;
typedef struct StopLight__struct {
    mod__mc__Color currentColor;
    int timeLeft;
} mod__mc__StopLight;

