Created the AI Infrastructure Lab directory with git tracking.

Command used:
    ** mkdir -p -> this creates nested directories safely.
    ** cd -> navigation to other directory from current open directory.
    ** git init -> creates a new git repository.
                -> can be used to convert an existing, unversioned project into a git repository.
                -> Initialization of a nwe empty repository.
    ** touch -> used to change file timestamps.
             -> creates an empty file if it does not exist.
             -> often used to create placeholder files or update timestamps for build systems.

        Timestamp: used to track when data was created,      modified or accessed.
                 : touch updates access and modification times of an existing file to current time.

    ** find . -type d -empty -exec touch {}/.gitkeep \; -> adds .gitkeep inside empty directories so structure is maintained.

        .gitkeep; a file placed in empty directories so git can track them.
            NOTE:: git does not track empty directories.

    ** git add . -> add change(s) in the working directory to staging area.
    ** git commit -m ".." -> used to create a snapshot of the staged changes along a timeline of a git project history.
                          -> it is used only after 'git add .'
                          -> ".." ;; put a decriptive sentence of what changes you have done.
    ** git branch -M main -> change from master to main.

        master vs main::
            -Both are names for the default branch in a  git repository.
            -Changed from 'master' to 'main' in 2020 to avoid terminology associated with 'master/slave'

    ** git remote add origin https://github.com/YOUR_USERNAME/REPOSITORY_NAME.git -> 
            ^ git remote :: tells git you want to manage connections to external(remote) repositories.
            ^ add :: the action of creating the new connection.
            ^ origin :: standard industry name for your primary central server.

    ** git push -u origin main -> takes the code you have saved locally on your computer and uploads it to your github repository for the FIRST time.