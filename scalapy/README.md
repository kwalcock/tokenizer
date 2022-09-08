# Apps

1. Check `python.sbt` to see if anything needs to change.  There are several strings that specify directory and file names that have been hard-coded and won't work on your computer.


2. Several Python packages are required and might be installed with these commands:
 
    * pip3 install torch
    * pip3 install tqdm
    * pip3 install sklearn
    * pip3 install ipywidgets (for Notebook)


3. If you are using IntelliJ, remember to include JVM options similar to these which for Linux and Mac should be printed by `sbt` during the build:
    * Linux:
        * -Djna.library.path=/usr/lib/python3.8/config-3.8-x86_64-linux-gnu:/usr/lib
        * -Dscalapy.python.library=python3.8
    * Mac:
        * -Djna.library.path=/Library/Frameworks/Python.framework/Versions/3.10/lib/python3.10/config-3.10-darwin:/Library/Frameworks/Python.framework/Versions/3.10/lib
        * -Dscalapy.python.library=python3.10
    * Windows:
        * -Djna.library.path=D:/ProgramFiles/Python39
        * -Dscalapy.python.library=python39
