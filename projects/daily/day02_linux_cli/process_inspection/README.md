the tool of inspection:

    'ps' :: (process status)
        * gives you a static picture of what is running right now.

        'ps aux'
                a ::: all users
                u ::: display user/owner information
                x ::: processes not attached to a terminal(background services)

                look at the PID and %CPU/%MEM. this tells who is eating the resources.

    'top' and 'htop' :: the live monitor
        'top' :: the classic built-in real-time monitor
        'htop' :: the modern version.
                  usually needs 'sudo apt install htop'.
                  provides a color-coded, interactive view where you can scroll and kill processes without typing PIDs


parent-child trees(pstree):
    processes are hierarchical. when you run a python script from your terminal(Bash), Bash is the parent and Python is the child.

    pstree -p  ::: shows exactly who owns whom.


background and job control:
    start a process in the background :: add an ampersand &
        'sleep 1000 &' (this runs a "do nothing" command for 1000 seconds in the background)
    see your background jobs
        'jobs'
    bring it back to the front
        'fg %1'
    send a running task to the back
        Ctrl+Z(suspend) then type 'bg'
    kill a process
        'kill %1'


influencing performance :::: 'nice'
    niceness range: -20(highest priority) to 19(lowest priority/nicest)
    starting a process with priority:
        'nice -n 10 python3 backup_script.py  :: this makes the script nicer giving way to other tasks.
    changing a running process
        'renice -n 5 -p [PID]  :: requires 'sudo' to make a process less nice.


the kill signals:
    signal: 15
    name : SIGTERM
    command: 'kill [PID]'
    effect: soft kill. asks the program to save its data and exit.

    signal: 9
    name: SIGKILL
    command: 'kill -9 [PID]'
    effect: hard kill. the kernel deletes the process from RAM immediately with no clean up.

    signal: 2
    name: SIGINT
    command: 'Ctrl+C'
    effect: interrupts the process from the keyboard.
