# Workload report generator

Gets worklog report from Jira and calculated workload.

## Platform

Windows 10

Requires LLVM to be installed in the system. Used by xlsxwriter crate.
Windowns binary can be downloaded from [LLVM releases](https://releases.llvm.org/)

## Setup

Configuration is read from config.toml which is stored near executable.

## VSCode integration

Ctrl+Shift+B - trigger build menu

.vscode/tasks.json content

```json
{
 "version": "2.0.0",
 "tasks": [
  {
   "type": "shell",
   "label": "cargo check",
   "command": "cargo",
   "args": [
    "check"
   ],
   "problemMatcher": [
    "$rustc"
   ],
   "group": "build"
  },
  {
   "type": "shell",
   "label": "cargo build",
   "command": "cargo",
   "args": [
    "build"
   ],
   "problemMatcher": [
    "$rustc"
   ],
   "group": "build"
  },
  {
   "type": "shell",
   "label": "cargo run",
   "command": "cargo",
   "args": [
    "run"
   ],
   "problemMatcher": [
    "$rustc"
   ],
   "group": "build"
  },
  {
   "type": "shell",
   "label": "cargo clippy",
   "command": "cargo",
   "args": [
    "clippy"
   ],
   "problemMatcher": [
    "$rustc"
   ],
   "group": "build"
  },
  {
   "type": "shell",
   "label": "cargo build --release",
   "command": "cargo",
   "args": [
    "build",
    "--release"
   ],
   "problemMatcher": [
    "$rustc"
   ],
   "group": "build"
  }
 ]
}
```
