
// Allow certain warnings to be ignored for compatibility with C-style code
#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]

// Declare external C functions for use in Rust
extern "C" {
    // Prints formatted output to the console
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    // Allocates memory on the heap
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    // Frees previously allocated memory
    fn free(_: *mut libc::c_void);
    // Terminates the program with an exit code
    fn exit(_: libc::c_int) -> !;
    // Sets a block of memory to a specified value
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    // Compares two blocks of memory
    fn memcmp(_: *const libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> libc::c_int;
    // Closes a file descriptor
    fn close(__fd: libc::c_int) -> libc::c_int;
    // Reads data from a file descriptor into a buffer
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    // Opens a file and returns a file descriptor
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
}

// Define type aliases for C-compatible types
pub type size_t = libc::c_ulong; // Unsigned long for sizes
pub type ssize_t = libc::c_long; // Signed long for sizes or error codes

// Define Token type for lexical analysis (e.g., operators, keywords)
pub type Token = libc::c_uint;
// Constants for token types, used in parsing source code
pub const Brak: Token = 164; // '['
pub const Dec: Token = 163; // '--'
pub const Inc: Token = 162; // '++'
pub const Mod: Token = 161; // '%'
pub const Div: Token = 160; // '/'
pub const Mul: Token = 159; // '*'
pub const Sub: Token = 158; // '-'
pub const Add: Token = 157; // '+'
pub const Shr: Token = 156; // '>>'
pub const Shl: Token = 155; // '<<'
pub const Ge: Token = 154; // '>='
pub const Le: Token = 153; // '<='
pub const Gt: Token = 152; // '>'
pub const Lt: Token = 151; // '<'
pub const Ne: Token = 150; // '!='
pub const Eq: Token = 149; // '=='
pub const And: Token = 148; // '&'
pub const Xor: Token = 147; // '^'
pub const Or: Token = 146; // '|'
pub const Lan: Token = 145; // '&&'
pub const Lor: Token = 144; // '||'
pub const Cond: Token = 143; // '?'
pub const Assign: Token = 142; // '='
pub const While: Token = 141; // 'while'
pub const Sizeof: Token = 140; // 'sizeof'
pub const Return: Token = 139; // 'return'
pub const Int: Token = 138; // 'int'
pub const If: Token = 137; // 'if'
pub const Enum: Token = 136; // 'enum'
pub const Else: Token = 135; // 'else'
pub const Char: Token = 134; // 'char'
pub const Id: Token = 133; // Identifier (variable/function name)
pub const Loc: Token = 132; // Local variable
pub const Glo: Token = 131; // Global variable
pub const Sys: Token = 130; // System call/function
pub const Fun: Token = 129; // Function
pub const Num: Token = 128; // Numeric literal

// Define OpCode type for intermediate code instructions
pub type OpCode = libc::c_uint;
// Constants for operation codes used in code generation
pub const EXIT: OpCode = 38; // Exit program
pub const MCMP: OpCode = 37; // Memory compare
pub const MSET: OpCode = 36; // Memory set
pub const FREE: OpCode = 35; // Free memory
pub const MALC: OpCode = 34; // Allocate memory
pub const PRTF: OpCode = 33; // Print formatted
pub const CLOS: OpCode = 32; // Close file
pub const READ: OpCode = 31; // Read file
pub const OPEN: OpCode = 30; // Open file
pub const MOD: OpCode = 29; // Modulo
pub const DIV: OpCode = 28; // Division
pub const MUL: OpCode = 27; // Multiplication
pub const SUB: OpCode = 26; // Subtraction
pub const ADD: OpCode = 25; // Addition
pub const SHR: OpCode = 24; // Shift right
pub const SHL: OpCode = 23; // Shift left
pub const GE: OpCode = 22; // Greater or equal
pub const LE: OpCode = 21; // Less or equal
pub const GT: OpCode = 20; // Greater than
pub const LT: OpCode = 19; // Less than
pub const NE: OpCode = 18; // Not equal
pub const EQ: OpCode = 17; // Equal
pub const AND: OpCode = 16; // Bitwise AND
pub const XOR: OpCode = 15; // Bitwise XOR
pub const OR: OpCode = 14; // Bitwise OR
pub const PSH: OpCode = 13; // Push to stack
pub const SC: OpCode = 12; // Store char
pub const SI: OpCode = 11; // Store int
pub const LC: OpCode = 10; // Load char
pub const LI: OpCode = 9; // Load int
pub const LEV: OpCode = 8; // Leave function
pub const ADJ: OpCode = 7; // Adjust stack
pub const ENT: OpCode = 6; // Enter function
pub const BNZ: OpCode = 5; // Branch if not zero
pub const BZ: OpCode = 4; // Branch if zero
pub const JSR: OpCode = 3; // Jump to subroutine
pub const JMP: OpCode = 2; // Jump
pub const IMM: OpCode = 1; // Immediate value
pub const LEA: OpCode = 0; // Load effective address

// Define TypeKind for variable types
pub type TypeKind = libc::c_uint;
pub const CHAR: TypeKind = 0; // Character type
pub const INT: TypeKind = 1; // Integer type
pub const PTR: TypeKind = 2; // Pointer type

// Define IdField for identifier attributes in symbol table
pub type IdField = libc::c_uint;
pub const Tk: IdField = 0; // Token type
pub const Hash: IdField = 1; // Hash value for identifier
pub const Name: IdField = 2; // Name of identifier
pub const Class: IdField = 3; // Class (e.g., global, local)
pub const Type: IdField = 4; // Type (e.g., int, char)
pub const Val: IdField = 5; // Value or address
pub const HClass: IdField = 6; // Saved class for locals
pub const HType: IdField = 7; // Saved type for locals
pub const HVal: IdField = 8; // Saved value for locals
pub const Idsz: IdField = 9; // Size of identifier structure

// Global variables for compiler state
#[no_mangle]
pub static mut p: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char; // Current position in source code
#[no_mangle]
pub static mut lp: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char; // Last position for line printing
#[no_mangle]
pub static mut data: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char; // Data segment for globals
#[no_mangle]
pub static mut e: *mut libc::c_longlong = 0 as *const libc::c_longlong as *mut libc::c_longlong; // Code segment for generated code
#[no_mangle]
pub static mut le: *mut libc::c_longlong = 0 as *const libc::c_longlong as *mut libc::c_longlong; // Last emitted code position
#[no_mangle]
pub static mut id: *mut libc::c_longlong = 0 as *const libc::c_longlong as *mut libc::c_longlong; // Current identifier in symbol table
#[no_mangle]
pub static mut sym: *mut libc::c_longlong = 0 as *const libc::c_longlong as *mut libc::c_longlong; // Symbol table
#[no_mangle]
pub static mut tk: libc::c_longlong = 0; // Current token
#[no_mangle]
pub static mut ival: libc::c_longlong = 0; // Value of current numeric literal
#[no_mangle]
pub static mut ty: libc::c_longlong = 0; // Type of current expression
#[no_mangle]
pub static mut loc: libc::c_longlong = 0; // Local variable offset
#[no_mangle]
pub static mut line: libc::c_longlong = 0; // Current line number
#[no_mangle]
pub static mut src: libc::c_longlong = 0; // Flag for source output
#[no_mangle]
pub static mut debug: libc::c_longlong = 0; // Flag for debug output
#[no_mangle]

// Defines a function to tokenize input source code, accessible from C code
pub unsafe extern "C" fn next() {
    // Initialize a pointer to track the start of identifiers or strings
    let mut pp: *mut libc::c_char = 0 as *mut libc::c_char;
    
    // Loop through the source code until no more tokens or end of input
    loop {
        // Get the current character as a token and convert to a long integer
        tk = *p as libc::c_longlong;
        // Exit loop if end of input (null character)
        if !(tk != 0) {
            break;
        }
        // Move to the next character in the source code
        p = p.offset(1);

        // Handle newline character
        if tk == '\n' as i32 as libc::c_longlong {
            // If source output is enabled, print the current line and generated code
            if src != 0 {
                // Print line number and source code up to the current position
                printf(b"%d: %.*s\0" as *const u8 as *const libc::c_char, line, p.offset_from(lp) as libc::c_long, lp);
                lp = p; // Update last printed position
                // Print generated opcodes from last emitted to current
                while le < e {
                    le = le.offset(1);
                    // Print opcode name (e.g., LEA, IMM) from a fixed string table
                    printf(
                        b"%8.4s\0" as *const u8 as *const libc::c_char,
                        &*(b"LEA ,IMM ,JMP ,JSR ,BZ  ,BNZ ,ENT ,ADJ ,LEV ,LI  ,LC  ,SI  ,SC  ,PSH ,OR  ,XOR ,AND ,EQ  ,NE  ,LT  ,GT  ,LE  ,GE  ,SHL ,SHR ,ADD ,SUB ,MUL ,DIV ,MOD ,OPEN,READ,CLOS,PRTF,MALC,FREE,MSET,MCMP,EXIT,\0"
                            as *const u8 as *const libc::c_char).offset((*le * 5 as libc::c_int as libc::c_longlong) as isize) as *const libc::c_char,
                    );
                    // If opcode requires an operand (e.g., LEA, IMM), print it
                    if *le <= ADJ as libc::c_int as libc::c_longlong {
                        le = le.offset(1);
                        printf(b" %d\n\0" as *const u8 as *const libc::c_char, *le);
                    } else {
                        printf(b"\n\0" as *const u8 as *const libc::c_char);
                    }
                }
            }
            line += 1; // Increment line number
        }
        // Handle preprocessor directives (lines starting with #)
        else if tk == '#' as i32 as libc::c_longlong {
            // Skip until end of line or end of input
            while *p as libc::c_int != 0 as libc::c_int && *p as libc::c_int != '\n' as i32 {
                p = p.offset(1);
            }
        }
        // Handle identifiers (letters, underscores, followed by alphanumerics or underscores)
        else if tk >= 'a' as i32 as libc::c_longlong && tk <= 'z' as i32 as libc::c_longlong
            || tk >= 'A' as i32 as libc::c_longlong && tk <= 'Z' as i32 as libc::c_longlong
            || tk == '_' as i32 as libc::c_longlong
        {
            pp = p.offset(-(1 as libc::c_int as isize)); // Mark start of identifier
            // Compute a hash while scanning identifier characters
            while *p as libc::c_int >= 'a' as i32 && *p as libc::c_int <= 'z' as i32
                || *p as libc::c_int >= 'A' as i32 && *p as libc::c_int <= 'Z' as i32
                || *p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '9' as i32
                || *p as libc::c_int == '_' as i32
            {
                let fresh0 = p;
                p = p.offset(1);
                tk = tk * 147 as libc::c_int as libc::c_longlong + *fresh0 as libc::c_longlong;
            }
            // Finalize hash with length of identifier
            tk = (tk << 6 as libc::c_int) + p.offset_from(pp) as libc::c_long as libc::c_longlong;
            id = sym; // Start at symbol table beginning
            // Search symbol table for existing identifier
            while *id.offset(Tk as libc::c_int as isize) != 0 {
                if tk == *id.offset(Hash as libc::c_int as isize)
                    && memcmp(
                        *id.offset(Name as libc::c_int as isize) as *mut libc::c_char as *const libc::c_void,
                        pp as *const libc::c_void,
                        p.offset_from(pp) as libc::c_long as libc::c_ulong,
                    ) == 0
                {
                    tk = *id.offset(Tk as libc::c_int as isize); // Found, use existing token
                    return;
                }
                id = id.offset(Idsz as libc::c_int as isize); // Move to next symbol
            }
            // New identifier, add to symbol table
            *id.offset(Name as libc::c_int as isize) = pp as libc::c_longlong;
            *id.offset(Hash as libc::c_int as isize) = tk;
            let ref mut fresh1 = *id.offset(Tk as libc::c_int as isize);
            *fresh1 = Id as libc::c_int as libc::c_longlong;
            tk = *fresh1;
            return;
        }
        // Handle numeric literals (decimal, hexadecimal, octal)
        else if tk >= '0' as i32 as libc::c_longlong && tk <= '9' as i32 as libc::c_longlong {
            ival = tk - '0' as i32 as libc::c_longlong; // Start with first digit
            // Decimal number (non-zero leading digit)
            if ival != 0 {
                while *p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '9' as i32 {
                    let fresh2 = p;
                    p = p.offset(1);
                    ival = ival * 10 as libc::c_int as libc::c_longlong + *fresh2 as libc::c_longlong - '0' as i32 as libc::c_longlong;
                }
            }
            // Hexadecimal number (starts with 0x or 0X)
            else if *p as libc::c_int == 'x' as i32 || *p as libc::c_int == 'X' as i32 {
                loop {
                    p = p.offset(1);
                    tk = *p as libc::c_longlong;
                    if !(tk != 0
                        && (tk >= '0' as i32 as libc::c_longlong && tk <= '9' as i32 as libc::c_longlong
                            || tk >= 'a' as i32 as libc::c_longlong && tk <= 'f' as i32 as libc::c_longlong
                            || tk >= 'A' as i32 as libc::c_longlong && tk <= 'F' as i32 as libc::c_longlong))
                    {
                        break;
                    }
                    ival = ival * 16 as libc::c_int as libc::c_longlong
                        + (tk & 15 as libc::c_int as libc::c_longlong)
                        + (if tk >= 'A' as i32 as libc::c_longlong { 9 as libc::c_int } else { 0 as libc::c_int }) as libc::c_longlong;
                }
            }
            // Octal number (starts with 0)
            else {
                while *p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '7' as i32 {
                    let fresh3 = p;
                    p = p.offset(1);
                    ival = ival * 8 as libc::c_int as libc::c_longlong + *fresh3 as libc::c_longlong - '0' as i32 as libc::c_longlong;
                }
            }
            tk = Num as libc::c_int as libc::c_longlong; // Set token as number
            return;
        }
        // Handle division operator or comments
        else if tk == '/' as i32 as libc::c_longlong {
            // Single-line comment (//)
            if *p as libc::c_int == '/' as i32 {
                p = p.offset(1);
                while *p as libc::c_int != 0 as libc::c_int && *p as libc::c_int != '\n' as i32 {
                    p = p.offset(1);
                }
            } else {
                tk = Div as libc::c_int as libc::c_longlong; // Division operator
                return;
            }
        }
        // Handle character or string literals
        else if tk == '\'' as i32 as libc::c_longlong || tk == '"' as i32 as libc::c_longlong {
            pp = data; // Point to data segment for strings
            // Process characters until closing quote
            while *p as libc::c_int != 0 as libc::c_int && *p as libc::c_longlong != tk {
                let fresh4 = p;
                p = p.offset(1);
                ival = *fresh4 as libc::c_longlong;
                // Handle escape sequences (e.g., \n)
                if ival == '\\' as i32 as libc::c_longlong {
                    let fresh5 = p;
                    p = p.offset(1);
                    ival = *fresh5 as libc::c_longlong;
                    if ival == 'n' as i32 as libc::c_longlong {
                        ival = '\n' as i32 as libc::c_longlong;
                    }
                }
                // Store characters for string literals
                if tk == '"' as i32 as libc::c_longlong {
                    let fresh6 = data;
                    data = data.offset(1);
                    *fresh6 = ival as libc::c_char;
                }
            }
            p = p.offset(1); // Skip closing quote
            // For strings, set ival to data segment address
            if tk == '"' as i32 as libc::c_longlong {
                ival = pp as libc::c_longlong;
            } else {
                tk = Num as libc::c_int as libc::c_longlong; // Character treated as number
            }
            return;
        }
        // Handle assignment or equality operators
        else if tk == '=' as i32 as libc::c_longlong {
            if *p as libc::c_int == '=' as i32 {
                p = p.offset(1);
                tk = Eq as libc::c_int as libc::c_longlong; // Equality (==)
            } else {
                tk = Assign as libc::c_int as libc::c_longlong; // Assignment (=)
            }
            return;
        }
        // Handle addition or increment operators
        else if tk == '+' as i32 as libc::c_longlong {
            if *p as libc::c_int == '+' as i32 {
                p = p.offset(1);
                tk = Inc as libc::c_int as libc::c_longlong; // Increment (++)
            } else {
                tk = Add as libc::c_int as libc::c_longlong; // Addition (+)
            }
            return;
        }
        // Handle subtraction or decrement operators
        else if tk == '-' as i32 as libc::c_longlong {
            if *p as libc::c_int == '-' as i32 {
                p = p.offset(1);
                tk = Dec as libc::c_int as libc::c_longlong; // Decrement (--)
            } else {
                tk = Sub as libc::c_int as libc::c_longlong; // Subtraction (-)
            }
            return;
        }
        // Handle not equal operator
        else if tk == '!' as i32 as libc::c_longlong {
            if *p as libc::c_int == '=' as i32 {
                p = p.offset(1);
                tk = Ne as libc::c_int as libc::c_longlong; // Not equal (!=)
            }
            return;
        }
        // Handle less than, less than or equal, or left shift operators
        else if tk == '<' as i32 as libc::c_longlong {
            if *p as libc::c_int == '=' as i32 {
                p = p.offset(1);
                tk = Le as libc::c_int as libc::c_longlong; // Less or equal (<=)
            } else if *p as libc::c_int == '<' as i32 {
                p = p.offset(1);
                tk = Shl as libc::c_int as libc::c_longlong; // Left shift (<<)
            } else {
                tk = Lt as libc::c_int as libc::c_longlong; // Less than (<)
            }
            return;
        }
        // Handle greater than, greater than or equal, or right shift operators
        else if tk == '>' as i32 as libc::c_longlong {
            if *p as libc::c_int == '=' as i32 {
                p = p.offset(1);
                tk = Ge as libc::c_int as libc::c_longlong; // Greater or equal (>=)
            } else if *p as libc::c_int == '>' as i32 {
                p = p.offset(1);
                tk = Shr as libc::c_int as libc::c_longlong; // Right shift (>>)
            } else {
                tk = Gt as libc::c_int as libc::c_longlong; // Greater than (>)
            }
            return;
        }
        // Handle bitwise or logical OR operators
        else if tk == '|' as i32 as libc::c_longlong {
            if *p as libc::c_int == '|' as i32 {
                p = p.offset(1);
                tk = Lor as libc::c_int as libc::c_longlong; // Logical OR (||)
            } else {
                tk = Or as libc::c_int as libc::c_longlong; // Bitwise OR (|)
            }
            return;
        }
        // Handle bitwise or logical AND operators
        else if tk == '&' as i32 as libc::c_longlong {
            if *p as libc::c_int == '&' as i32 {
                p = p.offset(1);
                tk = Lan as libc::c_int as libc::c_longlong; // Logical AND (&&)
            } else {
                tk = And as libc::c_int as libc::c_longlong; // Bitwise AND (&)
            }
            return;
        }
        // Handle bitwise XOR operator
        else if tk == '^' as i32 as libc::c_longlong {
            tk = Xor as libc::c_int as libc::c_longlong;
            return;
        }
        // Handle modulo operator
        else if tk == '%' as i32 as libc::c_longlong {
            tk = Mod as libc::c_int as libc::c_longlong;
            return;
        }
        // Handle multiplication operator
        else if tk == '*' as i32 as libc::c_longlong {
            tk = Mul as libc::c_int as libc::c_longlong;
            return;
        }
        // Handle array subscript operator
        else if tk == '[' as i32 as libc::c_longlong {
            tk = Brak as libc::c_int as libc::c_longlong;
            return;
        }
        // Handle conditional (ternary) operator
        else if tk == '?' as i32 as libc::c_longlong {
            tk = Cond as libc::c_int as libc::c_longlong;
            return;
        }
        // Handle single-character tokens (e.g., ~, ;, {, }, (, ), ], ,, :)
        else if tk == '~' as i32 as libc::c_longlong
            || tk == ';' as i32 as libc::c_longlong
            || tk == '{' as i32 as libc::c_longlong
            || tk == '}' as i32 as libc::c_longlong
            || tk == '(' as i32 as libc::c_longlong
            || tk == ')' as i32 as libc::c_longlong
            || tk == ']' as i32 as libc::c_longlong
            || tk == ',' as i32 as libc::c_longlong
            || tk == ':' as i32 as libc::c_longlong
        {
            return;
        }
    }
}

// Defines a function to parse and generate code for expressions, accessible from C code
#[no_mangle]
pub unsafe extern "C" fn expr(mut lev: libc::c_longlong) {
    // Temporary variable for storing type or token values
    let mut t: libc::c_longlong = 0;
    // Pointer for tracking identifier or jump addresses
    let mut d: *mut libc::c_longlong = 0 as *mut libc::c_longlong;

    // Handle unexpected end of input
    if tk == 0 {
        printf(b"%d: unexpected eof in expression\n\0" as *const u8 as *const libc::c_char, line);
        exit(-(1 as libc::c_int));
    }
    // Handle numeric literals
    else if tk == Num as libc::c_int as libc::c_longlong {
        e = e.offset(1); *e = IMM as libc::c_int as libc::c_longlong; // Emit immediate value opcode
        e = e.offset(1); *e = ival; // Store the numeric value
        next(); // Move to next token
        ty = INT as libc::c_int as libc::c_longlong; // Set type to integer
    }
    // Handle string literals
    else if tk == '"' as i32 as libc::c_longlong {
        e = e.offset(1); *e = IMM as libc::c_int as libc::c_longlong; // Emit immediate value opcode
        e = e.offset(1); *e = ival; // Store string address
        next();
        // Handle concatenated strings
        while tk == '"' as i32 as libc::c_longlong {
            next();
        }
        // Align data segment to next longlong boundary
        data = ((data as libc::c_longlong as libc::c_ulonglong)
            .wrapping_add(::core::mem::size_of::<libc::c_longlong>() as libc::c_ulong as libc::c_ulonglong)
            & (::core::mem::size_of::<libc::c_longlong>() as libc::c_ulong).wrapping_neg() as libc::c_ulonglong) as *mut libc::c_char;
        ty = PTR as libc::c_int as libc::c_longlong; // Set type to pointer
    }
    // Handle sizeof operator
    else if tk == Sizeof as libc::c_int as libc::c_longlong {
        next();
        if tk == '(' as i32 as libc::c_longlong {
            next();
        } else {
            printf(b"%d: open paren expected in sizeof\n\0" as *const u8 as *const libc::c_char, line);
            exit(-(1 as libc::c_int));
        }
        ty = INT as libc::c_int as libc::c_longlong; // Default type
        if tk == Int as libc::c_int as libc::c_longlong {
            next();
        } else if tk == Char as libc::c_int as libc::c_longlong {
            next();
            ty = CHAR as libc::c_int as libc::c_longlong;
        }
        // Handle pointer types (e.g., int*, char**)
        while tk == Mul as libc::c_int as libc::c_longlong {
            next();
            ty = ty + PTR as libc::c_int as libc::c_longlong;
        }
        if tk == ')' as i32 as libc::c_longlong {
            next();
        } else {
            printf(b"%d: close paren expected in sizeof\n\0" as *const u8 as *const libc::c_char, line);
            exit(-(1 as libc::c_int));
        }
        e = e.offset(1); *e = IMM as libc::c_int as libc::c_longlong; // Emit immediate value
        e = e.offset(1); *e = (if ty == CHAR as libc::c_int as libc::c_longlong {
            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
        } else {
            ::core::mem::size_of::<libc::c_longlong>() as libc::c_ulong
        }) as libc::c_longlong; // Emit size
        ty = INT as libc::c_int as libc::c_longlong; // Result is integer
    }
    // Handle identifiers (variables, functions, constants)
    else if tk == Id as libc::c_int as libc::c_longlong {
        d = id; // Save identifier pointer
        next();
        // Function call
        if tk == '(' as i32 as libc::c_longlong {
            next();
            t = 0 as libc::c_int as libc::c_longlong; // Argument count
            // Parse arguments
            while tk != ')' as i32 as libc::c_longlong {
                expr(Assign as libc::c_int as libc::c_longlong); // Parse argument expression
                e = e.offset(1); *e = PSH as libc::c_int as libc::c_longlong; // Push argument
                t += 1;
                if tk == ',' as i32 as libc::c_longlong {
                    next();
                }
            }
            next();
            // Handle system call or function call
            if *d.offset(Class as libc::c_int as isize) == Sys as libc::c_int as libc::c_longlong {
                e = e.offset(1); *e = *d.offset(Val as libc::c_int as isize); // Emit syscall
            } else if *d.offset(Class as libc::c_int as isize) == Fun as libc::c_int as libc::c_longlong {
                e = e.offset(1); *e = JSR as libc::c_int as libc::c_longlong; // Emit jump to subroutine
                e = e.offset(1); *e = *d.offset(Val as libc::c_int as isize); // Function address
            } else {
                printf(b"%d: bad function call\n\0" as *const u8 as *const libc::c_char, line);
                exit(-(1 as libc::c_int));
            }
            // Adjust stack for arguments
            if t != 0 {
                e = e.offset(1); *e = ADJ as libc::c_int as libc::c_longlong;
                e = e.offset(1); *e = t;
            }
            ty = *d.offset(Type as libc::c_int as isize); // Set return type
        }
        // Numeric constant
        else if *d.offset(Class as libc::c_int as isize) == Num as libc::c_int as libc::c_longlong {
            e = e.offset(1); *e = IMM as libc::c_int as libc::c_longlong;
            e = e.offset(1); *e = *d.offset(Val as libc::c_int as isize);
            ty = INT as libc::c_int as libc::c_longlong;
        }
        // Variable (local or global)
        else {
            if *d.offset(Class as libc::c_int as isize) == Loc as libc::c_int as libc::c_longlong {
                e = e.offset(1); *e = LEA as libc::c_int as libc::c_longlong; // Load effective address
                e = e.offset(1); *e = loc - *d.offset(Val as libc::c_int as isize); // Local offset
            } else if *d.offset(Class as libc::c_int as isize) == Glo as libc::c_int as libc::c_longlong {
                e = e.offset(1); *e = IMM as libc::c_int as libc::c_longlong; // Immediate address
                e = e.offset(1); *e = *d.offset(Val as libc::c_int as isize); // Global address
            } else {
                printf(b"%d: undefined variable\n\0" as *const u8 as *const libc::c_char, line);
                exit(-(1 as libc::c_int));
            }
            ty = *d.offset(Type as libc::c_int as isize); // Set variable type
            e = e.offset(1); *e = (if ty == CHAR as libc::c_int as libc::c_longlong { LC as libc::c_int } else { LI as libc::c_int }) as libc::c_longlong; // Load char or int
        }
    }
    // Handle type casts or parenthesized expressions
    else if tk == '(' as i32 as libc::c_longlong {
        next();
        if tk == Int as libc::c_int as libc::c_longlong || tk == Char as libc::c_int as libc::c_longlong {
            t = (if tk == Int as libc::c_int as libc::c_longlong { INT as libc::c_int } else { CHAR as libc::c_int }) as libc::c_longlong;
            next();
            // Handle pointer types in cast
            while tk == Mul as libc::c_int as libc::c_longlong {
                next();
                t = t + PTR as libc::c_int as libc::c_longlong;
            }
            if tk == ')' as i32 as libc::c_longlong {
                next();
            } else {
                printf(b"%d: bad cast\n\0" as *const u8 as *const libc::c_char, line);
                exit(-(1 as libc::c_int));
            }
            expr(Inc as libc::c_int as libc::c_longlong); // Parse expression
            ty = t; // Set cast type
        } else {
            expr(Assign as libc::c_int as libc::c_longlong); // Parse parenthesized expression
            if tk == ')' as i32 as libc::c_longlong {
                next();
            } else {
                printf(b"%d: close paren expected\n\0" as *const u8 as *const libc::c_char, line);
                exit(-(1 as libc::c_int));
            }
        }
    }
    // Handle dereference operator (*)
    else if tk == Mul as libc::c_int as libc::c_longlong {
        next();
        expr(Inc as libc::c_int as libc::c_longlong); // Parse expression
        if ty > INT as libc::c_int as libc::c_longlong {
            ty = ty - PTR as libc::c_int as libc::c_longlong; // Decrease pointer level
        } else {
            printf(b"%d: bad dereference\n\0" as *const u8 as *const libc::c_char, line);
            exit(-(1 as libc::c_int));
        }
        e = e.offset(1); *e = (if ty == CHAR as libc::c_int as libc::c_longlong { LC as libc::c_int } else { LI as libc::c_int }) as libc::c_longlong; // Load value
    }
    // Handle address-of operator (&)
    else if tk == And as libc::c_int as libc::c_longlong {
        next();
        expr(Inc as libc::c_int as libc::c_longlong); // Parse expression
        if *e == LC as libc::c_int as libc::c_longlong || *e == LI as libc::c_int as libc::c_longlong {
            e = e.offset(-1); // Remove load instruction
        } else {
            printf(b"%d: bad address-of\n\0" as *const u8 as *const libc::c_char, line);
            exit(-(1 as libc::c_int));
        }
        ty = ty + PTR as libc::c_int as libc::c_longlong; // Increase pointer level
    }
    // Handle logical NOT operator (!)
    else if tk == '!' as i32 as libc::c_longlong {
        next();
        expr(Inc as libc::c_int as libc::c_longlong); // Parse expression
        e = e.offset(1); *e = PSH as libc::c_int as libc::c_longlong;
        e = e.offset(1); *e = IMM as libc::c_int as libc::c_longlong;
        e = e.offset(1); *e = 0 as libc::c_int as libc::c_longlong;
        e = e.offset(1); *e = EQ as libc::c_int as libc::c_longlong; // Compare with 0
        ty = INT as libc::c_int as libc::c_longlong;
    }
    // Handle bitwise NOT operator (~)
    else if tk == '~' as i32 as libc::c_longlong {
        next();
        expr(Inc as libc::c_int as libc::c_longlong); // Parse expression
        e = e.offset(1); *e = PSH as libc::c_int as libc::c_longlong;
        e = e.offset(1); *e = IMM as libc::c_int as libc::c_longlong;
        e = e.offset(1); *e = -(1 as libc::c_int) as libc::c_longlong;
        e = e.offset(1); *e = XOR as libc::c_int as libc::c_longlong; // XOR with -1
        ty = INT as libc::c_int as libc::c_longlong;
    }
    // Handle unary plus operator (+)
    else if tk == Add as libc::c_int as libc::c_longlong {
        next();
        expr(Inc as libc::c_int as libc::c_longlong); // Parse expression
        ty = INT as libc::c_int as libc::c_longlong;
    }
    // Handle unary minus operator (-)
    else if tk == Sub as libc::c_int as libc::c_longlong {
        next();
        e = e.offset(1); *e = IMM as libc::c_int as libc::c_longlong;
        if tk == Num as libc::c_int as libc::c_longlong {
            e = e.offset(1); *e = -ival; // Negate constant directly
            next();
        } else {
            e = e.offset(1); *e = -(1 as libc::c_int) as libc::c_longlong;
            e = e.offset(1); *e = PSH as libc::c_int as libc::c_longlong;
            expr(Inc as libc::c_int as libc::c_longlong); // Parse expression
            e = e.offset(1); *e = MUL as libc::c_int as libc::c_longlong; // Multiply by -1
        }
        ty = INT as libc::c_int as libc::c_longlong;
    }
    // Handle pre-increment/decrement operators
    else if tk == Inc as libc::c_int as libc::c_longlong || tk == Dec as libc::c_int as libc::c_longlong {
        t = tk;
        next();
        expr(Inc as libc::c_int as libc::c_longlong); // Parse expression
        if *e == LC as libc::c_int as libc::c_longlong {
            *e = PSH as libc::c_int as libc::c_longlong; e = e.offset(1); *e = LC as libc::c_int as libc::c_longlong;
        } else if *e == LI as libc::c_int as libc::c_longlong {
            *e = PSH as libc::c_int as libc::c_longlong; e = e.offset(1); *e = LI as libc::c_int as libc::c_longlong;
        } else {
            printf(b"%d: bad lvalue in pre-increment\n\0" as *const u8 as *const libc::c_char, line);
            exit(-(1 as libc::c_int));
        }
        e = e.offset(1); *e = PSH as libc::c_int as libc::c_longlong;
        e = e.offset(1); *e = IMM as libc::c_int as libc::c_longlong;
        e = e.offset(1); *e = (if ty > PTR as libc::c_int as libc::c_longlong {
            ::core::mem::size_of::<libc::c_longlong>() as libc::c_ulong
        } else {
            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
        }) as libc::c_longlong; // Increment size
        e = e.offset(1); *e = (if t == Inc as libc::c_int as libc::c_longlong { ADD as libc::c_int } else { SUB as libc::c_int }) as libc::c_longlong;
        e = e.offset(1); *e = (if ty == CHAR as libc::c_int as libc::c_longlong { SC as libc::c_int } else { SI as libc::c_int }) as libc::c_longlong; // Store result
    }
    // Handle binary operators based on precedence
    while tk >= lev {
        t = ty; // Save current type
        // Assignment operator
        if tk == Assign as libc::c_int as libc::c_longlong {
            next();
            if *e == LC as libc::c_int as libc::c_longlong || *e == LI as libc::c_int as libc::c_longlong {
                *e = PSH as libc::c_int as libc::c_longlong; // Push address
            } else {
                printf(b"%d: bad lvalue in assignment\n\0" as *const u8 as *const libc::c_char, line);
                exit(-(1 as libc::c_int));
            }
            expr(Assign as libc::c_int as libc::c_longlong); // Parse right-hand side
            ty = t;
            e = e.offset(1); *e = (if ty == CHAR as libc::c_int as libc::c_longlong { SC as libc::c_int } else { SI as libc::c_int }) as libc::c_longlong; // Store value
        }
        // Ternary conditional operator (?:)
        else if tk == Cond as libc::c_int as libc::c_longlong {
            next();
            e = e.offset(1); *e = BZ as libc::c_int as libc::c_longlong; // Branch if zero
            e = e.offset(1); d = e; // Save jump address
            expr(Assign as libc::c_int as libc::c_longlong); // Parse true expression
            if tk == ':' as i32 as libc::c_longlong {
                next();
            } else {
                printf(b"%d: conditional missing colon\n\0" as *const u8 as *const libc::c_char, line);
                exit(-(1 as libc::c_int));
            }
            *d = e.offset(3 as libc::c_int as isize) as libc::c_longlong; // Patch jump
            e = e.offset(1); *e = JMP as libc::c_int as libc::c_longlong; // Jump to end
            e = e.offset(1); d = e; // Save jump address
            expr(Cond as libc::c_int as libc::c_longlong); // Parse false expression
            *d = e.offset(1 as libc::c_int as isize) as libc::c_longlong; // Patch jump
        }
        // Logical OR (||)
        else if tk == Lor as libc::c_int as libc::c_longlong {
            next();
            e = e.offset(1); *e = BNZ as libc::c_int as libc::c_longlong; // Branch if non-zero
            e = e.offset(1); d = e;
            expr(Lan as libc::c_int as libc::c_longlong); // Parse right-hand side
            *d = e.offset(1 as libc::c_int as isize) as libc::c_longlong; // Patch jump
            ty = INT as libc::c_int as libc::c_longlong;
        }
        // Logical AND (&&)
        else if tk == Lan as libc::c_int as libc::c_longlong {
            next();
            e = e.offset(1); *e = BZ as libc::c_int as libc::c_longlong; // Branch if zero
            e = e.offset(1); d = e;
            expr(Or as libc::c_int as libc::c_longlong); // Parse right-hand side
            *d = e.offset(1 as libc::c_int as isize) as libc::c_longlong; // Patch jump
            ty = INT as libc::c_int as libc::c_longlong;
        }
        // Bitwise OR (|)
        else if tk == Or as libc::c_int as libc::c_longlong {
            next();
            e = e.offset(1); *e = PSH as libc::c_int as libc::c_longlong;
            expr(Xor as libc::c_int as libc::c_longlong);
            e = e.offset(1); *e = OR as libc::c_int as libc::c_longlong;
            ty = INT as libc::c_int as libc::c_longlong;
        }
        // Bitwise XOR (^)
        else if tk == Xor as libc::c_int as libc::c_longlong {
            next();
            e = e.offset(1); *e = PSH as libc::c_int as libc::c_longlong;
            expr(And as libc::c_int as libc::c_longlong);
            e = e.offset(1); *e = XOR as libc::c_int as libc::c_longlong;
            ty = INT as libc::c_int as libc::c_longlong;
        }
        // Bitwise AND (&)
        else if tk == And as libc::c_int as libc::c_longlong {
            next();
            e = e.offset(1); *e = PSH as libc::c_int as libc::c_longlong;
            expr(Eq as libc::c_int as libc::c_longlong);
            e = e.offset(1); *e = AND as libc::c_int as libc::c_longlong;
            ty = INT as libc::c_int as libc::c_longlong;
        }
        // Equality (==)
        else if tk == Eq as libc::c_int as libc::c_longlong {
            next();
            e = e.offset(1); *e = PSH as libc::c_int as libc::c_longlong;
            expr(Lt as libc::c_int as libc::c_longlong);
            e = e.offset(1); *e = EQ as libc::c_int as libc::c_longlong;
            ty = INT as libc::c_int as libc::c_longlong;
        }
        // Inequality (!=)
        else if tk == Ne as libc::c_int as libc::c_longlong {
            next();
            e = e.offset(1); *e = PSH as libc::c_int as libc::c_longlong;
            expr(Lt as libc::c_int as libc::c_longlong);
            e = e.offset(1); *e = NE as libc::c_int as libc::c_longlong;
            ty = INT as libc::c_int as libc::c_longlong;
        }
        // Less than (<)
        else if tk == Lt as libc::c_int as libc::c_longlong {
            next();
            e = e.offset(1); *e = PSH as libc::c_int as libc::c_longlong;
            expr(Shl as libc::c_int as libc::c_longlong);
            e = e.offset(1); *e = LT as libc::c_int as libc::c_longlong;
            ty = INT as libc::c_int as libc::c_longlong;
        }
        // Greater than (>)
        else if tk == Gt as libc::c_int as libc::c_longlong {
            next();
            e = e.offset(1); *e = PSH as libc::c_int as libc::c_longlong;
            expr(Shl as libc::c_int as libc::c_longlong);
            e = e.offset(1); *e = GT as libc::c_int as libc::c_longlong;
            ty = INT as libc::c_int as libc::c_longlong;
        }
        // Less than or equal (<=)
        else if tk == Le as libc::c_int as libc::c_longlong {
            next();
            e = e.offset(1); *e = PSH as libc::c_int as libc::c_longlong;
            expr(Shl as libc::c_int as libc::c_longlong);
            e = e.offset(1); *e = LE as libc::c_int as libc::c_longlong;
            ty = INT as libc::c_int as libc::c_longlong;
        }
        // Greater than or equal (>=)
        else if tk == Ge as libc::c_int as libc::c_longlong {
            next();
            e = e.offset(1); *e = PSH as libc::c_int as libc::c_longlong;
            expr(Shl as libc::c_int as libc::c_longlong);
            e = e.offset(1); *e = GE as libc::c_int as libc::c_longlong;
            ty = INT as libc::c_int as libc::c_longlong;
        }
        // Left shift (<<)
        else if tk == Shl as libc::c_int as libc::c_longlong {
            next();
            e = e.offset(1); *e = PSH as libc::c_int as libc::c_longlong;
            expr(Add as libc::c_int as libc::c_longlong);
            e = e.offset(1); *e = SHL as libc::c_int as libc::c_longlong;
            ty = INT as libc::c_int as libc::c_longlong;
        }
        // Right shift (>>)
        else if tk == Shr as libc::c_int as libc::c_longlong {
            next();
            e = e.offset(1); *e = PSH as libc::c_int as libc::c_longlong;
            expr(Add as libc::c_int as libc::c_longlong);
            e = e.offset(1); *e = SHR as libc::c_int as libc::c_longlong;
            ty = INT as libc::c_int as libc::c_longlong;
        }
        // Addition (+)
        else if tk == Add as libc::c_int as libc::c_longlong {
            next();
            e = e.offset(1); *e = PSH as libc::c_int as libc::c_longlong;
            expr(Mul as libc::c_int as libc::c_longlong);
            ty = t;
            // Handle pointer arithmetic
            if ty > PTR as libc::c_int as libc::c_longlong {
                e = e.offset(1); *e = PSH as libc::c_int as libc::c_longlong;
                e = e.offset(1); *e = IMM as libc::c_int as libc::c_longlong;
                e = e.offset(1); *e = ::core::mem::size_of::<libc::c_longlong>() as libc::c_ulong as libc::c_longlong;
                e = e.offset(1); *e = MUL as libc::c_int as libc::c_longlong;
            }
            e = e.offset(1); *e = ADD as libc::c_int as libc::c_longlong;
        }
        // Subtraction (-)
        else if tk == Sub as libc::c_int as libc::c_longlong {
            next();
            e = e.offset(1); *e = PSH as libc::c_int as libc::c_longlong;
            expr(Mul as libc::c_int as libc::c_longlong);
            if t > PTR as libc::c_int as libc::c_longlong && t == ty {
                // Pointer subtraction
                e = e.offset(1); *e = SUB as libc::c_int as libc::c_longlong;
                e = e.offset(1); *e = PSH as libc::c_int as libc::c_longlong;
                e = e.offset(1); *e = IMM as libc::c_int as libc::c_longlong;
                e = e.offset(1); *e = ::core::mem::size_of::<libc::c_longlong>() as libc::c_ulong as libc::c_longlong;
                e = e.offset(1); *e = DIV as libc::c_int as libc::c_longlong;
                ty = INT as libc::c_int as libc::c_longlong;
            } else {
                ty = t;
                if ty > PTR as libc::c_int as libc::c_longlong {
                    // Pointer arithmetic
                    e = e.offset(1); *e = PSH as libc::c_int as libc::c_longlong;
                    e = e.offset(1); *e = IMM as libc::c_int as libc::c_longlong;
                    e = e.offset(1); *e = ::core::mem::size_of::<libc::c_longlong>() as libc::c_ulong as libc::c_longlong;
                    e = e.offset(1); *e = MUL as libc::c_int as libc::c_longlong;
                    e = e.offset(1); *e = SUB as libc::c_int as libc::c_longlong;
                } else {
                    e = e.offset(1); *e = SUB as libc::c_int as libc::c_longlong;
                }
            }
        }
        // Multiplication (*)
        else if tk == Mul as libc::c_int as libc::c_longlong {
            next();
            e = e.offset(1); *e = PSH as libc::c_int as libc::c_longlong;
            expr(Inc as libc::c_int as libc::c_longlong);
            e = e.offset(1); *e = MUL as libc::c_int as libc::c_longlong;
            ty = INT as libc::c_int as libc::c_longlong;
        }
        // Division (/)
        else if tk == Div as libc::c_int as libc::c_longlong {
            next();
            e = e.offset(1); *e = PSH as libc::c_int as libc::c_longlong;
            expr(Inc as libc::c_int as libc::c_longlong);
            e = e.offset(1); *e = DIV as libc::c_int as libc::c_longlong;
            ty = INT as libc::c_int as libc::c_longlong;
        }
        // Modulo (%)
        else if tk == Mod as libc::c_int as libc::c_longlong {
            next();
            e = e.offset(1); *e = PSH as libc::c_int as libc::c_longlong;
            expr(Inc as libc::c_int as libc::c_longlong);
            e = e.offset(1); *e = MOD as libc::c_int as libc::c_longlong;
            ty = INT as libc::c_int as libc::c_longlong;
        }
        // Post-increment/decrement
        else if tk == Inc as libc::c_int as libc::c_longlong || tk == Dec as libc::c_int as libc::c_longlong {
            if *e == LC as libc::c_int as libc::c_longlong {
                *e = PSH as libc::c_int as libc::c_longlong; e = e.offset(1); *e = LC as libc::c_int as libc::c_longlong;
            } else if *e == LI as libc::c_int as libc::c_longlong {
                *e = PSH as libc::c_int as libc::c_longlong; e = e.offset(1); *e = LI as libc::c_int as libc::c_longlong;
            } else {
                printf(b"%d: bad lvalue in post-increment\n\0" as *const u8 as *const libc::c_char, line);
                exit(-(1 as libc::c_int));
            }
            e = e.offset(1); *e = PSH as libc::c_int as libc::c_longlong;
            e = e.offset(1); *e = IMM as libc::c_int as libc::c_longlong;
            e = e.offset(1); *e = (if ty > PTR as libc::c_int as libc::c_longlong {
                ::core::mem::size_of::<libc::c_longlong>() as libc::c_ulong
            } else {
                ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
            }) as libc::c_longlong;
            e = e.offset(1); *e = (if tk == Inc as libc::c_int as libc::c_longlong { ADD as libc::c_int } else { SUB as libc::c_int }) as libc::c_longlong;
            e = e.offset(1); *e = (if ty == CHAR as libc::c_int as libc::c_longlong { SC as libc::c_int } else { SI as libc::c_int }) as libc::c_longlong;
            // Restore original value
            e = e.offset(1); *e = PSH as libc::c_int as libc::c_longlong;
            e = e.offset(1); *e = IMM as libc::c_int as libc::c_longlong;
            e = e.offset(1); *e = (if ty > PTR as libc::c_int as libc::c_longlong {
                ::core::mem::size_of::<libc::c_longlong>() as libc::c_ulong
            } else {
                ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
            }) as libc::c_longlong;
            e = e.offset(1); *e = (if tk == Inc as libc::c_int as libc::c_longlong { SUB as libc::c_int } else { ADD as libc::c_int }) as libc::c_longlong;
            next();
        }
        // Array subscript
        else if tk == Brak as libc::c_int as libc::c_longlong {
            next();
            e = e.offset(1); *e = PSH as libc::c_int as libc::c_longlong;
            expr(Assign as libc::c_int as libc::c_longlong); // Parse index
            if tk == ']' as i32 as libc::c_longlong {
                next();
            } else {
                printf(b"%d: close bracket expected\n\0" as *const u8 as *const libc::c_char, line);
                exit(-(1 as libc::c_int));
            }
            if t > PTR as libc::c_int as libc::c_longlong {
                e = e.offset(1); *e = PSH as libc::c_int as libc::c_longlong;
                e = e.offset(1); *e = IMM as libc::c_int as libc::c_longlong;
                e = e.offset(1); *e = ::core::mem::size_of::<libc::c_longlong>() as libc::c_ulong as libc::c_longlong;
                e = e.offset(1); *e = MUL as libc::c_int as libc::c_longlong;
            } else if t < PTR as libc::c_int as libc::c_longlong {
                printf(b"%d: pointer type expected\n\0" as *const u8 as *const libc::c_char, line);
                exit(-(1 as libc::c_int));
            }
            e = e.offset(1); *e = ADD as libc::c_int as libc::c_longlong; // Add offset
            ty = t - PTR as libc::c_int as libc::c_longlong; // Set element type
            e = e.offset(1); *e = (if ty == CHAR as libc::c_int as libc::c_longlong { LC as libc::c_int } else { LI as libc::c_int }) as libc::c_longlong; // Load value
        }
        // Unknown token
        else {
            printf(b"%d: compiler error tk=%d\n\0" as *const u8 as *const libc::c_char, line, tk);
            exit(-(1 as libc::c_int));
        }
    }
}
// Defines a function to parse and generate code for statements, accessible from C code
#[no_mangle]
pub unsafe extern "C" fn stmt() {
    // Pointers for tracking jump addresses in control flow
    let mut a: *mut libc::c_longlong = 0 as *mut libc::c_longlong; // Start of loop
    let mut b: *mut libc::c_longlong = 0 as *mut libc::c_longlong; // Jump target

    // Handle 'if' statement
    if tk == If as libc::c_int as libc::c_longlong {
        next(); // Consume 'if'
        // Expect opening parenthesis
        if tk == '(' as i32 as libc::c_longlong {
            next();
        } else {
            printf(b"%d: open paren expected\n\0" as *const u8 as *const libc::c_char, line);
            exit(-(1 as libc::c_int));
        }
        // Parse condition expression
        expr(Assign as libc::c_int as libc::c_longlong);
        // Expect closing parenthesis
        if tk == ')' as i32 as libc::c_longlong {
            next();
        } else {
            printf(b"%d: close paren expected\n\0" as *const u8 as *const libc::c_char, line);
            exit(-(1 as libc::c_int));
        }
        // Emit branch-if-zero for false condition
        e = e.offset(1); *e = BZ as libc::c_int as libc::c_longlong;
        e = e.offset(1); b = e; // Save address for jump patching
        stmt(); // Parse 'then' statement
        // Handle 'else' clause
        if tk == Else as libc::c_int as libc::c_longlong {
            *b = e.offset(3 as libc::c_int as isize) as libc::c_longlong; // Patch BZ to skip else
            e = e.offset(1); *e = JMP as libc::c_int as libc::c_longlong; // Jump to end
            e = e.offset(1); b = e; // Save address for jump patching
            next(); // Consume 'else'
            stmt(); // Parse 'else' statement
        }
        *b = e.offset(1 as libc::c_int as isize) as libc::c_longlong; // Patch jump to end
    }
    // Handle 'while' statement
    else if tk == While as libc::c_int as libc::c_longlong {
        next(); // Consume 'while'
        a = e.offset(1 as libc::c_int as isize); // Mark loop start
        // Expect opening parenthesis
        if tk == '(' as i32 as libc::c_longlong {
            next();
        } else {
            printf(b"%d: open paren expected\n\0" as *const u8 as *const libc::c_char, line);
            exit(-(1 as libc::c_int));
        }
        // Parse loop condition
        expr(Assign as libc::c_int as libc::c_longlong);
        // Expect closing parenthesis
        if tk == ')' as i32 as libc::c_longlong {
            next();
        } else {
            printf(b"%d: close paren expected\n\0" as *const u8 as *const libc::c_char, line);
            exit(-(1 as libc::c_int));
        }
        // Emit branch-if-zero to exit loop
        e = e.offset(1); *e = BZ as libc::c_int as libc::c_longlong;
        e = e.offset(1); b = e; // Save address for jump patching
        stmt(); // Parse loop body
        // Emit jump back to loop start
        e = e.offset(1); *e = JMP as libc::c_int as libc::c_longlong;
        e = e.offset(1); *e = a as libc::c_longlong;
        *b = e.offset(1 as libc::c_int as isize) as libc::c_longlong; // Patch BZ to loop end
    }
    // Handle 'return' statement
    else if tk == Return as libc::c_int as libc::c_longlong {
        next(); // Consume 'return'
        // Parse optional return value
        if tk != ';' as i32 as libc::c_longlong {
            expr(Assign as libc::c_int as libc::c_longlong);
        }
        // Emit leave function instruction
        e = e.offset(1); *e = LEV as libc::c_int as libc::c_longlong;
        // Expect semicolon
        if tk == ';' as i32 as libc::c_longlong {
            next();
        } else {
            printf(b"%d: semicolon expected\n\0" as *const u8 as *const libc::c_char, line);
            exit(-(1 as libc::c_int));
        }
    }
    // Handle compound statement (block)
    else if tk == '{' as i32 as libc::c_longlong {
        next(); // Consume '{'
        // Parse statements until closing brace
        while tk != '}' as i32 as libc::c_longlong {
            stmt();
        }
        next(); // Consume '}'
    }
    // Handle empty statement
    else if tk == ';' as i32 as libc::c_longlong {
        next(); // Consume ';'
    }
    // Handle expression statement
    else {
        expr(Assign as libc::c_int as libc::c_longlong); // Parse expression
        // Expect semicolon
        if tk == ';' as i32 as libc::c_longlong {
            next();
        } else {
            printf(b"%d: semicolon expected\n\0" as *const u8 as *const libc::c_char, line);
            exit(-(1 as libc::c_int));
        }
    }
}
//COMMENTED
// Main function for the C4 compiler/interpreter, accessible from C code
unsafe fn main_0(
    mut argc: libc::c_longlong, // Argument count
    mut argv: *mut *mut libc::c_char, // Argument vector
) -> libc::c_longlong {
    // Initialize variables
    let mut fd: libc::c_longlong = 0; // File descriptor
    let mut bt: libc::c_longlong = 0; // Base type for declarations
    let mut ty_0: libc::c_longlong = 0; // Current type
    let mut poolsz: libc::c_longlong = (256 * 1024) as libc::c_longlong; // Memory pool size (256KB)
    let mut idmain: *mut libc::c_longlong = 0 as *mut libc::c_longlong; // Pointer to main function symbol
    let mut pc: *mut libc::c_longlong = 0 as *mut libc::c_longlong; // Program counter
    let mut sp: *mut libc::c_longlong = 0 as *mut libc::c_longlong; // Stack pointer
    let mut bp: *mut libc::c_longlong = 0 as *mut libc::c_longlong; // Base pointer
    let mut a: libc::c_longlong = 0; // Accumulator
    let mut cycle: libc::c_longlong = 0; // Instruction cycle counter
    let mut i: libc::c_longlong = 0; // General-purpose counter
    let mut t: *mut libc::c_longlong = 0 as *mut libc::c_longlong; // Temporary pointer

    // Process command-line arguments
    argc -= 1; // Skip program name
    argv = argv.offset(1);
    // Check for source output flag (-s)
    if argc > 0 && **argv as libc::c_int == '-' as i32 && *(*argv).offset(1) as libc::c_int == 's' as i32 {
        src = 1; // Enable source output
        argc -= 1;
        argv = argv.offset(1);
    }
    // Check for debug flag (-d)
    if argc > 0 && **argv as libc::c_int == '-' as i32 && *(*argv).offset(1) as libc::c_int == 'd' as i32 {
        debug = 1; // Enable debug output
        argc -= 1;
        argv = argv.offset(1);
    }
    // Ensure a source file is provided
    if argc < 1 {
        printf(b"usage: c4 [-s] [-d] file ...\n\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int) as libc::c_longlong;
    }

    // Open source file
    fd = open(*argv, 0 as libc::c_int) as libc::c_longlong;
    if fd < 0 {
        printf(b"could not open(%s)\n\0" as *const u8 as *const libc::c_char, *argv);
        return -(1 as libc::c_int) as libc::c_longlong;
    }

    // Allocate memory pools
    sym = malloc(poolsz as libc::c_ulong) as *mut libc::c_longlong; // Symbol table
    if sym.is_null() {
        printf(b"could not malloc(%d) symbol area\n\0" as *const u8 as *const libc::c_char, poolsz);
        return -(1 as libc::c_int) as libc::c_longlong;
    }
    e = malloc(poolsz as libc::c_ulong) as *mut libc::c_longlong; // Code segment
    le = e;
    if le.is_null() {
        printf(b"could not malloc(%d) text area\n\0" as *const u8 as *const libc::c_char, poolsz);
        return -(1 as libc::c_int) as libc::c_longlong;
    }
    data = malloc(poolsz as libc::c_ulong) as *mut libc::c_char; // Data segment
    if data.is_null() {
        printf(b"could not malloc(%d) data area\n\0" as *const u8 as *const libc::c_char, poolsz);
        return -(1 as libc::c_int) as libc::c_longlong;
    }
    sp = malloc(poolsz as libc::c_ulong) as *mut libc::c_longlong; // Stack
    if sp.is_null() {
        printf(b"could not malloc(%d) stack area\n\0" as *const u8 as *const libc::c_char, poolsz);
        return -(1 as libc::c_int) as libc::c_longlong;
    }

    // Initialize memory pools
    memset(sym as *mut libc::c_void, 0, poolsz as libc::c_ulong);
    memset(e as *mut libc::c_void, 0, poolsz as libc::c_ulong);
    memset(data as *mut libc::c_void, 0, poolsz as libc::c_ulong);

    // Initialize keyword and system call symbols
    p = b"char else enum if int return sizeof while open read close printf malloc free memset memcmp exit void main\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
    i = Char as libc::c_int as libc::c_longlong;
    while i <= While as libc::c_int as libc::c_longlong {
        next(); // Tokenize keyword
        *id.offset(Tk as libc::c_int as isize) = i; // Assign token type
        i += 1;
    }
    i = OPEN as libc::c_int as libc::c_longlong;
    while i <= EXIT as libc::c_int as libc::c_longlong {
        next(); // Tokenize system call
        *id.offset(Class as libc::c_int as isize) = Sys as libc::c_int as libc::c_longlong; // Mark as system call
        *id.offset(Type as libc::c_int as isize) = INT as libc::c_int as libc::c_longlong; // Set return type
        *id.offset(Val as libc::c_int as isize) = i; // Set syscall number
        i += 1;
    }
    next(); // Tokenize 'void'
    *id.offset(Tk as libc::c_int as isize) = Char as libc::c_int as libc::c_longlong; // Treat as char
    next(); // Tokenize 'main'
    idmain = id; // Save main function symbol

    // Allocate source code buffer
    p = malloc(poolsz as libc::c_ulong) as *mut libc::c_char;
    lp = p;
    if lp.is_null() {
        printf(b"could not malloc(%d) source area\n\0" as *const u8 as *const libc::c_char, poolsz);
        return -(1 as libc::c_int) as libc::c_longlong;
    }

    // Read source file
    i = read(fd as libc::c_int, p as *mut libc::c_void, (poolsz - 1) as size_t) as libc::c_longlong;
    if i <= 0 {
        printf(b"read() returned %d\n\0" as *const u8 as *const libc::c_char, i);
        return -(1 as libc::c_int) as libc::c_longlong;
    }
    *p.offset(i as isize) = 0; // Null-terminate source
    close(fd as libc::c_int);

    // Initialize parser
    line = 1;
    next(); // Get first token

    // Parse global declarations
    while tk != 0 {
        bt = INT as libc::c_int as libc::c_longlong; // Default base type
        if tk == Int as libc::c_int as libc::c_longlong {
            next();
        } else if tk == Char as libc::c_int as libc::c_longlong {
            next();
            bt = CHAR as libc::c_int as libc::c_longlong;
        } else if tk == Enum as libc::c_int as libc::c_longlong {
            next();
            if tk != '{' as i32 as libc::c_longlong {
                next(); // Skip enum name
            }
            if tk == '{' as i32 as libc::c_longlong {
                next();
                i = 0; // Enum value counter
                while tk != '}' as i32 as libc::c_longlong {
                    if tk != Id as libc::c_int as libc::c_longlong {
                        printf(b"%d: bad enum identifier %d\n\0" as *const u8 as *const libc::c_char, line, tk);
                        return -(1 as libc::c_int) as libc::c_longlong;
                    }
                    next();
                    if tk == Assign as libc::c_int as libc::c_longlong {
                        next();
                        if tk != Num as libc::c_int as libc::c_longlong {
                            printf(b"%d: bad enum initializer\n\0" as *const u8 as *const libc::c_char, line);
                            return -(1 as libc::c_int) as libc::c_longlong;
                        }
                        i = ival; // Set enum value
                        next();
                    }
                    *id.offset(Class as libc::c_int as isize) = Num as libc::c_int as libc::c_longlong; // Mark as constant
                    *id.offset(Type as libc::c_int as isize) = INT as libc::c_int as libc::c_longlong;
                    *id.offset(Val as libc::c_int as isize) = i; // Store value
                    i += 1;
                    if tk == ',' as i32 as libc::c_longlong {
                        next();
                    }
                }
                next(); // Consume '}'
            }
        }
        // Parse variables or functions
        while tk != ';' as i32 as libc::c_longlong && tk != '}' as i32 as libc::c_longlong {
            ty_0 = bt;
            // Handle pointer types
            while tk == Mul as libc::c_int as libc::c_longlong {
                next();
                ty_0 = ty_0 + PTR as libc::c_int as libc::c_longlong;
            }
            if tk != Id as libc::c_int as libc::c_longlong {
                printf(b"%d: bad global declaration\n\0" as *const u8 as *const libc::c_char, line);
                return -(1 as libc::c_int) as libc::c_longlong;
            }
            if *id.offset(Class as libc::c_int as isize) != 0 {
                printf(b"%d: duplicate global definition\n\0" as *const u8 as *const libc::c_char, line);
                return -(1 as libc::c_int) as libc::c_longlong;
            }
            next();
            *id.offset(Type as libc::c_int as isize) = ty_0;
            // Function definition
            if tk == '(' as i32 as libc::c_longlong {
                *id.offset(Class as libc::c_int as isize) = Fun as libc::c_int as libc::c_longlong;
                *id.offset(Val as libc::c_int as isize) = e.offset(1 as libc::c_int as isize) as libc::c_longlong; // Function address
                next();
                i = 0; // Parameter counter
                // Parse parameters
                while tk != ')' as i32 as libc::c_longlong {
                    ty_0 = INT as libc::c_int as libc::c_longlong;
                    if tk == Int as libc::c_int as libc::c_longlong {
                        next();
                    } else if tk == Char as libc::c_int as libc::c_longlong {
                        next();
                        ty_0 = CHAR as libc::c_int as libc::c_longlong;
                    }
                    while tk == Mul as libc::c_int as libc::c_longlong {
                        next();
                        ty_0 = ty_0 + PTR as libc::c_int as libc::c_longlong;
                    }
                    if tk != Id as libc::c_int as libc::c_longlong {
                        printf(b"%d: bad parameter declaration\n\0" as *const u8 as *const libc::c_char, line);
                        return -(1 as libc::c_int) as libc::c_longlong;
                    }
                    if *id.offset(Class as libc::c_int as isize) == Loc as libc::c_int as libc::c_longlong {
                        printf(b"%d: duplicate parameter definition\n\0" as *const u8 as *const libc::c_char, line);
                        return -(1 as libc::c_int) as libc::c_longlong;
                    }
                    // Save and set symbol attributes
                    *id.offset(HClass as libc::c_int as isize) = *id.offset(Class as libc::c_int as isize);
                    *id.offset(Class as libc::c_int as isize) = Loc as libc::c_int as libc::c_longlong;
                    *id.offset(HType as libc::c_int as isize) = *id.offset(Type as libc::c_int as isize);
                    *id.offset(Type as libc::c_int as isize) = ty_0;
                    *id.offset(HVal as libc::c_int as isize) = *id.offset(Val as libc::c_int as isize);
                    *id.offset(Val as libc::c_int as isize) = i;
                    i += 1;
                    next();
                    if tk == ',' as i32 as libc::c_longlong {
                        next();
                    }
                }
                next(); // Consume ')'
                if tk != '{' as i32 as libc::c_longlong {
                    printf(b"%d: bad function definition\n\0" as *const u8 as *const libc::c_char, line);
                    return -(1 as libc::c_int) as libc::c_longlong;
                }
                i += 1;
                loc = i; // Set local variable offset
                next(); // Consume '{'
                // Parse local variable declarations
                while tk == Int as libc::c_int as libc::c_longlong || tk == Char as libc::c_int as libc::c_longlong {
                    bt = (if tk == Int as libc::c_int as libc::c_longlong { INT as libc::c_int } else { CHAR as libc::c_int }) as libc::c_longlong;
                    next();
                    while tk != ';' as i32 as libc::c_longlong {
                        ty_0 = bt;
                        while tk == Mul as libc::c_int as libc::c_longlong {
                            next();
                            ty_0 = ty_0 + PTR as libc::c_int as libc::c_longlong;
                        }
                        if tk != Id as libc::c_int as libc::c_longlong {
                            printf(b"%d: bad local declaration\n\0" as *const u8 as *const libc::c_char, line);
                            return -(1 as libc::c_int) as libc::c_longlong;
                        }
                        if *id.offset(Class as libc::c_int as isize) == Loc as libc::c_int as libc::c_longlong {
                            printf(b"%d: duplicate local definition\n\0" as *const u8 as *const libc::c_char, line);
                            return -(1 as libc::c_int) as libc::c_longlong;
                        }
                        // Save and set symbol attributes
                        *id.offset(HClass as libc::c_int as isize) = *id.offset(Class as libc::c_int as isize);
                        *id.offset(Class as libc::c_int as isize) = Loc as libc::c_int as libc::c_longlong;
                        *id.offset(HType as libc::c_int as isize) = *id.offset(Type as libc::c_int as isize);
                        *id.offset(Type as libc::c_int as isize) = ty_0;
                        *id.offset(HVal as libc::c_int as isize) = *id.offset(Val as libc::c_int as isize);
                        i += 1;
                        *id.offset(Val as libc::c_int as isize) = i;
                        next();
                        if tk == ',' as i32 as libc::c_longlong {
                            next();
                        }
                    }
                    next(); // Consume ';'
                }
                // Emit function entry
                e = e.offset(1); *e = ENT as libc::c_int as libc::c_longlong;
                e = e.offset(1); *e = i - loc; // Stack frame size
                // Parse function body
                while tk != '}' as i32 as libc::c_longlong {
                    stmt();
                }
                // Emit function exit
                e = e.offset(1); *e = LEV as libc::c_int as libc::c_longlong;
                // Restore symbol table
                id = sym;
                while *id.offset(Tk as libc::c_int as isize) != 0 {
                    if *id.offset(Class as libc::c_int as isize) == Loc as libc::c_int as libc::c_longlong {
                        *id.offset(Class as libc::c_int as isize) = *id.offset(HClass as libc::c_int as isize);
                        *id.offset(Type as libc::c_int as isize) = *id.offset(HType as libc::c_int as isize);
                        *id.offset(Val as libc::c_int as isize) = *id.offset(HVal as libc::c_int as isize);
                    }
                    id = id.offset(Idsz as libc::c_int as isize);
                }
            } else {
                // Global variable
                *id.offset(Class as libc::c_int as isize) = Glo as libc::c_int as libc::c_longlong;
                *id.offset(Val as libc::c_int as isize) = data as libc::c_longlong; // Data segment address
                data = data.offset(::core::mem::size_of::<libc::c_longlong>() as libc::c_ulong as isize); // Allocate space
            }
            if tk == ',' as i32 as libc::c_longlong {
                next();
            }
        }
        next(); // Consume ';' or '}'
    }

    // Locate main function
    pc = *idmain.offset(Val as libc::c_int as isize) as *mut libc::c_longlong;
    if pc.is_null() {
        printf(b"main() not defined\n\0" as *const u8 as *const libc::c_char);
        return -(1 as libc::c_int) as libc::c_longlong;
    }
    // Exit if source output mode
    if src != 0 {
        return 0;
    }

    // Initialize runtime stack
    sp = (sp as libc::c_longlong + poolsz) as *mut libc::c_longlong; // Set stack top
    bp = sp;
    sp = sp.offset(-1); *sp = EXIT as libc::c_int as libc::c_longlong; // Push exit instruction
    sp = sp.offset(-1); *sp = PSH as libc::c_int as libc::c_longlong; // Push return address
    t = sp;
    sp = sp.offset(-1); *sp = argc; // Push argument count
    sp = sp.offset(-1); *sp = argv as libc::c_longlong; // Push argument vector
    sp = sp.offset(-1); *sp = t as libc::c_longlong; // Push return address

    // Execute generated code
    cycle = 0;
    loop {
        i = *pc; // Fetch instruction
        pc = pc.offset(1);
        cycle += 1;

        // Debug output
        if debug != 0 {
            printf(b"%d> %.4s\0" as *const u8 as *const libc::c_char, cycle,
                &*(b"LEA ,IMM ,JMP ,JSR ,BZ  ,BNZ ,ENT ,ADJ ,LEV ,LI  ,LC  ,SI  ,SC  ,PSH ,OR  ,XOR ,AND ,EQ  ,NE  ,LT  ,GT  ,LE  ,GE  ,SHL ,SHR ,ADD ,SUB ,MUL ,DIV ,MOD ,OPEN,READ,CLOS,PRTF,MALC,FREE,MSET,MCMP,EXIT,\0"
                    as *const u8 as *const libc::c_char).offset((i * 5) as isize) as *const libc::c_char);
            if i <= ADJ as libc::c_int as libc::c_longlong {
                printf(b" %d\n\0" as *const u8 as *const libc::c_char, *pc);
            } else {
                printf(b"\n\0" as *const u8 as *const libc::c_char);
            }
        }

        // Execute instructions
        if i == LEA as libc::c_int as libc::c_longlong {
            a = bp.offset(*pc as isize) as libc::c_longlong; // Load effective address
            pc = pc.offset(1);
        } else if i == IMM as libc::c_int as libc::c_longlong {
            a = *pc; // Load immediate value
            pc = pc.offset(1);
        } else if i == JMP as libc::c_int as libc::c_longlong {
            pc = *pc as *mut libc::c_longlong; // Jump
        } else if i == JSR as libc::c_int as libc::c_longlong {
            sp = sp.offset(-1); *sp = pc.offset(1) as libc::c_longlong; // Push return address
            pc = *pc as *mut libc::c_longlong; // Jump to subroutine
        } else if i == BZ as libc::c_int as libc::c_longlong {
            pc = if a != 0 { pc.offset(1) } else { *pc as *mut libc::c_longlong }; // Branch if zero
        } else if i == BNZ as libc::c_int as libc::c_longlong {
            pc = if a != 0 { *pc as *mut libc::c_longlong } else { pc.offset(1) }; // Branch if non-zero
        } else if i == ENT as libc::c_int as libc::c_longlong {
            sp = sp.offset(-1); *sp = bp as libc::c_longlong; // Save base pointer
            bp = sp;
            sp = sp.offset(-(*pc as isize)); // Allocate stack frame
            pc = pc.offset(1);
        } else if i == ADJ as libc::c_int as libc::c_longlong {
            sp = sp.offset(*pc as isize); // Adjust stack
            pc = pc.offset(1);
        } else if i == LEV as libc::c_int as libc::c_longlong {
            sp = bp; // Restore stack pointer
            bp = *sp as *mut libc::c_longlong; // Restore base pointer
            sp = sp.offset(1);
            pc = *sp as *mut libc::c_longlong; // Return
            sp = sp.offset(1);
        } else if i == LI as libc::c_int as libc::c_longlong {
            a = *(a as *mut libc::c_longlong); // Load integer
        } else if i == LC as libc::c_int as libc::c_longlong {
            a = *(a as *mut libc::c_char) as libc::c_longlong; // Load char
        } else if i == SI as libc::c_int as libc::c_longlong {
            *(sp as *mut libc::c_longlong) = a; // Store integer
            sp = sp.offset(1);
        } else if i == SC as libc::c_int as libc::c_longlong {
            *(sp as *mut libc::c_char) = a as libc::c_char; // Store char
            a = *(sp as *mut libc::c_char) as libc::c_longlong;
            sp = sp.offset(1);
        } else if i == PSH as libc::c_int as libc::c_longlong {
            sp = sp.offset(-1); *sp = a; // Push accumulator
        } else if i == OR as libc::c_int as libc::c_longlong {
            a = *sp | a; // Bitwise OR
            sp = sp.offset(1);
        } else if i == XOR as libc::c_int as libc::c_longlong {
            a = *sp ^ a; // Bitwise XOR
            sp = sp.offset(1);
        } else if i == AND as libc::c_int as libc::c_longlong {
            a = *sp & a; // Bitwise AND
            sp = sp.offset(1);
        } else if i == EQ as libc::c_int as libc::c_longlong {
            a = (*sp == a) as libc::c_int as libc::c_longlong; // Equality
            sp = sp.offset(1);
        } else if i == NE as libc::c_int as libc::c_longlong {
            a = (*sp != a) as libc::c_int as libc::c_longlong; // Inequality
            sp = sp.offset(1);
        } else if i == LT as libc::c_int as libc::c_longlong {
            a = (*sp < a) as libc::c_int as libc::c_longlong; // Less than
            sp = sp.offset(1);
        } else if i == GT as libc::c_int as libc::c_longlong {
            a = (*sp > a) as libc::c_int as libc::c_longlong; // Greater than
            sp = sp.offset(1);
        } else if i == LE as libc::c_int as libc::c_longlong {
            a = (*sp <= a) as libc::c_int as libc::c_longlong; // Less or equal
            sp = sp.offset(1);
        } else if i == GE as libc::c_int as libc::c_longlong {
            a = (*sp >= a) as libc::c_int as libc::c_longlong; // Greater or equal
            sp = sp.offset(1);
        } else if i == SHL as libc::c_int as libc::c_longlong {
            a = *sp << a; // Left shift
            sp = sp.offset(1);
        } else if i == SHR as libc::c_int as libc::c_longlong {
            a = *sp >> a; // Right shift
            sp = sp.offset(1);
        } else if i == ADD as libc::c_int as libc::c_longlong {
            a = *sp + a; // Addition
            sp = sp.offset(1);
        } else if i == SUB as libc::c_int as libc::c_longlong {
            a = *sp - a; // Subtraction
            sp = sp.offset(1);
        } else if i == MUL as libc::c_int as libc::c_longlong {
            a = *sp * a; // Multiplication
            sp = sp.offset(1);
        } else if i == DIV as libc::c_int as libc::c_longlong {
            a = *sp / a; // Division
            sp = sp.offset(1);
        } else if i == MOD as libc::c_int as libc::c_longlong {
            a = *sp % a; // Modulo
            sp = sp.offset(1);
        } else if i == OPEN as libc::c_int as libc::c_longlong {
            a = open(*sp.offset(1) as *mut libc::c_char, *sp as libc::c_int) as libc::c_longlong; // File open
        } else if i == READ as libc::c_int as libc::c_longlong {
            a = read(*sp.offset(2) as libc::c_int, *sp.offset(1) as *mut libc::c_void, *sp as size_t) as libc::c_longlong; // File read
        } else if i == CLOS as libc::c_int as libc::c_longlong {
            a = close(*sp as libc::c_int) as libc::c_longlong; // File close
        } else if i == PRTF as libc::c_int as libc::c_longlong {
            t = sp.offset(*pc.offset(1) as isize); // Get argument count
            a = printf(*t.offset(-1) as *mut libc::c_char, *t.offset(-2), *t.offset(-3), *t.offset(-4), *t.offset(-5), *t.offset(-6)) as libc::c_longlong; // Printf
        } else if i == MALC as libc::c_int as libc::c_longlong {
            a = malloc(*sp as libc::c_ulong) as libc::c_longlong; // Memory allocation
        } else if i == FREE as libc::c_int as libc::c_longlong {
            free(*sp as *mut libc::c_void); // Free memory
        } else if i == MSET as libc::c_int as libc::c_longlong {
            a = memset(*sp.offset(2) as *mut libc::c_void, *sp.offset(1) as libc::c_int, *sp as libc::c_ulong) as libc::c_longlong; // Memory set
        } else if i == MCMP as libc::c_int as libc::c_longlong {
            a = memcmp(*sp.offset(2) as *const libc::c_void, *sp.offset(1) as *const libc::c_void, *sp as libc::c_ulong) as libc::c_longlong; // Memory compare
        } else if i == EXIT as libc::c_int as libc::c_longlong {
            return *sp; // Exit program
        } else {
            printf(b"unknown instruction = %d! cycle = %d\n\0" as *const u8 as *const libc::c_char, i, cycle);
            return -(1 as libc::c_int) as libc::c_longlong;
        }
    }
}

// Entry point for the C4 compiler/interpreter
pub fn main() {
    // Create a vector to store C-style string pointers
    let mut args: Vec::<*mut libc::c_char> = Vec::new();

    // Convert Rust command-line arguments to C-style strings
    for arg in ::std::env::args() {
        // Convert each argument to a CString and leak it to get a raw pointer
        args.push(
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }

    // Add null terminator to the argument list
    args.push(::core::ptr::null_mut());

    // Call the main_0 function with the argument count and array
    unsafe {
        // Exit with the return code from main_0
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_longlong, // Subtract 1 to exclude null terminator
            args.as_mut_ptr() as *mut *mut libc::c_char, // Pass argument array
        ) as i32);
    }
}
