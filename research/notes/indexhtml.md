---
title: index.html
id: indexhtml
tags:
- linux-agent-jupiteros-fleet-15537b
- storage-health
- home-assistant
- known-issue
- practitioner-guide
- reliability-failure-modes
created: '2026-09-02T05:38:56.228511Z'
updated: '2026-09-05T10:51:21.914240Z'
source: https://docs.oracle.com/cd/E19253-01/819-5461/gamno/index.html
source_domain: docs.oracle.com
fetched_at: '2026-09-02T05:38:46.973011Z'
fetch_provider: builtin
status: evergreen
type: note
tier: institutional
content_type: docs
deprecated: false
summary: 'Oracle Solaris ZFS Administration Guide chapter defining the canonical zpool
  device/vdev states (ONLINE, DEGRADED, FAULTED, OFFLINE, UNAVAIL, REMOVED) and the
  roll-up rules: pool is ONLINE only if ALL top-level vdevs are ONLINE; any DEGRADED
  or UNAVAIL top-level vdev makes the pool DEGRADED; a FAULTED or OFFLINE top-level
  vdev makes the pool FAULTED and completely inaccessible. Documents zpool status
  -x (''all pools are healthy'' machine-friendly check), zpool status -v per-device
  READ/WRITE/CKSUM error columns, the errors: data-error line, and fmd/console//var/adm/messages
  fault reporting. Upstream authority behind the same state vocabulary the Linux OpenZFS
  port and scriptable ''zpool list -H -o health'' use.'
---

Documentation Home  > Oracle Solaris ZFS Administration Guide    > Chapter 4 Managing
Oracle Solaris ZFS Storage Pools    > Querying ZFS Storage Pool Status     > Determining the Health Status of ZFS Storage Pools

Oracle Solaris ZFS Administration Guide

Previous: Viewing I/O Statistics for ZFS Storage Pools
Next: Migrating ZFS Storage Pools

Determining the Health Status of ZFS Storage Pools

ZFS provides an integrated method of examining pool and device health.
The health of a pool is determined from the state of all its devices. This
state information is displayed by using the zpool status command.
In addition, potential pool and device failures are reported by fmd,
displayed on the system console, and logged in the /var/adm/messages file.

This section describes how to determine pool and device health. This
chapter does not document how to repair or recover from unhealthy pools. For
more information about troubleshooting and data recovery, see Chapter 11, Oracle Solaris ZFS Troubleshooting and Pool Recovery.

Each device can fall into one of the following states:

ONLINE

The device or virtual device is in normal working order. Although
some transient errors might still occur, the device is otherwise in working
order.

DEGRADED

The virtual device has experienced a failure but can still
function. This state is most common when a mirror or RAID-Z device has lost
one or more constituent devices. The fault tolerance of the pool might be
compromised, as a subsequent fault in another device might be unrecoverable.

FAULTED

The device or virtual device is completely inaccessible. This
status typically indicates total failure of the device, such that ZFS is incapable
of sending data to it or receiving data from it. If a top-level virtual device
is in this state, then the pool is completely inaccessible.

OFFLINE

The device has been explicitly taken offline by the administrator.

UNAVAIL

The device or virtual device cannot be opened. In some cases,
pools with UNAVAIL devices appear in DEGRADED mode.
If a top-level virtual device is UNAVAIL, then nothing
in the pool can be accessed.

REMOVED

The device was physically removed while the system was running.
Device removal detection is hardware-dependent and might not be supported
on all platforms.

The health of a pool is determined from the health of all its top-level
virtual devices. If all virtual devices are ONLINE, then
the pool is also ONLINE. If any one of the virtual devices
is DEGRADED or UNAVAIL, then the pool
is also DEGRADED. If a top-level virtual device is FAULTED or OFFLINE, then the pool is also FAULTED.
A pool in the FAULTED state is completely inaccessible.
No data can be recovered until the necessary devices are attached or repaired.
A pool in the DEGRADED state continues to run, but you
might not achieve the same level of data redundancy or data throughput than
if the pool were online.

Basic Storage Pool Health Status

You can quickly review pool health status by using the zpool
status command as follows:

# zpool status -x
all pools are healthy

Specific pools can be examined by specifying a pool name in the command
syntax. Any pool that is not in the ONLINE state should
be investigated for potential problems, as described in the next section.

Detailed Health Status

You can request a more detailed health summary status by using the -v option. For example:

# zpool status -v tank
pool: tank
state: DEGRADED
status: One or more devices could not be opened.  Sufficient replicas exist for
the pool to continue functioning in a degraded state.
action: Attach the missing device and online it using 'zpool online'.
see: http://www.sun.com/msg/ZFS-8000-2Q
scrub: scrub completed after 0h0m with 0 errors on Wed Jan 20 15:13:59 2010
config:

NAME        STATE     READ WRITE CKSUM
tank        DEGRADED     0     0     0
mirror-0  DEGRADED     0     0     0
c1t0d0  ONLINE       0     0     0
c1t1d0  UNAVAIL      0     0     0  cannot open

errors: No known data errors

This output displays a complete description of why the pool is in its
current state, including a readable description of the problem and a link
to a knowledge article for more information. Each knowledge article provides
up-to-date information about the best way to recover from your current problem.
Using the detailed configuration information, you can determine which device
is damaged and how to repair the pool.

In the preceding example, the faulted device should be replaced. After
the device is replaced, use the zpool online command to
bring the device online. For example:

# zpool online tank c1t0d0
Bringing device c1t0d0 online
# zpool status -x
all pools are healthy

If the autoreplace property is on, you might not
have to online the replaced device.

If a pool has an offline device, the command output identifies the problem
pool. For example:

# zpool status -x
pool: tank
state: DEGRADED
status: One or more devices has been taken offline by the administrator.
Sufficient replicas exist for the pool to continue functioning in a
degraded state.
action: Online the device using 'zpool online' or replace the device with
'zpool replace'.
scrub: resilver completed after 0h0m with 0 errors on Wed Jan 20 15:15:09 2010
config:

NAME        STATE     READ WRITE CKSUM
tank        DEGRADED     0     0     0
mirror-0  DEGRADED     0     0     0
c1t0d0  ONLINE       0     0     0
c1t1d0  OFFLINE      0     0     0  48K resilvered

errors: No known data errors

The READ and WRITE columns provide
a count of I/O errors that occurred on the device, while the CKSUM column
provides a count of uncorrectable checksum errors that occurred on the device.
Both error counts indicate a potential device failure, and some corrective
action is needed. If non-zero errors are reported for a top-level virtual
device, portions of your data might have become inaccessible.

The errors: field identifies any known data errors.

In the preceding example output, the offline device is not causing data
errors.

For more information about diagnosing and repairing faulted pools and
data, see Chapter 11, Oracle Solaris ZFS Troubleshooting and Pool Recovery.

Previous: Viewing I/O Statistics for ZFS Storage Pools
Next: Migrating ZFS Storage Pools

© 2010, Oracle Corporation and/or its affiliates