# miniC

miniC is intented to make some slight improvements upon C. miniC transpiles to C code. miniC does not build as much upon C as C++ does, moreso adding smaller utility functionalities.

## Why a Transpiler?
miniC transpiles to C for ease of working with existing codebases. It allows users to enjoy the benefits that miniC provides without forcing others to use it, and gives access to all of the great tooling that already exists for C. Transpiled miniC is still human-readable, and miniC merely exists to reduce boilerplate.

In the future, there are plans to allow for the reverse, where code that has gone from miniC -> C can go back from C -> miniC. The ultimate vision is a world in which a repository can be written entirely in C, and a miniC user can come in, make their changes in miniC, build + push to C, and then later pull changes from the repo and make their edits in miniC. In the far future, there could be tooling to convert code that wasn't even transpiled using miniC to miniC, making code written before miniC itself more readable and easy to work with.

## Struct Functions
miniC allows structs to have functions defined within them and called on them, like this:

    struct BankAccount {
        int balance;

        void deposit(int amount) {
            this->balance += amount;
        }
    }

    // ...
    // Somewhere else, we can call this function:
    BankAccount *b = malloc(sizeof(BankAccount));
    b:deposit(5); // b.balance is now 5

NOTE: As an added feature, miniC will also typedef your structs for you, so you can refer to `BankAccount` by name rather than `struct BankAccount`.

Within functions defined in a struct, you have access to the value it is called on via the `this` value, which is a pointer.

As an example, the above would transpile to something like the below code:

    typedef struct BankAccount_s {
        int balance;
    } BankAccount;

    void BankAccount__deposit(BankAccount *this, int amount) {
        this->balance += amount;
    }

    // ...
    BankAccount *b = malloc(sizeof(BankAccount));
    BankAccount__deposit(b, 5);

You must call struct methods on a pointer to the struct, using a colon.

## Additional Types
miniC's standard library comes with additional features to make working in C easier. For example:

### Booleans
miniC comes with the `bool` type, with `true` and `false` values for it. The `bool` struct does not have any additional functionality defined.

### Strings
miniC defines a `string` struct, and string literals in C will be converted into this new `string` struct. The `string` struct defines a constructor taking in a `char *`, so you can easily convert any "vanilla" C string into a miniC `string`. The `string` uses a `char *` internally, so you can call `.toChars()` to get the `char *` value for the string. Keep in mind this *duplicates* the values, so manipulating the char array that comes out will *not* change the original.

The `string` struct also defines a `length` property.
TODO

## Standard Library

### Memory Management
TODO

### The Future
Exceptions? Try/catch blocks?
