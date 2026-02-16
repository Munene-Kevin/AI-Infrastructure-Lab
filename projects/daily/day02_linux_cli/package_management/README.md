apt vs dpkg:
    apt :: high-level package manager that automates package downloading and dependency resolution and requires an internet connection.

    dpkg :: low-level tool that handles the installation, removal, and management of individual .deb files on a local system and works offline with local files only.

        apt acts as a user-friendly frontend that uses dpkg to perform the actual installation and removal operations.

    
    where is the metadata stored?
        ..metadata is the catalogue of what software is available, where it is, and what version is currently installed.

        the source list: '/etc/apt/sources.list' 
            ..this is a text file that tells 'apt' which warehouses(repositories) to trust.

        the local cache: '/var/lib/apt/lists/'
            ..when you run 'sudo apt update', you are not updating your software; you are updating your METADATA.
                ..'apt' downloads the list of available packages from the servers in your 'sources.list'
                ..it stores these lists in '/var/lib/apt/lists/'
                ..when you search for a program, 'apt' looks here, not the internet.


    why '/var/lib' is critical:
        ..it is the system's memory.
        
        '/var/lib/dpkg/status' :: perhaps the most important file for system auditing. It is a text file containing the state of every single package on your system. It tracks what is installed, what is half installled, and what version is present.

        ..the "lock" file prevents two instances from corrupting each other.


    the lifecycle of a package installation:
        1. request :: 'sudo apt install rustc'
        2. lookup :: 'apt' checks the metadata in '/var/bin/apt/lists/' to see if 'rustc' exists.
        3. dependency resolution :: 'apt' calculates that 'rustc' needs 'libc6'
        4. download :: 'apt' pulls the '.deb' files from the internet and puts them in a temporary holding cell at '/var/cache/apt/archives/'
        5. handoff :: 'apt' calls 'dpkg' and says, "here are the files, put them in the right directories"
        6. record :: 'dpkg' updates '/var/lib/dpkg/status' to reflect that the new software is live.




    cat /var/lib/dpkg/status | grep -A 5 "package: python3"
        shows package name and the next 5 lines of its metadata status.