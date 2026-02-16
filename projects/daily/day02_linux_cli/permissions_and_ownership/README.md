Create users:
    sudo adduser [name_of_user]

    verify the user exixts:
        grep "[name_of_user]" /etc/passwd

create a file:
    mkdir ~/[project_name]

    create file and redirect to project directory:
        echo "[file_contents]" > ~/[project_name]/[file_name.txt]

    check current ownersip:
        ls -l ~/[project_name]/[file_name.txt]

change ownership:
    sudo chown [name_of_user]:[name_of_user] ~/[project_name]/[file_name.txt]

    verify the change:
        ls -l ~/[project_name]/[file_name.txt]

intentional sabotage(breaking access):
    set permission to 600;
        chmod 600 ~/[project_name]/[file_name.txt]

    try to read it (the admin):
        cat ~/[project_name]/[file_name.txt]

        ** even though you are the ADMIN(sudo-capable user), the kernel respects the file bits. if you are not the owner and the "Other" bits are 0, you are locked out.


the repair(sudo skeleton key):
    read it with elevated privileges:
        sudo cat ~/[project_name]/[file_name.txt]

    reclaim ownership:
        sudo chown $USEER:$USER ~/[project_name]/[file_name.txt]

    restore standard access:
        chmod 644 ~/[project_name]/[file_name.txt]


deleting a user:
    sudo deluser [name_of_user]
        * remove user but keep their files.

    sudo deluser --remove-home [name_of_user]
        * user is deleted and their entire /home/[name_of_user] directory is nuked.

    sudo userdel [name_of_user] :: deletes user only
    sudo userdel -r [name_of_user]
        * these two are the universal linux commands that work on every distribution.


    under the hood:
        /etc/passwd  ::: user's line is deleted.
        /etc/shadow  ::: encrypted password hash is removed.
        /etc/group   ::: user is removed from any groups they belonged to.

            **the system modifies these critical files when you delete a user.


            ## after deleting a user, always search for orphaned files using;;
                sudo find / -uid 1001 (replacing 1001 with the old ID).