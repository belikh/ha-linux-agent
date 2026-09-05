---
title: Backlight support — The Linux Kernel  documentation
id: backlight-support-the-linux-kernel-documentation
tags:
- linux-agent-jupiteros-fleet-15537b
- kernel-docs
- backlight
- sysfs
- hardware-telemetry
created: '2026-09-02T06:54:26.028431Z'
updated: '2026-09-05T10:51:22.103608Z'
source: https://www.kernel.org/doc/html/latest/gpu/backlight.html
source_domain: www.kernel.org
fetched_at: '2026-09-02T06:54:26.026860Z'
fetch_provider: builtin
status: evergreen
type: note
tier: unknown
content_type: unknown
deprecated: false
summary: 'Kernel backlight documentation (current for 7.3.0-rc1). Backlight sysfs
  attributes: brightness (R/W — the REQUESTED level), actual_brightness (RO — the
  level actually used by HW), max_brightness (RO). Interface at /sys/class/backlight/<backlight>/brightness
  and /sys/class/backlight/<backlight>/bl_power (power mode: 0 full on, 4 full off).
  Drivers must implement get_brightness() if HW doesn''t support every brightness
  value, so actual_brightness is the readback truth. uevents are emitted on actual_brightness
  when backlight changes; hotkey/platform brightness changes require the driver to
  call backlight_force_update() to re-sync userspace (BACKLIGHT_UPDATE_HOTKEY vs BACKLIGHT_UPDATE_SYSFS
  reasons). backlight_type enum: RAW (hardware registers), PLATFORM (platform-specific
  interface), FIRMWARE (standard firmware interface) — userspace can make policy decisions
  based on type. For a fleet agent controlling kiosk brightness: write brightness
  for control, read actual_brightness for state truth (a write may be clamped or ignored
  by HW), max_brightness for the scale, and type for preferring firmware/native interfaces.
  NOTE: this page replaces the 404''d ABI/testing/sysfs-class-backlight URL assigned
  in the batch; full attribute list lives in the stable/sysfs-class-backlight ABI
  file referenced by this page.'
---

Backlight support — The Linux Kernel  documentation

The Linux Kernel

7.3.0-rc1

Quick search

Contents

Working with the development community

Development process

Submitting patches

Code of conduct

Maintainer handbook

All development-process docs

Internal API manuals

Core API

Driver APIs

Subsystems

Core subsystems

Human interfaces

Input Documentation

Human Interface Devices (HID)

Sound Subsystem Documentation

GPU Driver Developer’s Guide

Frame Buffer

LEDs

Networking interfaces

Storage interfaces

Other subsystems

Locking

Development tools and processes

Licensing rules

Writing documentation

Development tools

Testing guide

Hacking guide

Tracing

Fault injection

Livepatching

Rust

User-oriented documentation

Administration

Build system

Reporting issues

Userspace tools

Userspace API

Firmware-related documentation

Firmware

Firmware and Devicetree

Architecture-specific documentation

CPU architectures

Other documentation

Unsorted documentation

Translations

Translations

This Page

Show Source

Backlight support¶

The backlight core supports implementing backlight drivers.

A backlight driver registers a driver using
devm_backlight_device_register(). The properties of the backlight
driver such as type and max_brightness must be specified.
When the core detect changes in for example brightness or power state
the update_status() operation is called. The backlight driver shall
implement this operation and use it to adjust backlight.

Several sysfs attributes are provided by the backlight core:

- brightness         R/W, set the requested brightness level
- actual_brightness  RO, the brightness level used by the HW
- max_brightness     RO, the maximum  brightness level supported

See ABI file stable/sysfs-class-backlight for the full list.

The backlight can be adjusted using the sysfs interface, and
the backlight driver may also support adjusting backlight using
a hot-key or some other platform or firmware specific way.

The driver must implement the get_brightness() operation if
the HW do not support all the levels that can be specified in
brightness, thus providing user-space access to the actual level
via the actual_brightness attribute.

When the backlight changes this is reported to user-space using
an uevent connected to the actual_brightness attribute.
When brightness is set by platform specific means, for example
a hot-key to adjust backlight, the driver must notify the backlight
core that brightness has changed using backlight_force_update().

Display drives can control the backlight device’s status using
backlight_notify_blank() and backlight_notify_blank_all(). If this
results in a change in the backlight state the functions call the
update_status() operation.

enum backlight_update_reason¶

what method was used to update backlight

Constants

BACKLIGHT_UPDATE_HOTKEY
The backlight was updated using a hot-key.

BACKLIGHT_UPDATE_SYSFS
The backlight was updated using sysfs.

Description

A driver indicates the method (reason) used for updating the backlight
when calling backlight_force_update().

enum backlight_type¶

the type of backlight control

Constants

BACKLIGHT_RAW
The backlight is controlled using hardware registers.

BACKLIGHT_PLATFORM
The backlight is controlled using a platform-specific interface.

BACKLIGHT_FIRMWARE
The backlight is controlled using a standard firmware interface.

BACKLIGHT_TYPE_MAX
Number of entries.

Description

The type of interface used to control the backlight.

struct backlight_ops¶

backlight operations

Definition:

struct backlight_ops {
unsigned int options;
#define BL_CORE_SUSPENDRESUME   (1 << 0);
int (*update_status)(struct backlight_device *);
int (*get_brightness)(struct backlight_device *);
bool (*controls_device)(struct backlight_device *bd, struct device *display_dev);
};

Members

options
Configure how operations are called from the core.

The options parameter is used to adjust the behaviour of the core.
Set BL_CORE_SUSPENDRESUME to get the update_status() operation called
upon suspend and resume.

update_status
Operation called when properties have changed.

Notify the backlight driver some property has changed.
The update_status operation is protected by the update_lock.

The backlight driver is expected to use backlight_is_blank()
to check if the display is blanked and set brightness accordingly.
update_status() is called when any of the properties has changed.

RETURNS:

0 on success, negative error code if any failure occurred.

get_brightness
Return the current backlight brightness.

The driver may implement this as a readback from the HW.
This operation is optional and if not present then the current
brightness property value is used.

RETURNS:

A brightness value which is 0 or a positive number.
On failure a negative error code is returned.

controls_device
Check against the display device

Check if the backlight controls the given display device. This
operation is optional and if not implemented it is assumed that
the display is always the one controlled by the backlight.

RETURNS:

If display_dev is NULL or display_dev matches the device controlled by
the backlight, return true. Otherwise return false.

Description

The backlight operations are specified when the backlight device is registered.

struct backlight_properties¶

backlight properties

Definition:

struct backlight_properties {
int brightness;
int max_brightness;
int power;
#define BACKLIGHT_POWER_ON              (0);
#define BACKLIGHT_POWER_OFF             (4);
#define BACKLIGHT_POWER_REDUCED         (1);
enum backlight_type type;
unsigned int state;
#define BL_CORE_SUSPENDED       (1 << 0)        ;
#define BL_CORE_FBBLANK         (1 << 1)        ;
enum backlight_scale scale;
};

Members

brightness
The current brightness requested by the user.

The backlight core makes sure the range is (0 to max_brightness)
when the brightness is set via the sysfs attribute:
/sys/class/backlight/<backlight>/brightness.

This value can be set in the backlight_properties passed
to devm_backlight_device_register() to set a default brightness
value.

max_brightness
The maximum brightness value.

This value must be set in the backlight_properties passed to
devm_backlight_device_register() and shall not be modified by the
driver after registration.

power
The current power mode.

User space can configure the power mode using the sysfs
attribute: /sys/class/backlight/<backlight>/bl_power
When the power property is updated update_status() is called.

The possible values are: (0: full on, 4: full off), see
BACKLIGHT_POWER constants.

When the backlight device is enabled, power is set to
BACKLIGHT_POWER_ON. When the backlight device is disabled,
power is set to BACKLIGHT_POWER_OFF.

type
The type of backlight supported.

The backlight type allows userspace to make appropriate
policy decisions based on the backlight type.

This value must be set in the backlight_properties
passed to devm_backlight_device_register().

state
The state of the backlight core.

The state is a bitmask. BL_CORE_FBBLANK is set when the display
is expected to be blank. BL_CORE_SUSPENDED is set when the
driver is suspended.

backlight drivers are expected to use backlight_is_blank()
in their update_status() operation rather than reading the
state property.

The state is maintained by the core and drivers may not modify it.

scale
The type of the brightness scale.

Description

This structure defines all the properties of a backlight.

struct backlight_device¶

backlight device data

Definition:

struct backlight_device {
struct backlight_properties props;
struct mutex update_lock;
struct mutex ops_lock;
const struct backlight_ops *ops;
struct list_head entry;
struct device dev;
int use_count;
};

Members

props
Backlight properties

update_lock
The lock used when calling the update_status() operation.

update_lock is an internal backlight lock that serialise access
to the update_status() operation. The backlight core holds the update_lock
when calling the update_status() operation. The update_lock shall not
be used by backlight drivers.

ops_lock
The lock used around everything related to backlight_ops.

ops_lock is an internal backlight lock that protects the ops pointer
and is used around all accesses to ops and when the operations are
invoked. The ops_lock shall not be used by backlight drivers.

ops
Pointer to the backlight operations.

If ops is NULL, the driver that registered this device has been unloaded,
and if class_get_devdata() points to something in the body of that driver,
it is also invalid.

entry
List entry of all registered backlight devices

dev
Parent device.

use_count
The number of unblanked displays.

Description

This structure holds all data required by a backlight device.

int backlight_update_status(struct backlight_device *bd)¶

force an update of the backlight device status

Parameters

struct backlight_device *bd
the backlight device

int backlight_enable(struct backlight_device *bd)¶

Enable backlight

Parameters

struct backlight_device *bd
the backlight device to enable

int backlight_disable(struct backlight_device *bd)¶

Disable backlight

Parameters

struct backlight_device *bd
the backlight device to disable

bool backlight_is_blank(const struct backlight_device *bd)¶

Return true if display is expected to be blank

Parameters

const struct backlight_device *bd
the backlight device

Description

Display is expected to be blank if any of these is true:

1) if power in not UNBLANK
2) if state indicate BLANK or SUSPENDED

Returns true if display is expected to be blank, false otherwise.

int backlight_get_brightness(const struct backlight_device *bd)¶

Returns the current brightness value

Parameters

const struct backlight_device *bd
the backlight device

Description

Returns the current brightness value, taking in consideration the current
state. If backlight_is_blank() returns true then return 0 as brightness
otherwise return the current brightness property value.

Backlight drivers are expected to use this function in their update_status()
operation to get the brightness value.

void *bl_get_data(struct backlight_device *bl_dev)¶

access devdata

Parameters

struct backlight_device *bl_dev
pointer to backlight device

Description

When a backlight device is registered the driver has the possibility
to supply a void * devdata. bl_get_data() return a pointer to the
devdata.

pointer to devdata stored while registering the backlight device.

void backlight_force_update(struct backlight_device *bd, enum backlight_update_reason reason)¶

tell the backlight subsystem that hardware state has changed

Parameters

struct backlight_device *bd
the backlight device to update

enum backlight_update_reason reason
reason for update

Description

Updates the internal state of the backlight in response to a hardware event,
and generates an uevent to notify userspace. A backlight driver shall call
backlight_force_update() when the backlight is changed using, for example,
a hot-key. The updated brightness is read using get_brightness() and the
brightness value is reported using an uevent.

struct backlight_device *backlight_device_get_by_name(const char *name)¶

Get backlight device by name

Parameters

const char *name
Device name

Description

This function looks up a backlight device by its name. It obtains a reference
on the backlight device and it is the caller’s responsibility to drop the
reference by calling put_device().

Return

A pointer to the backlight device if found, otherwise NULL.

struct backlight_device *devm_backlight_device_register(struct device *dev, const char *name, struct device *parent, void *devdata, const struct backlight_ops *ops, const struct backlight_properties *props)¶

register a new backlight device

Parameters

struct device *dev
the device to register

const char *name
the name of the device

struct device *parent
a pointer to the parent device (often the same as dev)

void *devdata
an optional pointer to be stored for private driver use

const struct backlight_ops *ops
the backlight operations structure

const struct backlight_properties *props
the backlight properties

Description

Creates and registers new backlight device. When a backlight device
is registered the configuration must be specified in the props
parameter. See description of backlight_properties.

struct backlight on success, or an ERR_PTR on error

void devm_backlight_device_unregister(struct device *dev, struct backlight_device *bd)¶

unregister backlight device

Parameters

struct device *dev
the device to unregister

struct backlight_device *bd
the backlight device to unregister

Description

Deallocates a backlight allocated with devm_backlight_device_register().
Normally this function will not need to be called and the resource management
code will ensure that the resources are freed.

struct backlight_device *of_find_backlight_by_node(struct device_node *node)¶

find backlight device by device-tree node

Parameters

struct device_node *node
device-tree node of the backlight device

Description

Returns a pointer to the backlight device corresponding to the given DT
node or NULL if no such backlight device exists or if the device hasn’t
been probed yet.

This function obtains a reference on the backlight device and it is the
caller’s responsibility to drop the reference by calling put_device() on
the backlight device’s .dev field.

struct backlight_device *devm_of_find_backlight(struct device *dev)¶

find backlight for a device

Parameters

struct device *dev
the device

Description

This function looks for a property named ‘backlight’ on the DT node
connected to dev and looks up the backlight device. The lookup is
device managed so the reference to the backlight device is automatically
dropped on driver detach.

A pointer to the backlight device if found.
Error pointer -EPROBE_DEFER if the DT property is set, but no backlight
device is found. NULL if there’s no backlight property.

©The kernel development community.

|
Powered by Sphinx 5.3.0
& Alabaster 0.7.16

|
Page source