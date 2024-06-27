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
    from platform import system as _sys
    if _sys().lower() == "windows":
        raise WindowsOnly("This Action is only for Unix.")
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

def listdir(path: str = getcwd(), use_sudo: bool = False) -> list:
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
        if use_sudo:
            stdout = sudo(f"ls -A {path}", stdout=True)
        else:
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

    @staticmethod
    def basename(path: str) -> str:
        """
        Get the Basename from an Path.
        """
        from .pymainprocess import path_basename as _basename
        return _basename(path)

    @staticmethod
    def splitext(path: str) -> tuple:
        """
        Split an Path into a Tuple with Basename and Extension.
        """
        from .pymainprocess import path_splitext as _splitext
        return _splitext(path)

    @staticmethod
    def walk(start_path: str = getcwd(), recursive: bool = False) -> any:
        """
        Walk through a Directory and Get all Dirs, Subdirs, Files and More.
        """
        from .pymainprocess import path_walk as _walk
        if not recursive:
            return _walk(start_path)
        else:
            _result = _walk(start_path)
            basename = path.basename(start_path)
            _dirs = _result[basename]['dirs']
            def recursive_walk(base_path, dirs):
                for i, _dir in enumerate(dirs):
                    full_path = path.join(base_path, _dir)
                    dirs[i] = _walk(full_path)
                    sub_basename = path.basename(full_path)
                    if 'dirs' in dirs[i][sub_basename]:
                        recursive_walk(full_path, dirs[i][sub_basename]['dirs'])
            recursive_walk(start_path, _dirs)
            return _result
        
    @staticmethod
    def walksearch(result: dict, subdirs: list, search_file: bool = True, search_dir: bool = True) -> dict:
        """
        Filter the walk result based on search_file and search_dir parameters.
        """
        def recursive_search(current_result, subdirs):
            if not subdirs:
                return current_result
            
            current_dir = subdirs[0]
            remaining_subdirs = subdirs[1:]
            
            for subdir_dict in current_result:
                subdir_name = next(iter(subdir_dict))
                if subdir_name == current_dir:
                    next_result = subdir_dict[subdir_name]
                    if remaining_subdirs:
                        if 'dirs' in next_result:
                            return recursive_search(next_result['dirs'], remaining_subdirs)
                    else:
                        return next_result
            return {}

        _start_name = next(iter(result))
        filtered_result = recursive_search(result[_start_name]['dirs'], subdirs)
        
        if not search_file and 'files' in filtered_result:
            del filtered_result['files']
        
        if not search_dir and 'dirs' in filtered_result:
            del filtered_result['dirs']
        
        return filtered_result

    @staticmethod
    def split(path: str) -> list:
        """
        Split an Path into a List.
        """
        from .pymainprocess import path_split as _split
        return _split(path)

    @staticmethod
    def realpath(path: str) -> str:
        """
        Get the Real Path of a Path.
        """
        from .pymainprocess import path_realpath as _realpath
        return _realpath(path)

    @staticmethod
    def islink(path: str) -> bool:
        """
        Check if the Path is a Symbolic Link or not.
        """
        from .pymainprocess import path_islink as _islink
        return _islink(path)

    @staticmethod
    def ismount(path: str) -> bool:
        """
        Check if the Path is a Mount or not.
        """
        from platform import system as _sys
        if _sys().lower() == "windows":
            raise UnixOnly("This Action is only for Unix.")
        from .pymainprocess import path_ismount as _ismount
        return _ismount(path)

    @staticmethod
    def splitroot(path: str) -> list:
        """
        Split an Path into a List.
        """
        from .pymainprocess import path_splitroot as _splitroot
        return _splitroot(path)

    @staticmethod
    def symlink(original: any, link: any):
        """
        Create an Symlink.
        """
        from .pymainprocess import path_symlink as _symlink
        if not isinstance(original, (str, list)):
            raise ProcessBaseError("Original must be a string or a list.")
        if not isinstance(link, (str, list)):
            raise ProcessBaseError("Link must be a string or a list.")
        if isinstance(original, list):
            if isinstance(link, list):
                if len(original) != len(link):
                    raise ProcessBaseError("Original and Links must have the same length")
                else:
                    for i in range(len(original)):
                        _symlink(original[i], link[i])
            else:
                raise ProcessBaseError("You must pass str and str or list and list")
        elif isinstance(original, str):
            if isinstance(link, str):
                _symlink(original, link)
            else:
                raise ProcessBaseError("You must pass str and str or list and list")

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

def copy(src: str, dest: str):
    """
    Copy Files from src to dest
    """
    from .pymainprocess import copy as _copy
    if src == dest:
        raise ProcessBaseError("Source and Destination are the same.")
    if path.is_dir(src):
        _copy(src, dest, is_dir=True)
    else:
        _copy(src, dest, is_dir=False)

__all__.append("copy")

def remove(filepath: str):
    """
    Remove the File or Dir.
    """
    from .pymainprocess import remove as _remove
    if path.is_dir(filepath):
        _remove(filepath, is_dir=True)
    else:
        _remove(filepath, is_dir=False)

class user:
    """
    Class to Works with user.
    """
    @staticmethod
    def add(username: str, password: str, use_sudo: bool = False, add_sudoers: bool = False):
        """
        Add a User.
        """
        from platform import system as _sys
        if _sys().lower() != "linux":
            raise UnixOnly("This Action is only for Linux.")
        from .pymainprocess import useradd as _useradd1
        from .pymainprocess import useradd_with_sudo as _useradd2
        if use_sudo:
            _useradd2(username, password)
        else:
            _useradd1(username, password)
        if add_sudoers:
            _sudoers = path.join('/', 'etc', 'sudoers.d')
            makedir(_sudoers, exist_ok=True)
            with open(path.join(_sudoers, username), 'w') as f:
                f.write(f"{username} ALL=(root) ALL")

    @staticmethod
    def delete(username: str, use_sudo: bool = False, remove_sudoers: bool = False):
        """
        Delete a User.
        """
        from platform import system as _sys
        if _sys().lower() != "linux":
            raise UnixOnly("This Action is only for Linux.")
        from .pymainprocess import userdel as _userdel1
        from .pymainprocess import userdel_with_sudo as _userdel2
        if use_sudo:
            _userdel2(username)
        else:
            _userdel1(username)
        if remove_sudoers:
            _sudoers = path.join('/', 'etc', 'sudoers.d')
            remove(path.join(_sudoers, username))

    @staticmethod
    def uid() -> int:
        """
        Get the UID of the Current User.
        """
        from platform import system as _sys
        if _sys().lower() != "linux":
            raise UnixOnly("This Action is only for Linux.")
        from .pymainprocess import get_uid as _get_uid
        return _get_uid()
    
    @staticmethod
    def gid() -> int:
        """
        Get the GID of the Current User.
        """
        from platform import system as _sys
        if _sys().lower() != "linux":
            raise UnixOnly("This Action is only for Linux.")
        from .pymainprocess import get_gid as _get_gid
        return _get_gid()

    @staticmethod
    def euid() -> int:
        """
        Get the EUID of the Current User.
        """
        from platform import system as _sys
        if _sys().lower() != "linux":
            raise UnixOnly("This Action is only for Linux.")
        from .pymainprocess import get_euid as _get_euid
        return _get_euid()
    
    @staticmethod
    def egid() -> int:
        """
        Get the EGID of the Current User.
        """  
        from platform import system as _sys
        if _sys().lower() != "linux":
            raise UnixOnly("This Action is only for Linux.")
        from .pymainprocess import get_egid as _get_egid
        return _get_egid()

user = user()

__all__.append("user")

def chmod(path: str, mode: any):
    """
    Change the Mode of a File or Dir.
    """
    from platform import system as _sys
    if _sys().lower() == "windows":
        raise UnixOnly("This Action is only for Unix.")
    from .pymainprocess import chmod as _chmod
    if not isinstance(mode, (str, int, float)):
        raise ProcessBaseError("Mode must be a string, int or float.")
    if isinstance(mode, float):
        mode = int(mode)
    if isinstance(mode, str):
        try:
            mode = int(mode)
        except ValueError:
            raise ProcessBaseError("Mode must be a string, int or float.")
    _chmod(path, mode)

def chown(path: str, uid: int, gid: int):
    """
    Change the Owner of a File or Dir.
    """
    from platform import system as _sys
    if _sys().lower() == "windows":
        raise UnixOnly("This Action is only for Unix.")
    from .pymainprocess import chown as _chown
    _chown(path, uid, gid)

def clear():
    """
    Clear the Terminal.
    """
    from .pymainprocess import clear as _clear
    _clear()

def download(url: str, output: str = 'default', curl: bool = False, silent: bool = False):
    """
    Download a File from an Url.
    """
    from .pymainprocess import download as _download
    if output == 'default':
        _raw = url.replace('https://', '').replace('http://', '')
        _name = path.basename(_raw)
        _path = path.join(getcwd(), _name)
        output = _path
    _download(url, output, curl, silent)

class Temporary:
    """
    Class to work with temporary files and directories.
    """
    def __init__(self, is_dir: bool = False, suffix: any = None):
        """
        Initialize a Temporary object.

        :param is_dir: Boolean indicating if a directory should be created.
        :param suffix: Optional suffix for the temporary file or directory.
        """
        if suffix is not None:
            if not isinstance(suffix, str):
                raise ProcessBaseError("Suffix must be a string.")
            self.suffix = suffix
        else:
            self.suffix = None
        self.is_dir = is_dir
        if self.is_dir:
            self.path = self.create_temp_dir(self.suffix)
        else:
            self.path = self.create_temp_file(self.suffix)
    
    def create_temp_file(self, suffix: str):
        """
        Create a temporary file with the given suffix.

        :param suffix: Suffix for the temporary file.
        :return: Path to the temporary file.
        """
        from .pymainprocess import create_temp_file as _create_temp_file
        return _create_temp_file(suffix)
    
    def create_temp_dir(self, suffix: str):
        """
        Create a temporary directory with the given suffix.

        :param suffix: Suffix for the temporary directory.
        :return: Path to the temporary directory.
        """
        from .pymainprocess import create_temp_dir as _create_temp_dir
        return _create_temp_dir(suffix)

    def get_path(self):
        """
        Get the path of the temporary file or directory.

        :return: Path to the temporary file or directory.
        """
        from .pymainprocess import get_temp_path as _get_temp_path
        return _get_temp_path(self.path)

    def cleanup(self):
        """
        Clean up the temporary file or directory.
        """
        from .pymainprocess import cleanup_temp_path as _cleanup_temp_path
        _cleanup_temp_path(self.path, self.is_dir)