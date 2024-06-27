#include "call.hpp"

#ifdef _WIN32
#include <Windows.h>
#else
#include <unistd.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <cstring>
#include <vector>
#include <sstream>
#endif

namespace call {

int process(const std::string& command) {
#ifdef _WIN32
    STARTUPINFO si;
    PROCESS_INFORMATION pi;
    ZeroMemory(&si, sizeof(si));
    si.cb = sizeof(si);
    ZeroMemory(&pi, sizeof(pi));

    if (!CreateProcess(NULL, const_cast<char*>(command.c_str()), NULL, NULL, FALSE, 0, NULL, NULL, &si, &pi)) {
        return GetLastError();
    }

    WaitForSingleObject(pi.hProcess, INFINITE);
    CloseHandle(pi.hProcess);
    CloseHandle(pi.hThread);

    return 0;
#else
    pid_t pid = fork();
    if (pid == -1) {
        return errno;
    } else if (pid == 0) {
        // Child process
        std::istringstream iss(command);
        std::vector<char*> args;
        std::string arg;
        while (iss >> arg) {
            args.push_back(strdup(arg.c_str()));
        }
        args.push_back(nullptr);

        execvp(args[0], args.data());
        _exit(EXIT_FAILURE); // execvp failed
    } else {
        // Parent process
        int status;
        if (waitpid(pid, &status, 0) == -1) {
            return errno;
        }
        if (WIFEXITED(status)) {
            return WEXITSTATUS(status);
        } else {
            return -1; // Process did not exit normally
        }
    }
#endif
}

}