# pymainprocess

Python Extension Module for the best System Interaction, based on Rust.

# Opinion

1. Faster then Python OS and Python Subprocess Module.
2. Great Interaction, fast Import.
3. Runs Greate on Windows and Unix.

# Install

<details>
    <summary>Install with pip</summary>
    <ul>
        <li>
            <h1>Install from pypi.org</h1>
            <a href="https://pypi.org/project/pymainprocess/">pymainprocess</a>
            <pre>
                <code>
                    pip install -U --no-cache-dir pymainprocess
                </code>
            </pre>
        </li>
        <li>
            <h1>Install via git (Way 1)</h1>
                <ol>
                    <li>
                        <h2>Install Rust (Only Unix)</h2>
                        <pre>
                            <code>
                                curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
                            </code>
                        </pre>
                    </li>
                    <li>
                        <h2>Install Module with git</h2>
                        <pre>
                            <code>
                                pip install git+https://github.com/pymainprocess/pymainprocess
                            </code>
                        </pre>
                    </li>
                </ol>
        </li>
        <li>
            <h1>Install via git (Way 2)</h1>
            <ol>
                <li>Install Rust (Only Unix)</li>
                <li>
                    <h2>Install Module with git</h2>
                    <pre>
                        <code>
                            pip install https://github.com/pymainprocess/pymainprocess/archive/master.zip
                        </code>
                    </pre>
                </li>
            </ol>
        </li>
    </ul>
</details>

# Import

```python
import pymainprocess as proc
```

# Using

## Run Command

```python
command = 'apt-get update'
# Simple Run
proc.call(command)
# Run a Sudo
proc.sudo(command)
# Safe all output
stdout, stderr = proc.call(command, stdout=True, stderr=True)
# Safe all ouput as sudo
stdout, stderr = proc.sudo(command, stdout=True, stderr=True)
# Get only stdout
stdout = proc.call(command, stdout=True)
# Get only stderr
stderr = proc.call(command, stderr=True)
# Now as sudo
stdout = proc.sudo(command, stdout=True)
# Now as sudo
stderr = proc.sudo(command, stderr=True)
```

## Work with Child Process (Only on Unix)

```python
# Create a New Function
def cmd(command: str):
    # Get the pid
    pid = proc.fork(rust=True)
    # Now make the Command as List
    command = command.split(" ")
    # Now get the needed varibles
    file = command[0]
    args = command
    # Wait for PID 0
    if proc.wait(pid):
        proc.execvp(file, args, rust=True)

# Now Test the Function
command = 'ls -A'
cmd(command)
```

## Walk the Current Dir

```python
# A Simple walk will sort all Files and Dirs in the Given Path
path = proc.path.join('/', 'usr', 'local')
result = proc.path.walk(path)
# A Recursive Run will list the Current Path with all Sub Paths
result = proc.path.walk(path, recursive=True)
```

The Output looks so
===================

```python
(.venv) alex@DESKTOP-E4UA3DM:~/pymainprocess$ python
Python 3.12.3 (main, Apr 10 2024, 05:33:47) [GCC 13.2.0] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>> import pymainprocess as proc
>>> path = '/usr/local'
>>> result = proc.path.walk(path)
>>> result2 = proc.path.walk(path, recursive=True)
>>>
```

result
------

```python
>>> result
{'local': {'dirs': ['etc', 'man', 'share', 'sbin', 'games', 'src', 'lib', 'bin', 'include'], 'files': []}}
>>>
```

result2
-------

```python
>>> result2
{'local': {'dirs': [{'etc': {'files': [], 'dirs': []}}, {'man': {'dirs': [], 'files': []}}, {'share': {'dirs': [{'xml': {'files': [], 'dirs': [{'entities': {'files': [], 'dirs': []}}, {'misc': {'dirs': [], 'files': []}}, {'schema': {'files': [], 'dirs': []}}, {'declaration': {'dirs': [], 'files': []}}]}}, {'man': {'dirs': [], 'files': []}}, {'sgml': {'dirs': [{'entities': {'dirs': [], 'files': []}}, {'stylesheet': {'dirs': [], 'files': []}}, {'dtd': {'dirs': [], 'files': []}}, {'misc': {'dirs': [], 'files': []}}, {'declaration': {'files': [], 'dirs': []}}], 'files': []}}, {'fonts': {'files': [], 'dirs': []}}, {'ca-certificates': {'dirs': [], 'files': []}}], 'files': []}}, {'sbin': {'files': [], 'dirs': []}}, {'games': {'files': [], 'dirs': []}}, {'src': {'dirs': [], 'files': []}}, {'lib': {'files': [], 'dirs': [{'python3.12': {'files': [], 'dirs': [{'dist-packages': {'dirs': [], 'files': []}}]}}]}}, {'bin': {'files': ['maturin', 'cargo-deb', 'cargo-clone', 'mdbook'], 'dirs': []}}, {'include': {'dirs': [], 'files': []}}], 'files': []}}
>>>
```

The Output can be very Big, for this is Include as Function walksearch

walksearch
==========

```python
>>> proc.path.walksearch(result2, ['lib', 'python3.12'])
{'files': [], 'dirs': [{'dist-packages': {'dirs': [], 'files': []}}]}
>>>
```