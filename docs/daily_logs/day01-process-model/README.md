Goal:
    *understand:
        What happens when you type ./main
        How a Rust file becomes an executable
        Where that executable lives in memory
        What the OS does to run it


Program on Linux: its an executable file containing a set of instructions that directs the CPU on what to do.
                :They are stored in; /bin, /usr/bin, /usr/local/bin directories.
                :They are defined by executable permissions rather than file extensions (like in windows eg; .exe). These permissions define if they are system programs(system software) or user defined programs(user applications).
                :It becomes an active process when executed.
                :They are launched(executed) from the terminal(shell) or a desktop environment and do not need to be stored on a single dedicated folder.
                :Installed via package managers eg; apt, dnf, pacman, which manages dependencies and store files in specific system directories. 
                :They follow the Executable and Linkable Format(ELF).
                :Types;
                    1. System Programs -> manage resources such as the init system (systemd) or file system utilities(ls, cd, ...).
                    2. Apllication Programs -> tools for users such as terminal emulators, browsers, ...


Process: An active(running) instance of a program.

Source Package: An archieve containing human-readable source code, build scripts, metadata needed to compile a software locally.
              :They must be compiled by the user before installation to create executable binary(s).

Binary Package: A type of package in linux containing pre-compiled files ready for a specific installation.

Binary:
    ..Lives in the '/target' directory.
    ..When you run 'cargo build', it creates an unoptimized version used for testing. Path -> target/debug/day01_process_model
        This is for development.
    ..When you run 'cargo build --release', it creates a highly optimized version ready for deployment. Path -> target/release/day01_process_model
        This is for production.

    Reasons to inspect the Binary:
            ..Security Vulnerability Discovery;
                * binary analysis can identify security flaws such as buffer overflows, format string bugs, and memory safety issues that might be missed by source code analysis alone or introduced during the compilation process.
            ..Malware Analysis;
                * when a system is compromised, we dissect the malicious binary files to understand their infection mechanisms, persistence methods, and communications eg command-and-control servers. This helps in developing effective countermeasures.
            ..Third-Party and Legacy Software Validation;
                * for commercial off-the-shelf products, vendor-provided libraries, or old legacy systems where source code is lost or proprietary, binary analysis is the only way to verify their integrity and ensure they meet security and compliance standards.
            ..Debugging and Troubleshooting;
                * when a program crashes unexpectedly or behaves erratically, and the source code is unavailable, we use debuggers and other binary analysis tools to step through the execution and pinpoint the root cause of the issue.
            ..System Integration and Undocumented Protocols;
                * we may need to inspect a binary to understand undocumented internal workings, data formats, or network protocols to ensure proper integration with other systems or hardware.
            ..Digital Forensics;
                * forensic investigators examine binary files to recover evidence, identify the origin of a piece of code, or understand the timeline of a compromise in the event of a security incident or legal issue.
            ..Performance Optimization;
                * we can identify inefficiencies in code execution and resource usage to optimize critical sections for better performance by examining a binary's instructions and memory access patterns.
            ..Supply Chain Assurance;
                * inspecting binaries help detect if software components have been tampered with or if embedded malware was injected during the build or packaging process(post-build verification).

    How to inspect the Binary:
        ## check the file type;
            ..to confirm it is a true executable and see which architecture its built for eg; x86_64 or ARM.
            .. 'file target/debug/day01_process_model'
        ## view the size;
            ..high-performance systems should be lean.
            .. 'ls -lh target/debug/day01_process_model'
            .. 'ls -lh target/release/day01_process_model'
            ..the release version is often smaller and much faster because Cargo removes "debug symbols".

            rwxrwxr-x -> this is the file permission mode
                      ->[user/owner] [group] [others]
                      -> r = 4 ;;; read
                      -> w = 2 ;;; write
                      -> x = 1 ;;; execute
                      -> numerical value of such is 775
                      > to set this permission, 'chmod 775 filename' or 'chmod u=rwx,g=rwx,o=rz filename'.
        ## look at the "Strings" (security basics);
            ..this command prints every readable sequence of characters inside the binary.
            ..this is how to find hidden URLs or hardcoded passwords in malware analysis.
            .. 'strings target/debug/day01_process_model | grep "PID"'
        ## check shared dependencies(lld);
            ..secure systems should have as few external dependencies as possible.
            ..this shows which system libraries your binary needs to run.
            .. 'ldd target/debug/day01_process_model'
        ## run 'readelf -h target/debug/day01_process_model' to inspect the ELF header.
            ..the first few bytes should be '7f 45 4c 46'(which spells '.ELF'). If these bytes are missing, the file is corrupted or not a Linux executable.
            .. 'Entry point address' is the exact memory location where the OS will start executing the code.

ELF (Executable andLinkable Format):
    ..It is a highly structured document that tells the opearting system exactly how to run the program.
    ..standardized shipping container for the code.
    ..the binary in the '/target' directory is an ELF file.

    @@ what is inside an ELF?
        1. Header
            ..it tells the OS if the file is 64-bit, which CPU it needs (Intel vs ARM), and where the entry point (fn main()) is located.
        2. Sections
            .; .text -> actual machine code i.e; Rust logic.
            .; .data -> global variables I defined.
            .; .rodata -> Read-only data like string eg; Digital Signature (PID)
        3. Symbol Table -> a list of functions and variables used (mostly found in 'debug' builds; usually stripped in 'release' builds for security).

    @@ why ELF?
        1. static analysis ->security tools scan the ELF sections to see if a program is trying to access parts of the system it should not.
        2. reverse engineering -> if a piece of suspicious software is captured, the ELF structure is analyzed to understand its capabilities without actually running it.
        3. stripping for stealth/size -> can use the command 'strip' on an ELF file to remove the symbol table. This makes the binary smaller and harder for an attacker to read the function names.
        


        The '/target' folder can become massive over time and since the binary can be recreated from my code, run 'cargo clean' to wipe the folder and start fresh.


How Linux executes a binary:
    1. The Execution Lifecycle
        i. the shell&system call;
            ..when you type './target/debug/day01_process_model' and hit ENTER, the shell issues a system call called 'execve' which tells the Linux kernel I want to run this specific ELF file.
        ii. the kernel check;
            ..the kernel intercepts the request and performs a security check:
            * does the user have executable permissions?
            * is the file a valid ELF?
        iii. mapping to memory;
            ..the kernel does not just copy the file into RAM. It maps the sections of the ELF into a Virtual Address Space.
            * the '.text'(code) section is marked as Read-Only and Executable.
            * the '.data' section is marked as Read/Write but Non-Executable. This is a key security feature to prevent hackers from running code stored in variables.
        iv. the dynamic linker(the handshake);
            ..the kernel starts a small program called the Dynamic Linker to find the dependencies on the disk and link them to the process unless the binary is statically linked which means it does not need external libraries.
        v. jumping to the entry point;
            ..the kernel sets the CPU's Instruction Pointer to the address of the 'fn main()'. The code is now running.


    2. The Memory Layout:
        ..once the binary is executing, it organizes itself into four main areas of RAM.

        memory area     what lives there                                why it matters in Rust

        stack           fast, short-term variables eg; my_pid            Rust manages this automatically; it is incredibly fast
        heap            flexible, long-term data eg; Vec::new()          Rust uses Ownership to ensure it is cleaned up without a garbage collector
        data            global variables and hardcoded strings           Fixed size; set at the moment of execution
        Text	        The actual machine instructions	                 Locked by the OS; if a program tries to change this while running, it's a "Segfault."  

    3. why it matters;
        ..Address Space Layout randomization(ASLR) -> modern OSs do not put my binary in the same spot in RAM everytime. They shuffle the memory map. This makes it much harder for an attacker to predict where my code is to exploit it.
        ..Static Binaries -> in high-security environments, we often compile Rust to be Static. This means the Dynamic Linking(1:iv) is skipped entirely. Everything the program needs is inside the ELF, making it faster to start and harder to poison by swapping out system libraries.



Why Mathematics matters in Infrastructure Engineering:
    ..It is the language of resource optimization and system reliability.
    1. compexity & scaling (Big O Notation);
        -I must predict how my system will behave as data grows.
        eg;; * linear growth O(n) -> processing 10x more data takes 10x more time :::: good.
             * exponential growth O(2^n) -> processing 10x more data might take 1,000x more time :::: system failure.
    2. probability 7 reliability (The "Five Nines");
        -used to calculate the Mean Time Between Failures(MTBF).
        eg;; * if one server has a 99% chance of being up, and your system requires three specific servers to work, the probability of the whole system being up is 0.99 * 0.99 * 0.99 = 97.02%.
        -math tells us that to reach Five Nines (99.999% uptime), you need redundancy infrastructure.
    3. cryptography (The Math of Security);
        -prime number theory -> modern encryption(RSA) relies on the fact that it is computationally easy to multiply two large prime numbers but incredibly hard (mathematically expensive) to factor the result back into the original primes.
        -hashing algorithms ->  the SHA-256 tool is a mathematical one-way function which maps an input of any size to a fixed-size string of bits.
    4. resource allocation and queuing theory;
        -infrastructure is essentially a series of "pipes" and "buckets"
        -bandwidth -> how many bits per second can the network handle?
        -little's law -> theorem used to calculate the average number of items in a system based on arrival rate and processing time.
        eg; * if the intelligence fees receives 1,000 packets/sec but the Rust binary can only process 800 packets/sec, math tells us exactly how long it will take for the memory(buffer) to overflow and crash the system.
    5. boolean algebra(logic gates);
        -at the lowest level of the ELF binary and the CPU, everything is discrete mathematics.
        -AND, OR, NOT, XOR operations are how the CPU makes decisions.
        -in infrastructure code (like Docker or Terraform), we use Boolean logic to define "Infrastructure as Code" eg; if environment is production AND security_scan is passed, THEN deploy.



    ** numerical instability destroys distributed systems silently.
    ** floating-point error propagation can:
        ^^ break gradient synchronization
        ^^ create non-deterministic training
        ^^ cause cross-node drift
    ** math is not for modelling alone, it is for debugging reality.
