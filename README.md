# Bawn

Bawn (formally Portable Pools) is a user friendly sandbox generator. To create and enter a user sandbox, simply execute bawn with your sandbox name.

Example: Create a test sandbox:

```bash
➜  pools git:(next) bawn test
╰─>Portable·org.kraftland.portable.test·🤓 ⤔

```

Usage:

```
portable-pools [Options] <Sandbox Name>

Usage:
	bawn <sandbox name> [options]
	Note that <option> means required, [option] means optional
	Available options:
		--inspect: print out generated sandbox configuration
		--discrete-gpu / -g: expose all GPUs to the sandbox
		--x11 / -x: Enable access to X11 on Wayland
		--no-lockdown / -n: Disable lockdown mode
	All arguments must be valid UTF-8 characters, additional restrictions
		apply for sandbox name

```