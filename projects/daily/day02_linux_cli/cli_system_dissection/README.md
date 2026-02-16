/proc :::: the process and kernel window.
        * it is a view of the kernel's internal data structures.
        
        ls /proc  ->you see many folders with numbers(PIDs).

        /proc/[PID]/status ->to see how much memory the AI model is actually using.

        /proc/[PID]/cmdline ->to see how the program was started.

        cat /proc/cpuinfo ->tells you exactly what processor you have.

        cat /proc/meminfo ->shows RAM memory information.


/dev :::: the hardware warehouse.
        * in linux, everything is a file. if you want to talk to a hard drive, or a secure hardware module, you do it here.
        * it is an access points for physical and virtual device drivers.
        * common files;
            
            /dev/sda ->your primary hard drive.

            /dev/random -> a source of entropy (random numbers). vital for encryption/security.

            /dev/null ->the black hole. anything you send here disappears.

/sys :::: the hardware configuration center.
        * it is a structured way to view and change hardware parameters.
        * it is organized by class i.e; block = drivers, net = internet cards, power = battery.
        * if you want to change how the CPU handles Turbo boost to get more speed for an AI calculation, you write a value to a file in '/sys/devices/system/cpu/...'
        * you can disable USB ports or specific network features by toggling files in this directory.



    'cat /proc/uptime' ::: shows how many seconds the system has been awake.

    'ls -l /dev/stdout' ::: showws that even your screen is just a pointer to a device.