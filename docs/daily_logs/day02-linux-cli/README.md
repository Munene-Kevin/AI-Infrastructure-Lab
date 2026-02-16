LINUX KERNEL:
    It is the core part of the OS.
    It is the first program to run after BIOS and stays running until the machine shuts down.
    It manages the communication between my code and the physical silicon.

    Main jobs of the Kernel:
        ..memory management -> it decides which process get how much RAM and ensures they 'do not step on each other toes'.
        ..process management -> schedules which tasks the CPU handles and in what order.
        ..device drivers -> acts as the translator so the software can talk to hardware.
        ..system calls -> provides a secure "gate" for applications to request resources from the hardware.

    Why it is essential:
        ..security and the least privilege principle -> it allows you to wall off certain parts of the system since you have total control over permissions.
        ..containerization(Docker/Kubernetes) -> containers are a feature of linux kernel. concepts like Namespaces and Cgroups(control groups) are kernel technologies that allow one to package an application so it thinks it is the only thing running on the machine.
        ..performance -> when running heavy AI workloads or Real-time intelligence pipelines, we need to eliminate overhead(wasted resources). linux allows us to strip everything away, running a headless server where 99% of the hardware power goes directly to the AI models. 


Linux filesystem hierarchy:
    /  -> the root  ::: everything starts here.
    /bin & /sbin  -> the tools  ::: * essential binaries(compiled programs)
                                   * commands like 'ls', 'cp', 'mkdir' live here.
                                   * /sbin (system binaries) -> contains tools usually reserved for the root(admin)user, like those for partitioning hard drives.
    /etc  -> the brain  ::: * editable text configurations.
                            * crucial for security.
                            * this is where system-wide settings live.
                            * it is the "settings" menu for the entire OS.
    /var  -> the moving parts  ::: * variable data.
                                   * this is where Logs and Databases live.
                                   * if AI model crashes, we come here to find out why.
    /home  -> the personal space  ::: * user directories.
                                      * when you log in as a user, you land in '/home/your_username'.
                                      * this is the ONLY place you have permission to create files by default.
    /root  -> the fortress  ::: * the home directory for the 'super user'.
                                * to prevent accidental damage, the "Root" user is kept separate from normal users.
                                * you rarely touch this.
    /dev  -> the hardware  ::: * device files.
                               * in linux everything is a file. your hard drive might be '/dev/sda', camera might be '/dev/video0'.
                               

        ## NEVER put raw code in '/bin' or '/etc'.
        ### standard practice:
            # development -> keep code in '/home/user/project'.
            # deployment -> move the finished high-performance Rust services to '/opt'(optional software) or '/usr/local/bin'.


What a file really is:
    ..a collection of three distinct parts separated by the kernel for efficiency and security.

    Three Parts of a File:
        1. The Filename ;; a human-friendly label eg; intel_report.txt.
        2. The Inode(Index Node) ;; the metadata or the passport of the file.
        3. The Data Blocks ;; the actual 1s and 0s stored on the physical disk.


        What is an INODE:
            ..unique data structure on your hard drive that describes a file.
            ..it contains everything except the filename and the actual content.

            Inside an Inode, the Kernel stores;
                a:: file type -> is it a regular file, a directory, or a hardware device?
                b:: permissions -> who can read, write, or execute(RWX)?
                c:: owner/group -> which user owns this data?
                d:: size -> how big is the file?
                e:: timestamps -> when was it created, accessed, or modified?
                f:: pointers -> a 'map' telling the kernel exactly which physical blocks on the disk contain the data.

        Why Inodes matter for security and AI:
            ..moving files is instant -> the 'mv' command just updates the directory to point that filename to the same Inode. The data stays exactly where it is.
            ..deleting files(the "Link Count") -> if you have two filenames pointing to one Inode and you delete one filename, the data is NOT DELETED. The kernel only wipes the data when the Link Count hits ZERO.
                ..you can "lock" a piece of data by creating a hard link to it in a secure hidden directory. Even if a user deletes the oroginal file, the data remains safe in your hidden link.
            ..the "File Limit" -> every linux partition has a fixed number of Inodes created when the drive is formatted.
                ..if your AI System generates millions of tiny 1kb log files, you can actually run out of space even if your hard drive is only 10% full - because you have run out of Inodes(Index numbers) to assign to them.


The Permission Model(UGO/RWX):
    ..linux is built on the Least Privilege principle.
    ..every file and directory has an owner and a set of 'mode bits'.

    The Triad:
        ** user(u) -> the individual who owns the file.
        ** group(g) -> a collection of users eg; the analysts group.
        ** other(o) -> everyone else on the system( the world)

    The Powers:
        Bit        Letter       Numeric     Action on File      Action on Directory
        ^ Read       r               4       ^ view contents     ^ list files inside(ls)
        ^ Write      w               2       ^ modify/delete     ^ add/remove files
        ^ Execute    x               1       ^ run as a program  ^ enter the directory(cd)

        ..The Math:
            you add the numbers ie; r(4) + w(2) + x(1) = 7

            ..755 -> owner can do everything
            ..600 -> only owner can read/write. Essential for private AI keys.


The procss model(the lifecycle of code):
    ..process is just a program in execution.
    ..every process has an ID (PID).

    parent and child(forking);
        ..in linux, processed do not just appear, they are born from parents.
        1. Fork -> a parent process(like the terminal) creates an exact copy of itself.
        2. Exec -> the copy replaces its 'brain' with a new program(like a python script).
        3. PPID -> Parent PID. if the parent dies, the child becomes an orphan and is usually adopted by PID 1(the system's first process).

    signals;
        ..signals are how the kernel tells a process to do something immediately.
        1. SIGINT (2) -> please stop(Ctrl+C).
        2.SIGKILL (9) -> die instantly(the kernel deletes the process from RAM).
        3. SIGTERM (15) -> wrap up your work and close safely


Pipes and Redirection:
    .. redirection( > [overwrites] or >> [appends to the end] ) :: send output to a file instead of the screen.
    .. the pipe( | ) :: connects the 'stdout' of one process to the 'stdin' of another.


Environment Variables and PATH:
    ..variables are short-term memory for the system.
    ..Environment Variables :: key-value pairs like [DB_PASSWORD=secret]. The python/Rust code reads these to avoid "hard-coding" sensitive info.
    ..The PATH :: a special variable containing a list of directories.


APT INternals(Package Management):
    ...APT(Advanced Package Tools) is how you securely install software. it does not just download a file; it manages a trust network.
    1. repositories -> a list of trusted warehouses stored in '/etc/apt/sources.list'.
    2. the update -> 'apt update' downloads a manifest(catalog) of every available program and its version.
    3. dependency resolution -> if you want to install an AI library that requires 'Lib-ABC', APT calculates the "Tree" and installs everything required in the correct order.
    4. verification -> APT uses GPG Keys(Digital Signatures). if the code was tampered with by a hacker, the signature will not match, and APT will refuse to install it.