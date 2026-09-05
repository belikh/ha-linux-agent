---
title: See Also
id: see-also
tags:
- linux-agent-jupiteros-fleet-15537b
- systemd
- logind
- dbus
- man-page
created: '2026-09-02T07:02:24.617333Z'
updated: '2026-09-02T17:37:22.381178Z'
source: https://raw.githubusercontent.com/systemd/systemd/main/man/systemd-logind.service.xml
source_domain: raw.githubusercontent.com
fetched_at: '2026-09-02T07:02:24.616108Z'
fetch_provider: builtin
status: review
type: note
tier: unknown
content_type: unknown
deprecated: false
summary: 'Canonical DocBook XML source of the systemd-logind.service(8) man page,
  fetched from systemd''s own GitHub repo (man/systemd-logind.service.xml, main branch)
  because freedesktop.org (both the wiki page and the rendered man page) sits behind
  an HTTP 418 anti-bot challenge and man7.org/man.archlinux.org/manpages.debian.org
  mirrors were false-positive login walls. Content: systemd-logind is the system service
  that manages user logins, responsible for tracking users/sessions/processes/idle
  state (implemented as a systemd slice unit per user below user.slice, a scope unit
  per concurrent session, and user@.service per logged-in user); generating session
  IDs (reuses audit session ID when available); polkit-based access for shutdown/sleep;
  shutdown/sleep inhibition locks; power/sleep hardware keys; multi-seat; session
  switch; device access management for users; automatic getty spawning on virtual
  console activation; user runtime directory management; scheduled shutdown; wall
  messages. Sessions register via the pam_systemd PAM module. D-Bus APIs: org.freedesktop.login1
  and org.freedesktop.LogControl1; config in logind.conf(5); concepts in sd-login(3).
  Also exposes a Varlink interface io.systemd.Shutdown (PowerOff/Reboot/Halt/KExec/SoftReboot,
  each with optional skipInhibitors bool matching SD_LOGIND_SKIP_INHIBITORS); queryable
  via ''varlinkctl introspect /run/systemd/io.systemd.Login io.systemd.Shutdown''.
  SIGHUP reloads config. For the agent: org.freedesktop.login1 D-Bus (via zbus) is
  the session/presence telemetry source, and the Varlink/D-Bus shutdown interfaces
  are the clean reboot/poweroff control paths that respect inhibitor locks.'
---

*Suggested by [[how-to-get-notifications-if-a-systemd-unit-fails-help-kde-discuss]] — canonical DocBook source of systemd-logind.service man page (freedesktop bot-wall 418)*

systemd-logind.service
systemd

systemd-logind.service
8

systemd-logind.service
systemd-logind
Login manager

systemd-logind.service
/usr/lib/systemd/systemd-logind

Description

systemd-logind is a system service that
manages user logins. It is responsible for:

Keeping track of users and sessions, their processes and their idle state. This is implemented by
allocating a systemd slice unit for each user below user.slice, and a scope unit below it
for each concurrent session of a user. Also, a per-user service manager is started as system service instance of
user@.service for each logged in user.

Generating and managing session IDs. If auditing is available and an audit session ID is already set for
a session, then this ID is reused as the session ID. Otherwise, an independent session counter is
used.

Providing polkit-based access for users for
operations such as system shutdown or sleep

Implementing a shutdown/sleep inhibition logic for applications

Handling of power/sleep hardware keys

Multi-seat management

Session switch management

Device access management for users

Automatic spawning of text logins (gettys) on virtual console activation and user
runtime directory management

Scheduled shutdown

Sending "wall" messages

User sessions are registered with logind via the
pam_systemd8
PAM module.

See
logind.conf5
for information about the configuration of this service.

See
sd-login3
for information about the basic concepts of logind
such as users, sessions and seats.

See
org.freedesktop.login15
and
org.freedesktop.LogControl15
for information about the D-Bus APIs systemd-logind provides.

In addition to the D-Bus interface, systemd-logind also provides a Varlink
interface io.systemd.Shutdown for shutting down or rebooting the system. It
supports PowerOff, Reboot, Halt,
KExec, and SoftReboot methods. Each method accepts an
optional skipInhibitors boolean parameter to bypass active block inhibitors
(matching the SD_LOGIND_SKIP_INHIBITORS flag of the D-Bus interface). The
interface can be queried with
varlinkctl introspect /run/systemd/io.systemd.Login io.systemd.Shutdown.

For more information see
Inhibitor Locks.

If you are interested in writing a display manager that makes use of logind, please have look at
Writing Display
Managers.
If you are interested in writing a desktop environment that makes use of logind, please have look at
Writing
Desktop Environments.

Signal

SIGHUP
Reloads the service configuration file.

See Also

systemd1
systemd-user-sessions.service8
loginctl1
logind.conf5
pam_systemd8
sd-login3
org.freedesktop.login15