pymainprocess
=============

Python Extension Module for the best System Interaction, based on Rust.

opinion
=======

1. Faster then Python OS and Python Subprocess Module.
2. Great Interaction, fast Import.
3. Runs Greate on Windows and Unix.

install
=======

pip
---

.. code-block:: bash

   python3 -m pip install pymainprocess

pip from git
------------

.. code-block:: bash

   python3 -m pip install git+https://github.com/pyrootcpp/pymainprocess

pip from archive
----------------

.. code-block:: bash

   python3 -m pip install https://github.com/pyrootcpp/pymainprocess/archive/master.zip

for pyrootcpp APT Repo user
---------------------------

.. code-block:: bash

   sudo apt-get install python3-pymainprocess

Import
======

Best Practice
-------------

.. code-block:: python

   import pymainprocess as procs


Using
=====

The Most Actions are similiar to OS and Subprocess but faster and better.

Maybe you want Safe an Output.

.. code-block:: python

   import pymainprocess as proc

   command = "dpkg --print-architecture"
   stdout = proc.call(command, stdout=True, safe_output=True)
   arch = stdout.strip()

This Module have an Implemented function for run commands as sudo, for example

.. code-block:: python

   from pymainprocess import sudo

   command = "apt-get update"
   user = "root"
   sudo(command=command, user=user)

sudo is not available on Windows Computer.