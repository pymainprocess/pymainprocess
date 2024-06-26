import pymainprocess as _pyprocess
import importlib.util as _util

class Plugin:
    """
    Class to Works with Plugins from pymainprocess.
    """
    def __init__(self, name: str, path: str = None):
        """
        Initial the Plugin with Given Path, load the Plugin and Get the Functions.
        """
        self.__home__ = _pyprocess.environ.get('HOME')
        self.__name__ = name
        if path is None:
            self.__path__ = _pyprocess.path.join(self.__home__, '.config', 'pymainprocess', 'plugins')
            self.__plugin__ = _pyprocess.path.join(self.__path__, name, 'preload.py')
        else:
            self.__path__ = path
            self.__plugin__ = _pyprocess.path.join(self.__path__, f'{name}.py')
        if not _pyprocess.path.exists(self.__plugin__):
            raise FileNotFoundError(f"Plugin {name} not found.")
        
        self._load_plugin()

    def _load_plugin(self):
        """
        Load the Plugin from the Given Path.
        """
        spec = _util.spec_from_file_location(name=self.__name__, location=self.__plugin__)
        self.__module__ = _util.module_from_spec(spec)
        spec.loader.exec_module(self.__module__)

        for attr in dir(self.__module__):
            if not attr.startswith('__'):
                func = getattr(self.__module__, attr)
                if callable(func):
                    setattr(self, attr, func)