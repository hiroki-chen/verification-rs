# Known problems

* The Why3 prover must be run in graphic mode, and if we simply invoke the `prove` command, the prover simply hangs and never responds to our query, and as per Xavier Danis:

> How are you using Creusot? Your issue makes me think you are calling why3 prove directly, which is not the intended way to use Creusot or Why3, **it really must be used interactively with the IDE to gain the full potential.**

**Solution 1:** Use X11 protocol to connect to the GUI interface provided by the remote server or simply use a local machine.

**Solution 2:** Install a newer version of `alt-ergo` (e.g. 2.4.1) using `opam`.
