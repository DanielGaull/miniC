module mc {
    void helper();
    void* malloc(usize sz) {
        return malloc(sz);
    }
    enum Color {
        RED,
        GREEN,
        YELLOW
    };
    struct StopLight {
        mc::Color currentColor;
        int timeLeft;
    };
}
