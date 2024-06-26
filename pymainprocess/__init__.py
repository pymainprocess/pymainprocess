from .pymainprocess import ProcessBaseError as _prbaer
from .pymainprocess import CommandFailed as _cofa 
from .pymainprocess import WindowsOnly as _wion
from .pymainprocess import UnixOnly as _unon
from .plugin import Plugin as _plugin

__all__ = []

class Plugin(_plugin):
    """
    Class to Works with Plugins.
    """
    def __init__(self, name: str, path: str = None):
        super().__init__(name=name, path=path)

__all__.append('Plugin')

class ProcessBaseError(_prbaer):
    """
    Base Exception CLass for pymainprocess.
    Attention.
    - This Exception will be Raised if not an Other Better for Error Handling.
    """
    def __init__(self, *args):
        self.args = args
        super().__init__(*args)

    def __str__(self):
        return " ".join(map(str, self.args))

__all__.append("ProcessBaseError")
    
class CommandFailed(_cofa):
    """
    Exception Class for Command Failed.
    """
    def __init__(self, *args):
        self.args = args
        super().__init__(*args)

    def __str__(self):
        return " ".join(map(str, self.args))
    
__all__.append("CommandFailed")
    
class WindowsOnly(_wion):
    """
    Exception Class for Windows Only.
    """
    def __init__(self, *args):
        self.args = args
        super().__init__(*args)

    def __str__(self):
        return " ".join(map(str, self.args))
    
__all__.append("WindowsOnly")

class UnixOnly(_unon):
    """
    Exception Class for Unix Only.
    """
    def __init__(self, *args):
        self.args = args
        super().__init__(*args)

    def __str__(self):
        return " ".join(map(str, self.args))
    
__all__.append("UnixOnly")

def call(command: any, stdout: bool = False, stderr: bool = False, strip: bool = False) -> any:
    """
    Call an Command and Return if needed the Result.
    """
    from .pymainprocess import call as _call1
    from .pymainprocess import call_and_safe as _call2
    if not isinstance(command, (list, str)):
        raise ProcessBaseError("Command must be a list or a string")
    if isinstance(command, (list)):
        command = " ".join(command)
    if stdout and stderr:
        _stdout, _stderr = _call2(command)
        if strip:
            _stdout = _stdout.strip()
            _stderr = _stderr.strip()
        return _stdout, _stderr
    elif stdout:
        _stdout, _stderr = _call2(command)
        if strip:
            _stdout = _stdout.strip()
        return _stdout
    elif stderr:
        _stdout, _stderr = _call2(command)
        if strip:
            _stderr = _stderr.strip()
        return _stderr
    else:
        _call1(command)

__all__.append("call")

def sudo(command: any, user: str = 'root', stdout: bool = False, stderr: bool = False, strip: bool = False):
    """
    Execute a Command as Sudo and user if user given.
    """
    from .pymainprocess import sudo as _sudo1
    from .pymainprocess import sudo_and_safe as _sudo2
    if not isinstance(command, (list, str)):
        raise ProcessBaseError("Command must be a list or a string")
    if isinstance(command, (list)):
        command = " ".join(command)
    if stdout and stderr:
        _stdout, _stderr = _sudo2(command, user)
        if strip:
            _stdout = _stdout.strip()
            _stderr = _stderr.strip()
        return _stdout, _stderr
    elif stdout:
        _stdout, _stderr = _sudo2(command, user)
        if strip:
            _stdout = _stdout.strip()
        return _stdout
    elif stderr:
        _stdout, _stderr = _sudo2(command, user)
        if strip:
            _stderr = _stderr.strip()
        return _stderr
    else:
        _sudo1(command, user)

__all__.append("sudo")

def getcwd() -> str:
    """
    Get the Current Workdir.
    """
    from .pymainprocess import get_cwd as _get_cwd
    return _get_cwd()

__all__.append("getcwd")

def listdir(path: str = getcwd()) -> list:
    """
    List all Files in the Directory.
    """
    from platform import system as _sys
    if _sys().lower() == "windows":
        stdout = call(f"dir {path}", stdout=True)
        return stdout.split("\n")
    elif _sys().lower() == "darwin":
        stdout = call(f"ls -A {path}", stdout=True)
        return stdout.split("\n")
    elif _sys().lower() == "linux":
        stdout = call(f"ls -A {path}", stdout=True)
        return stdout.split("\n")

__all__.append("listdir")

def which(command: str) -> str:
    """
    Found the Location from an Command.
    """
    from .pymainprocess import py_which as _which
    return _which(command)

__all__.append("which")

def fork(rust = True) -> int:
    """
    Fork an new Process an get him Process ID (pid).
    Attention
    - This Function is similar to os.fork()
    """
    from .child import fork as _fork
    import platform
    if platform.system().lower() == "windows":
        raise WindowsOnly("This Action is only for unix.")
    if rust:
        try:
            from .pymainprocess import py_fork as _fork
        except ImportError:
            raise UnixOnly("Unable to fork.")
    return _fork()

__all__.append('fork')

def wait(pid: int):
    """
    Wait for pid.
    Otherwise pass your pid which you have get with fork to him.
    The Result will be a True, you this for the Next ation. For Exmample.
    Example1.
    def cmd(command: str):
        import pymainprocess as proc
        command = command.split(" ")
        file = command[0]
        pid = proc.fork() # Get the PIDi
        if proc.wait(pid): # Create an If Category
            proc.execvp(file, command)
    """
    from .child import wait as _wait
    import platform
    if platform.system().lower() == "windows":
        raise WindowsOnly("This Action is only for unix.")
    return _wait(pid=pid)

__all__.append("wait")

def execvp(file: str, args: list, rust: bool = True):
    """
    Execute an Command with given file and args
    """
    from .child import execvp as _execvp
    import platform
    if platform.system().lower() == "windows":
        raise WindowsOnly("This Action is only for unix.")
    if rust:
        try:
            from .pymainprocess import py_execvp as _execvp
        except ImportError:
            raise UnixOnly("Unable to Execute.")
    return _execvp(file=file, args=args)

def execve(command: any):
    """
    Execute an Command as List or String.
    """
    from .child import execve as _execve
    import platform
    if platform.system().lower() == "windows":
        raise WindowsOnly("This Action is only for unix.")
    return _execve(command=command)

def execute(command: any):
    """
    Execute an Command on the Child Process.
    """
    from .child import execute as _execute
    import platform
    if platform.system().lower() == "windows":
        raise WindowsOnly("This Action is only for unix.")
    return _execute(command=command)

__all__.extend(["execvp", "execve", "execute"])

class path:
    """
    Class to Work with Paths.
    """
    @staticmethod
    def join(path: object, *paths: object) -> str:
        """
        Join an Path with other Paths.
        """
        from .pymainprocess import path_join as _path_join
        return _path_join(path, paths)

    @staticmethod
    def exists(path: str) -> bool:
        """
        Check if the Path exists or not.
        """
        from .pymainprocess import path_exists as _exists
        return _exists(path)

    @staticmethod
    def is_file(path: str) -> bool:
        """
        Check if the Path is a File or not.
        """
        from .pymainprocess import path_is_file as _is_file
        return _is_file(path)

    @staticmethod
    def is_dir(path: str) -> bool:
        """
        Check if the Path is a Directory or not.
        """
        from .pymainprocess import path_is_dir as _is_dir
        return _is_dir(path)

path = path()

__all__.append("path")

class environ:
    """
    Class to Works with Environs.
    """
    @staticmethod
    def get(key: str, file: bool = False, filepath: str = '.env', use_dotenv: bool = True) -> str:
        """
        Get an Environment Variable.
        """
        from .pymainprocess import env_get as _get1
        from .pymainprocess import env_get_from_file as _get2
        if file:
            return _get2(filepath=filepath, key=key, dotenv_use=use_dotenv)
        else:
            return _get1(key=key)
        
    @staticmethod
    def set(key: str, value: str):
        """
        Set an Environment Variable.
        """
        from .pymainprocess import env_set as _set
        return _set(key=key, value=value)
    
    @staticmethod
    def items() -> dict:
        """
        Get all Environment Variables.
        """
        from .pymainprocess import env_items as _items
        return _items()

    @staticmethod
    def reset():
        """
        Reset all Environment Variables.
        """
        from .pymainprocess import env_reset as _reset
        return _reset()

    @staticmethod
    def os_data(data: str) -> str:
        """
        Get an OS Data.
        Available
        - platform / os
        - os_version
        - architecture
        - kernel
        - cpu
        - hostname
        """
        from .pymainprocess import env_os_data as _os_data
        return _os_data(data)

environ = environ()

__all__.append('environ')

def exit(code: int):
    """
    Exit the Process.
    """
    from .pymainprocess import py_exit as _exit
    _exit(code)

__all__.append("exit")

def chdir(path: str = getcwd()):
    """
    Change the Current Workdir, default is the Current Workdir py starting Script, Shell.
    """
    from .pymainprocess import chdir as _chdir
    _chdir(path)

__all__.append("chdir")

def makedir(path: str, exist_ok: bool = False):
    """
    Create a Directory.
    """
    from .pymainprocess import mkdir as _mkdir
    _mkdir(path, exist_ok)