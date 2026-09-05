---
title: child.cc
id: childcc
tags:
- linux-agent-jupiteros-fleet-15537b
- sops-nix
- drop-in-pattern
created: '2026-09-02T17:20:36.980214Z'
updated: '2026-09-05T10:51:22.473493Z'
source: https://raw.githubusercontent.com/NixOS/nix/master/src/libstore/unix/build/child.cc
source_domain: raw.githubusercontent.com
fetched_at: '2026-09-02T17:20:36.973862Z'
fetch_provider: builtin
status: evergreen
type: note
deprecated: false
summary: logger = makeSimpleLogger().release();
---

#include "nix/store/build/child.hh"
#include "nix/util/current-process.hh"
#include "nix/util/logging.hh"

#include
#include

namespace nix {

void commonChildInit()
{
logger = makeSimpleLogger().release();

static const std::string pathNullDevice = "/dev/null";
restoreProcessContext(false);

/* Put the child in a separate session (and thus a separate
process group) so that it has no controlling terminal (meaning
that e.g. ssh cannot open /dev/tty) and it doesn't receive
terminal signals. */
if (setsid() == -1)
throw SysError("creating a new session");

/* Dup stderr to stdout. */
if (dup2(STDERR_FILENO, STDOUT_FILENO) == -1)
throw SysError("cannot dup stderr into stdout");

/* Reroute stdin to /dev/null. */
int fdDevNull = open(pathNullDevice.c_str(), O_RDWR | O_CLOEXEC);
if (fdDevNull == -1)
throw SysError("cannot open '%1%'", pathNullDevice);
if (dup2(fdDevNull, STDIN_FILENO) == -1)
throw SysError("cannot dup null device into stdin");
close(fdDevNull);
}

} // namespace nix

## Related

- [[build]]
