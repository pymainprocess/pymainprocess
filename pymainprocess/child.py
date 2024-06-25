import os as _os
import pymainprocess

def fork() -> int:
    """
    Fork an New Process and Get an PID.
    """
    return _os.fork()

def wait(pid: int) -> bool:
    """
    Wait for pid 0. If bool = True, can you works with the pid.
    """
    if pid != 0:
        _os.wait()
    else:
        return True
    
def execvp(file: str, args: list):
    """
    Execute a Command with Given File and Arguments.
    """
    _os.execvp(file=file, args=args)

def execve(command: any):
    """
    Execute a Command with Given Command String or List.
    """
    if not isinstance(command, (str, list)):
        raise pymainprocess.ProcessBaseError("Command is not an List or Argument")
    if isinstance(command, str):
        command = command.split(" ")
        file = command[0]
        args = command
    if isinstance(command, list):
        file = command[0]
        args = command
    execvp(file=file, args=args)

def execute(command: any):
    """
    Execute an Command on the Child Process and Safe.
    """
    pid = fork()
    if wait(pid):
        execve(command)