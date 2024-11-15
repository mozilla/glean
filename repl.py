# via https://bernsteinbear.com/blog/simple-python-repl/

import code
import readline  # noqa: F401
import sys

pings = ["prototype", "usage"]

metrics = [
    "boolean",
    "counter",
    "event",
    "event_default",
    "event_no_keys",
]

commands = {
    "metrics": [],
    "pings": [],
    "add": metrics,
    "upload": ["on", "off"],
    "enable": pings,
    "disable": pings,
}


class Completer:
    def __init__(self, env) -> None:
        self.env = env
        self.matches = []

    def complete(self, text: str, state: int):
        cmd = text.split(" ")
        if state == 0:
            if len(cmd) == 1:
                options = (key for key in commands.keys() if key.startswith(text))
                self.matches = sorted(options)
            if len(cmd) == 2:
                c = commands[cmd[0]]
                options = (key for key in c if key.startswith(cmd[1]))
                self.matches = sorted(options)
        try:
            cmd[-1] = self.matches[state]
            return " ".join(cmd)
        except IndexError:
            return None


class Repl(code.InteractiveConsole):
    def runsource(self, source, filename="<input>", symbol="single"):
        # TODO: Integrate your compiler/interpreter
        print("source:", source)


env = {"add": lambda x, y: x + y, "abs": abs}  # some builtins or something
readline.set_completer_delims("")
readline.set_completer(Completer(env).complete)
readline.parse_and_bind("tab: complete")  # or menu-complete
readline.parse_and_bind("bind ^I rl_complete")

repl = Repl()
repl.interact(banner="", exitmsg="")
