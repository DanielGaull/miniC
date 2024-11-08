# miniC

miniC is intented to make some slight improvements upon C. miniC transpiles to C code. miniC does not build as much upon C as C++ does, moreso adding smaller utility functionalities.

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
    BankAccount b;
    b.deposit(5); // b.balance is now 5

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
    BankAccount b;
    BankAccount__deposit(&b, 5);

You can also call functions on a struct pointer. For example, like below:

    BankAccount *b = ...
    b->deposit(5); // Use the arrow when calling functions on a pointer

Fields can be marked as readonly (from outside the struct) like so:

    struct BankAccount {
        readonly int balance;

        // ...
    }

The transpiler will check for attempted writes to readonly fields from outside the struct and fail if they appear

## Struct Constructors
You can define a constructor for a struct by creating a function with no return type called `constructor`. For the `BankAccount` example, this could look like so:

    struct BankAccount {
        readonly int balance;

        constructor(int startingBalance) {
            this->balance = startingBalance;
        }
    }

Then you use the `new` keyword, in syntax alike Java/C#, to create a new struct:

    BankAccount account = new BankAccount(0);

## Indexing
TODO

## Additional Types
miniC's standard library comes with additional features to make working in C easier. For example:

### Booleans
miniC comes with the `bool` type, with `true` and `false` values for it. miniC will raise a warning if a non-boolean value is used in a boolean context. The `bool` struct does not have any additional functionality defined.

### Strings
miniC defines a `string` struct, and string literals in C will be converted into this new `string` struct. The `string` struct defines a constructor taking in a `char *`, so you can easily convert any "vanilla" C string into a miniC `string`. The `string` uses a `char *` internally, so you can call `.toChars()` to get the `char *` value for the string. Keep in mind this *duplicates* the values, so manipulating the char array that comes out will *not* change the original.

The `string` struct also defines a `length` property.
TODO

## Standard Library

### Memory Management
TODO

### The Future
Exceptions? Try/catch blocks?
